use chrono::NaiveDate;
use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::chapter;

use super::epub::{epub_parse_chapters, Epub};

#[derive(Debug, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = chapter)]
pub struct Chapter {
    pub id: i32,
    pub title: String,
    pub index: i32,
    pub content_index: i32,
    pub level: i32,
    pub parent_id: i32,
    pub book_id: i32,
    pub created_time: NaiveDate,
    pub updated_time: NaiveDate
}

pub struct Chapters {
    pub chapters: Vec<Chapter>,
}

impl Chapters {
    pub async fn init(epub: &mut Epub) -> Self {
        let result = epub_parse_chapters(epub);
        Self {
            chapters: result
        }
    }
}
