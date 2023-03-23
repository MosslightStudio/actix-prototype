mod routes;
use actix_web::{web, App, HttpServer};
use routes::users::user_config;

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    HttpServer::new(|| App::new().service(web::scope("/api").configure(user_config)))
        .bind(("127.0.0.1", 3100))?
        .run()
        .await
}
