use actix_web::web;

mod app;
mod features;
mod shared;
mod utils;
// 导出
pub use app::AppState;
pub use app::AppStateData;
pub use features::*;
pub use shared::*;
// 路由配置
pub fn config_routes(cfg: &mut web::ServiceConfig) {
    // println!("路由配置");
    cfg.service(
        web::scope("/api/v1")
            .configure(users::routes)
            .configure(upload::routes),
    );
}
