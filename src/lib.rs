pub mod casdoor;
pub mod controller;
pub mod database;
pub mod error;
pub mod routes;
pub mod util;
pub use database::schema;

#[macro_use]
extern crate lazy_static;
