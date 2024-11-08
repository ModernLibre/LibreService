use std::{collections::{BTreeMap, HashMap}, io::{Cursor, Read}, path::PathBuf};

use actix_multipart::form::MultipartForm;
use epub::doc::EpubDoc;

use crate::controller::epub::epub_controller::UploadForm;

pub struct Epub {
    // data
    data: EpubDoc<Cursor<Vec<u8>>>,

    // menu info
    catalog: BTreeMap<usize, (PathBuf, String)>,

    // menu to resource mapping
    resources_mapping: BTreeMap<usize, usize>,
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

    pub fn get_catalog_name(&self) -> Vec<String> {
        self.catalog
        .iter()
        .map(|(_, (_, name))| name.clone())
        .collect::<Vec<String>>()
    }

    pub fn get_title(&self) -> String {
        self.data.mdata("title").unwrap()
    }

    pub fn get_author(&self) -> String {
        self.data.mdata("author").unwrap()
    }

    pub fn get_catalog(&self) -> BTreeMap<usize, (PathBuf, String)> {
        self.catalog.clone()
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
