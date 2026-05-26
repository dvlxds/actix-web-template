use actix_web::web;

mod controller;
mod services;

// 导出
pub use services::*;
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("users")
            .service(controller::get_user)
    );
}
