use actix_web::{ web, App, HttpResponse, HttpServer };
use actix_redis::{Command, RedisActor};
use redis_async::{resp::RespValue, resp_array};
use std::sync::Mutex;

mod routes;
mod endpoints;
mod db;
mod model;
// this function could be located in a different module

const REDIS_CON_STRING: &str = "192.168.1.188:6379";


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init Database Connection
    // App Data für die Connection
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    /* I have no idea why the move is needed */
    /* according to documentation it is needed to move the AppData into the Closure*/
    HttpServer::new( move || {
        let redis_addr = RedisActor::start(REDIS_CON_STRING);
        App::new()
            .app_data(web::Data::new(redis_addr))
            .service(web::scope("/vote").configure(routes::route_cast).configure(routes::route_result))
            // .service(web::scope("/vote").configure(routes::route_result))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}

