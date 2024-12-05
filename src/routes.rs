// use super::casdoor::validator;
use actix_web::web;

use crate::controller::{v1, test};

pub fn init_routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/libre")
            .service(
                web::scope("/v1")
                    .configure(v1::book::service_config)
            )
            .service(
                web::scope("/test")
                    .configure(test::service_config)
            )
    );
}

