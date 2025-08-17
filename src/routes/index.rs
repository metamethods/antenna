use actix_web::{HttpResponse, Responder, get, web};

#[get("/")]
async fn index() -> impl Responder {
    HttpResponse::Ok().body("Hello, World!")
}

pub fn config(config: &mut web::ServiceConfig) {
    config.service(index);
}
