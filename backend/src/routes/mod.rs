use crate::handlers::{easy_execution, hard_interpreter};
use actix_web::web;

pub fn register_routes(config: &mut web::ServiceConfig) {
    config.route("/execute", web::post().to(easy_execution::execute));
    config.route(
        "/hard-interpreter",
        web::post().to(hard_interpreter::execute),
    );
}
