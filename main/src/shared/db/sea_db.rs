//! 数据库 全局连接

use anyhow::{Context, Result};
use sea_orm::{ConnectOptions, Database};
use std::{sync::OnceLock, time::Duration};

static DB_POOL: OnceLock<sea_orm::prelude::DatabaseConnection> = OnceLock::new();

pub struct Db;

impl Db {
    // / 初始化全局数据库连接（在 main 中调用一次）
    pub async fn init() -> Result<()> {
        // 修正：使用动态路径
        let current_dir = std::env::current_dir()?;
        let db_path = current_dir.join("database.db");
        let db_path_str = db_path.to_str().context("路径转换失败")?;
        let connection_string = format!("sqlite:{}", db_path_str);

        println!("Connecting to database at: {}", connection_string);

        let mut opt = ConnectOptions::new(connection_string);
        opt.max_connections(100)
            .min_connections(5)
            .connect_timeout(Duration::from_secs(8))
            .acquire_timeout(Duration::from_secs(8))
            .idle_timeout(Duration::from_secs(8))
            .max_lifetime(Duration::from_secs(8))
            .sqlx_logging(false);

        let db: sea_orm::prelude::DatabaseConnection = Database::connect(opt).await?;
        DB_POOL
            .set(db)
            .map_err(|_| anyhow::anyhow!("数据库连接已初始化"))?;
        Ok(())
    }

    /// 获取数据库连接（在任意地方使用）
    pub fn get() -> Result<&'static sea_orm::prelude::DatabaseConnection> {
        DB_POOL.get().context("数据库连接获取失败".to_string())
    }
}
