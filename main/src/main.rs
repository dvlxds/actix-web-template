use actix_web::{App, HttpResponse, HttpServer, Responder, web};
// use env_logger::Env;
use main::{AppState, config_routes, db::sqlx_db};
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}


#[actix_web::main]
async fn main() -> std::io::Result<()> {
    // SeaORM debug 日志
    tracing_subscriber::fmt()
        // 允许通过环境变量 RUST_LOG 控制日志，没有配置默认打印 info 级别
        .with_env_filter(tracing_subscriber::EnvFilter::new(
            std::env::var("RUST_LOG").unwrap_or_else(|_| "info,sea_orm=debug".into()),
        ))
        .init();

    // env_logger::init_from_env(Env::default().default_filter_or("info"));
    // 全局服务
    let app_state = web::Data::new(AppState::new());
    // 数据库初始化
    sqlx_db::Db::init()
        .await
        .expect("数据库初始化失败，程序无法继续");
    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/hey", web::get().to(manual_hello))
            .configure(config_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
