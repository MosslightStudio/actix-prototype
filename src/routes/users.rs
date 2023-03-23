use actix_web::{get, web::{self, ServiceConfig}, Responder};
use serde::Serialize;


#[derive(Serialize)]
struct User {
    name: String,
    id: u32,
}

#[get("{id}")]
async fn get_user(path: web::Path<u32>) -> actix_web::Result<impl Responder> {
    let id = path.into_inner();
    let user = User {
        name: "Bob".to_string(),
        id,
    };
    Ok(web::Json(user))
}

pub fn user_config(cfg: &mut ServiceConfig) {
    cfg.service(web::scope("/users").service(get_user));
}

