use chrono::NaiveDate;
use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::chapter;

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = chapter)]
pub struct Chapter {
    pub id: i32,
    pub title: String,
    pub index: i32,
    pub content: String,
    pub level: i32,
    pub parent_id: i32,
    pub book_id: i32,
    pub created_time: NaiveDate,
    pub updated_time: NaiveDate
}
