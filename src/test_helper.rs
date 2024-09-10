use migration::{Migrator, MigratorTrait};
use sea_orm::{ConnectionTrait, Statement};
use sea_orm::{Database, DatabaseConnection};
use std::env;
use test_context::AsyncTestContext;

pub struct DbTestContext {
    pub db: DatabaseConnection,
    pub db_name: String,
}

impl AsyncTestContext for DbTestContext {
    async fn setup() -> Self {
        dotenv::from_filename(".env").expect("failed to read .env file");
        // データベース名を作成
        let db_name = format!("test_db_{}", generate_random_string(8));
        let base_url = env::var("DATABASE_ORIGIN_URL").expect("DATABASE_URL must be set");
        // SeaORMでベースデータベースに接続
        let db = Database::connect(&base_url)
            .await
            .expect("Failed to connect to database");
        // テスト用データベースを作成
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!(
        "CREATE DATABASE IF NOT EXISTS `{}` CHARACTER SET utf8mb4 COLLATE utf8mb4_0900_as_cs;",
        &db_name
        ),
        ))
        .await
        .expect("Failed to create test database");

        // テスト用データベースに接続
        let test_db_url = format!("{}/{}", base_url, &db_name);
        let test_db = Database::connect(&test_db_url)
            .await
            .expect("Failed to connect to the test database");

        Migrator::up(&test_db, None)
            .await
            .expect("Failed to migrate");
        DbTestContext {
            db: test_db,
            db_name,
        }
    }

    async fn teardown(self) {
        // ベースURLを取得
        let base_url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

        // SeaORMでベースデータベースに接続
        let db = Database::connect(&base_url)
            .await
            .expect("Failed to connect to the database");

        // テスト用データベースを削除
        db.execute(Statement::from_string(
            db.get_database_backend(),
            format!("DROP DATABASE IF EXISTS {};", self.db_name),
        ))
        .await
        .expect("Failed to drop test database");
    }
}

use rand::{distributions::Alphanumeric, Rng};

fn generate_random_string(length: usize) -> String {
    rand::thread_rng()
        .sample_iter(&Alphanumeric)
        .take(length)
        .map(char::from)
        .collect()
}
