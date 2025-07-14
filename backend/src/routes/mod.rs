use crate::handlers::execute;
use actix_web::web;

pub fn register_routes(config: &mut web::ServiceConfig) {
    config.route("/execute", web::post().to(execute::execute));
}
