use actix_web::web;

mod controller;
mod services;
pub use services::*;
pub fn routes(cfg: &mut web::ServiceConfig) {
    // let service = UserService::new();
    cfg.service(
        web::scope("users")
            .service(controller::get_user)
    );
}
