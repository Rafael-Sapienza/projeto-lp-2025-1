use actix_web::web;
use crate::handlers::execute;

pub fn register_routes(config: &mut web::ServiceConfig) {
    config.route("/execute", web::post().to(execute::execute));
}
