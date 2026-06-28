use actix_web::{App, HttpResponse, HttpServer, Responder, middleware::Logger, web};
use env_logger::Env;
use main::{AppState, config_routes, db::sea_db::Db};
async fn manual_hello() -> impl Responder {
    HttpResponse::Ok().body("Hey there!")
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    env_logger::init_from_env(Env::default().default_filter_or("info"));
    // 全局服务 也就是 请求业务服务
    let app_state = web::Data::new(AppState::new());
    // 数据库初始化
    Db::init().await.expect("数据库初始化失败，程序无法继续");

    // let db_conn = Db::get().expect("获取数据库连接失败");

    // Migrator::up(db_conn, None).await.expect("数据库迁移失败");
    println!("✅ 数据库迁移完成");

    HttpServer::new(move || {
        let logger = Logger::default();
        App::new()
            .app_data(app_state.clone())
            .wrap(logger)
            .route("/hey", web::get().to(manual_hello))
            .configure(config_routes)
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
