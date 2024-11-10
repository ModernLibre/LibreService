use std::io::Cursor;
use diesel::{prelude::{Insertable, Queryable}, Selectable};
use epub::doc::EpubDoc;
use serde::{Deserialize, Serialize};

use crate::schema::recources;

#[derive(Debug, Clone, Serialize, Deserialize, Queryable, Insertable, Selectable)]
#[diesel(table_name = recources)]
pub struct Resource {
    pub index: i32,
    pub content: String
}

pub struct Resources {
    resources: Vec<Resource>
}
impl Resources {
    pub fn init(mut data: EpubDoc<Cursor<Vec<u8>>>) -> Self {
        let mut index = 1;
        let mut resources = Vec::new();
        let spine = data.spine.clone();
        let epub_rescources = data.resources.clone();
        for content_name in spine {
            let (path, _) = epub_rescources.get(&content_name).unwrap();
            let content = data.get_resource_str_by_path(path).unwrap().clone();
            let recourse = Resource {
                index: index,
                content,
            };
            resources.push(recourse);
            index += 1;
        }
        Self {
            resources
        }
    } 
    pub fn get_resources(&self) -> Vec<Resource> {
        self.resources.clone()
    }
}