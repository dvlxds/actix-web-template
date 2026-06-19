//! 数据库 全局连接

use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database};
use std::{sync::OnceLock, time::Duration};

static DB_POOL: OnceLock<sea_orm::prelude::DatabaseConnection> = OnceLock::new();

pub struct Db;

impl Db {
    // / 初始化全局数据库连接（在 main 中调用一次）
    pub async fn init() -> Result<()> {
        let mut opt = ConnectOptions::new("sqlite://path/to/db.sqlite?mode=rwc");
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false) // disable SQLx logging
            // .sqlx_logging_level(log::LevelFilter::Info)
            .set_schema_search_path("my_schema"); // set default Postgres schema

        let db: sea_orm::prelude::DatabaseConnection = Database::connect(opt).await?;
        if let Err(err) = DB_POOL.set(db) {
            println!("连接失败{:?}", err)
        };
        Ok(())
    }

    /// 获取数据库连接（在任意地方使用）
    pub fn get() -> Result<&'static sea_orm::prelude::DatabaseConnection> {
        DB_POOL.get().context("数据库连接获取失败".to_string())
    }
}
