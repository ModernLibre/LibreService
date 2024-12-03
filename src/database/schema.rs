use diesel::{table, QueryDsl};
use chrono::NaiveDate;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = book)]
pub struct Book {
    pub id: i32,
    pub file_url: String,
    pub cover_url: String,
    pub title: String,
    pub author: String,
    pub rating: f64,
    pub status: i32,
    pub description: String,
    pub added_date: NaiveDate,
}

table! {
    book (id) {
        id -> Int4,
        file_url -> Varchar,
        cover_url -> Varchar,
        title -> Varchar,
        author -> Varchar,
        description -> Text,
        status -> Int4,
        rating -> Float8,
        added_date -> Date,
    }
}
