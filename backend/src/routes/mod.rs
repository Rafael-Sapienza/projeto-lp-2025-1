use crate::handlers::{execute, hard_interpreter};
use actix_web::web;

pub fn register_routes(config: &mut web::ServiceConfig) {
    config.route("/execute", web::post().to(execute::execute));
    config.route("/hard-interpreter", web::post().to(hard_interpreter::execute));
}
