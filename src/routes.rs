use crate::endpoints;
use actix_web::{web, HttpResponse};

// this function could be located in a different module
pub fn route_result(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/result")
            .route(web::get().to(endpoints::get_result))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
            // .route(web::update().to(HttpResponse::MethodNotAllowed))
            .route(web::post().to(HttpResponse::MethodNotAllowed)),
    );
}

// this function could be located in a different module
pub fn route_cast(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::resource("/cast")
            .route(web::get().to(HttpResponse::MethodNotAllowed))
            .route(web::head().to(HttpResponse::MethodNotAllowed))
            // .route(web::update().to(HttpResponse::MethodNotAllowed))
            .route(web::post().to(endpoints::cast_vote)),
    );
}
