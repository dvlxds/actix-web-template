mod services;
mod controller;

use actix_web::web;

pub use self::services::*;

// 路由配置
pub fn routes(cfg: &mut web::ServiceConfig) {
    cfg.service(
        web::scope("upload")
            .service(controller::upload_file)
    );
}
