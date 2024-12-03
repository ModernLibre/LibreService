use crate::controller::book::{
    book_config, get_book_details, upload_book_info
};
// use crate::controller::user_controller::{
//     add_user, delete_user, get_user, get_user_list, user_count,
// };
use actix_web::{middleware, web};
use super::casdoor::validator;

pub fn init_routes(cfg: &mut web::ServiceConfig) {

    let middleware = actix_web_httpauth::middleware::HttpAuthentication::bearer(validator);

    cfg.service(
        web::scope("/")
            .wrap(middleware)
            .configure(v1)
    );
}

pub fn v1(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("/api/libre/v1")
            .configure(book_config)
    );

        // .service(
        //     web::scope("/users")
        //         .service(user_count)
        //         .service(get_user)
        //         .service(get_user_list)
        //         .service(delete_user)
        //         .service(add_user),
        // )
}
