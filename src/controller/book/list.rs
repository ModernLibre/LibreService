use crate::error::ServiceError::{self, *};
use crate::schema::Book;
use crate::util::run_blocking;
use actix_web::{web, HttpResponse, Responder};
use diesel::prelude::*;
use diesel::r2d2::{self, ConnectionManager, PooledConnection};
use diesel::PgConnection;
use serde::Deserialize;

type DbPool = r2d2::Pool<ConnectionManager<PgConnection>>;

#[derive(Deserialize)]
pub struct BooksListQuery {
    pub limit: Option<u32>,
    pub by: Option<String>,
}

struct ListBuilder {
    database_connection: PooledConnection<ConnectionManager<PgConnection>>,
    limit: i64,
    query: String,
}

impl ListBuilder {
    fn new(
        database_connection: PooledConnection<ConnectionManager<PgConnection>>,
        param: BooksListQuery,
    ) -> Self {
        Self {
            database_connection,
            limit: param.limit.unwrap_or(10) as i64,
            query: param.by.unwrap_or("id".to_string()),
        }
    }

    async fn make_query(self) -> Result<Vec<Book>, ServiceError> {
        use crate::schema::book::dsl::*;
        match self.query.as_str() {
            // 沟槽的rust，closure没有泛型
            "id" => {
                run_blocking(move || unified_query(self.database_connection, id.desc(), self.limit))
                    .await?
            }
            "recent" => {
                run_blocking(move || {
                    unified_query(self.database_connection, added_date.desc(), self.limit)
                })
                .await?
            }
            "top-rated" => {
                run_blocking(move || {
                    unified_query(self.database_connection, rating.desc(), self.limit)
                })
                .await?
            }
            _ => Err(BadRequest("Invalid query parameter".to_string())),
        }
    }
}

pub async fn list(
    db_pool: web::Data<DbPool>,
    query: web::Query<BooksListQuery>,
) -> actix_web::Result<impl Responder> {
    let conn = crate::database::get_conn(db_pool).await?;

    let list = ListBuilder::new(conn, query.into_inner());

    let books = list.make_query().await?;

    Ok(HttpResponse::Ok().json(books))
}

pub fn unified_query<QueryType>(
    mut db_conn: PooledConnection<ConnectionManager<PgConnection>>,
    query: QueryType,
    limit: i64,
) -> Result<Vec<Book>, ServiceError>
where
    QueryType: diesel::Expression
        + diesel::AppearsOnTable<crate::schema::book::table>
        + diesel::query_builder::QueryId
        + diesel::query_builder::QueryFragment<diesel::pg::Pg>,
{
    use crate::schema::book::dsl::*;
    book.select(Book::as_select())
        .order(query)
        .limit(limit)
        .load::<Book>(&mut db_conn)
        .map_err(ServiceError::from)
}
