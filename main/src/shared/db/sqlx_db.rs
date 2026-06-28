//! 数据库 全局连接

use anyhow::{Context, Result};
use sqlx::{Pool, Sqlite, sqlite::SqlitePoolOptions};
use std::{sync::OnceLock, time::Duration};

static DB_POOL: OnceLock<Pool<Sqlite>> = OnceLock::new();

pub struct Db;

impl Db {
    /// 初始化全局数据库连接（在 main 中调用一次）
    pub async fn init() -> Result<()> {
        if DB_POOL.get().is_some() {
            println!("Database already initialized");
            return Ok(());
        }
        let current_dir = std::env::current_dir()?;
        let db_path = current_dir.join("database.db");
        let db_path_str = db_path.to_str().context("路径转换失败")?;
        let connection_string = format!("sqlite:{}", db_path_str);

        println!("Connecting to database at: {}", connection_string);

        let db_pool = SqlitePoolOptions::new()
            .max_connections(5)
            .max_lifetime(Duration::from_secs(30 * 60)) // 连接最大存活时间 30分钟
            .idle_timeout(Duration::from_secs(10 * 60))  // 空闲连接超时 10分钟
            .connect(&connection_string)
            .await
            // .context("数据库连接失败")?;
            .with_context(||format!("数据库连接失败"))?;

        // 存入全局变量
        DB_POOL
            .set(db_pool)
            .map_err(|_| anyhow::anyhow!("数据库已初始化"))?;

        println!("Database initialized successfully");
        Ok(())
    }

    /// 获取数据库连接（在任意地方使用）
    pub fn get() -> Result<&'static Pool<Sqlite>> {
        DB_POOL.get().context("数据库连接获取失败".to_string())
    }
}
