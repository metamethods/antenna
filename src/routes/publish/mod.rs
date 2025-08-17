use actix_web::web;

mod message;

pub fn config(config: &mut web::ServiceConfig) {
    config.service(web::scope("/publish").service(message::publish_message));
}
