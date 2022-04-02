use actix_redis::RedisActor;
use actix_web::middleware::Logger;
use actix_web::{web, App, HttpServer};

mod config;
mod db;
mod endpoints;
mod model;
mod routes;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // init Database Connection
    // App Data für die Connection
    env_logger::init_from_env(env_logger::Env::new().default_filter_or("debug"));

    /* I have no idea why the move is needed */
    /* according to documentation it is needed to move the AppData into the Closure*/
    HttpServer::new(move || {
        let redis_addr = RedisActor::start(config::get_connection_string());
        log::info!("Connection String: {}", config::get_connection_string());
        App::new()
            .wrap(Logger::new(
                "Serving Request for: %a with User Agent: %{User-Agent}i",
            ))
            .app_data(web::Data::new(redis_addr))
            .service(
                web::scope("/vote")
                    .configure(routes::route_cast)
                    .configure(routes::route_result),
            )
        // .service(web::scope("/vote").configure(routes::route_result))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
