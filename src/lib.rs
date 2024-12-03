pub mod casdoor;
pub mod controller;
pub mod error;
pub mod models;
pub mod routes;
pub mod util;
pub mod database;
pub use database::schema;

#[macro_use]
extern crate lazy_static;
