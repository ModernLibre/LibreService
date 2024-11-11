use diesel::{prelude::{Insertable, Queryable}, Selectable};
use serde::{Deserialize, Serialize};

use crate::schema::recources;

use super::epub::{epub_parse_resources, Epub};

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = recources)]
pub struct Resource {
    pub index: i32,
    pub content: String
}

pub struct Resources {
    pub resources: Vec<Resource>
}
impl Resources {
    pub async fn init(epub: &mut Epub) -> Self {
        let result = epub_parse_resources(epub);
        Self {
            resources: result
        }
    } 
}