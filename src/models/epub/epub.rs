use std::{collections::{BTreeMap, HashMap}, io::{Cursor, Read}, path::PathBuf};

use actix_multipart::form::MultipartForm;
use epub::doc::EpubDoc;

use crate::controller::epub::epub_controller::UploadForm;

use super::chapters::Chapter;

pub struct Epub {
    // data
    pub data: EpubDoc<Cursor<Vec<u8>>>,

    // menu info
    pub catalog: BTreeMap<usize, (PathBuf, String)>,

    // menu to resource mapping
    pub resources_mapping: BTreeMap<usize, usize>,
}

impl Epub {
    pub fn new(mut data_stream: MultipartForm<UploadForm>) -> Self {
        let mut epub_buffer = Vec::new();
    
        // 读取文件内容到 epub_buffer
        data_stream.file.file.read_to_end(&mut epub_buffer)
            .map_err(|err| Box::new(err)).unwrap();
        
        // 创建 Cursor 并从中读取 EpubDoc
        let cursor = std::io::Cursor::new(epub_buffer);
        let epub_doc = EpubDoc::from_reader(cursor).unwrap();
        let epub_catalog = Self::init_catalog(&epub_doc);
        let epub_resource_mapping = Self::init_resource_mapping(&epub_doc, &epub_catalog);

        Self {
            data: epub_doc,
            catalog: epub_catalog,
            resources_mapping: epub_resource_mapping,
        }
    }

    /// 初始化目录结构
    /// 
    /// key: 章节索引
    /// value: (PathBuf: 章节文件路径; String: 章节名)
    fn init_catalog(data: &EpubDoc<Cursor<Vec<u8>>>) -> BTreeMap<usize, (PathBuf, String)> {
        let toc = &data.toc;
        let mut key: usize = 0;
        let mut catalog = BTreeMap::new();

        for item in toc {
            catalog.insert(key, (item.content.clone(), item.label.clone()));
            key += 1;
        }
        
        return catalog
    }

    /// 初始化目录向资源的映射
    /// 
    /// key: 章节索引
    /// value: 资源索引
    fn init_resource_mapping(data: &EpubDoc<Cursor<Vec<u8>>>, catalog: &BTreeMap<usize, (PathBuf, String)>) -> BTreeMap<usize, usize>{
        let mut index: usize = 0;
        let mut spine = BTreeMap::new();

        // path -> index -> catalog_item_content
        for content_name in &data.spine {
            let (path, _) = &data.resources.get(content_name).unwrap();
            spine.insert(path.clone(), index);
            index += 1;
        }

        catalog
        .iter()
        .map(|(index, (path, _))| {
            let resource_index = spine.get(path).unwrap();
            (index.clone(), resource_index.clone())
        })
        .collect::<BTreeMap<usize, usize>>()

    }

    pub fn get_title(&self) -> String {
        self.data.mdata("title").unwrap()
    }

    pub fn get_author(&self) -> String {
        self.data.mdata("author").unwrap()
    }
    
    /// 获取Epub的css
    ///
    /// 返回一个HashMap<String, Vec<u8>>
    /// key为文件名，value为文件内容
    pub fn get_css(&mut self) -> HashMap<String, String> {
        let mut css_list = HashMap::new();
        for (name, (path, mime)) in self.data.resources.clone() {
            if mime == "text/css" {
                let css = String::from_utf8(self.data.get_resource_by_path(path).unwrap()).unwrap();
                css_list.insert(name + ".css", css);
            }
        }
        css_list
    }
}

pub fn get_catalog_name(epub: &mut Epub) -> Vec<String> {
    epub.catalog
    .iter()
    .map(|(_, (_, name))| name.clone())
    .collect::<Vec<String>>()
}

pub fn get_catalog_contents(epub: &mut Epub) -> Vec<Vec<u8>> {
    let len = epub.resources_mapping.len();
    let mut result = Vec::new();
    for index in 0..len {
        let rescourse_index = epub.resources_mapping.get(&index).unwrap();
        let cur_catalog_content = epub.data.get_resource_by_path(epub.data.spine[*rescourse_index].clone()).unwrap();
        result.push(cur_catalog_content);
    }
    result
}

// epub 文件解析
pub async fn epub_parse(epub: &mut  Epub) -> Vec<Chapter> {
    let len = epub.catalog.len();
    let mut result = Vec::new();
    let chapter_names = get_catalog_name(epub);
    let chapter_contents = get_catalog_contents(epub);
    for index in 0..len {
        let content = String::from_utf8(chapter_contents[index].clone()).unwrap_or_else(|_| String::from("Invalid UTF-8"));
        let chapter = Chapter {
            id: uuid::Uuid::new_v4().as_u128() as i32,
            title: chapter_names[index].clone(),
            index: index.clone() as i32,
            content,
            level: 0,
            parent_id: 0,
            book_id: 0,
            created_time: chrono::Local::now().naive_local().date(),
            updated_time: chrono::Local::now().naive_local().date()
        };
        result.push(chapter);  
    }
    result
}

pub async fn save_cover() {
    todo!()
}

pub async fn save_book_info() {
    todo!()
}

pub async fn save_recourse() {
    todo!()
}