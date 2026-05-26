use actix_web::web;

mod app;
mod features;
mod shared;
// 导出
pub use app::AppState;
pub use app::AppStateData;
pub use shared::*;
pub use features::*;
// 路由配置
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    // println!("路由配置");
    cfg.service(web::scope("/api/v1").configure(features::users::routes));
}
