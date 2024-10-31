use serde::Serialize;

#[derive(Serialize)]
pub struct Book {
    pub id: u32,
    pub title: String,
    pub author: String,
    pub rating: f32,
}