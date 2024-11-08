use chrono::NaiveDate;
use diesel::{Insertable, Queryable, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::book;
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