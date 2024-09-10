use futures::executor::block_on;
use sea_orm::*;
extern crate dotenv;
use sea_orm_practice_20240911::entities::{prelude::*, *};
use std::env;

async fn run() -> Result<(), DbErr> {
    dotenv::from_filename(".env").expect("Failed to read .env file");
    let database_origin_url = env::var("DATABASE_ORIGIN_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&database_origin_url).await?;
    let db_name = env::var("DB_NAME").expect("DATABASE_NAME must be set");

    db.execute(Statement::from_string(
        db.get_database_backend(),
        format!("CREATE DATABASE IF NOT EXISTS `{}`;", db_name),
    ))
    .await?;

    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");

    let db = Database::connect(&url).await?;

    Ok(())
}

fn main() {
    if let Err(err) = block_on(run()) {
        panic!("{}", err);
    }
}

// pallarel
async fn create_post(
    db: DatabaseConnection,
) -> Result<sea_orm::InsertResult<post::ActiveModel>, DbErr> {
    let post = post::ActiveModel {
        title: ActiveValue::Set("Post Title".to_owned()),
        text: ActiveValue::Set("Post Text".to_owned()),
        ..Default::default()
    };
    let post = Post::insert(post).exec(&db).await?;

    Ok(post)
}

use sea_orm_practice_20240911::entities::post::Model;
async fn update_post(post_id: i32, db: DatabaseConnection) -> Result<Model, DbErr> {
    let post = post::Entity::find_by_id(post_id).one(&db).await?.unwrap();
    let mut post: post::ActiveModel = post.into();
    post.title = ActiveValue::Set("Updated Post Title".to_owned());
    post.text = ActiveValue::Set("Updated Post Text".to_owned());
    let post = Post::update(post).exec(&db).await?;

    Ok(post)
}

#[cfg(test)]
mod tests_pallalel {
    use super::*;
    use sea_orm_practice_20240911::test_helper::*;
    use test_context::test_context;

    #[test_context(DbTestContext)]
    #[test]
    fn test_create_post(ctx: &mut DbTestContext) {
        let result = block_on(create_post(ctx.db.clone()));
        assert!(result.is_ok());
    }

    #[test_context(DbTestContext)]
    #[test]
    fn test_update_post(ctx: &mut DbTestContext) {
        let result = block_on(create_post(ctx.db.clone()));
        assert!(result.is_ok());
        let post = result.unwrap();
        let expect = block_on(update_post(post.last_insert_id, ctx.db.clone()));
        assert!(expect.is_ok());
    }
}

// serial
async fn create_postx() -> Result<sea_orm::InsertResult<post::ActiveModel>, DbErr> {
    dotenv::from_filename(".env").expect("Failed to read .env file");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&url).await?;

    let post = post::ActiveModel {
        title: ActiveValue::Set("Post Title".to_owned()),
        text: ActiveValue::Set("Post Text".to_owned()),
        ..Default::default()
    };
    let post = Post::insert(post).exec(&db).await?;

    Ok(post)
}

async fn update_postx(post_id: i32) -> Result<Model, DbErr> {
    dotenv::from_filename(".env").expect("Failed to read .env file");
    let url = env::var("DATABASE_URL").expect("DATABASE_URL must be set");
    let db = Database::connect(&url).await?;

    let post = post::Entity::find_by_id(post_id).one(&db).await?.unwrap();
    let mut post: post::ActiveModel = post.into();
    post.title = ActiveValue::Set("Updated Post Title".to_owned());
    post.text = ActiveValue::Set("Updated Post Text".to_owned());
    let post = Post::update(post).exec(&db).await?;

    Ok(post)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_create_post() {
        let result = block_on(create_postx());
        println!("create_post {:?}", result);
        assert!(result.is_ok());
    }

    #[test]
    fn test_update_post() {
        let result = block_on(create_postx());
        println!("update_post {:?}", result);
        assert!(result.is_ok());
        let post = result.unwrap();
        let expect = block_on(update_postx(post.last_insert_id));
        assert!(expect.is_ok());
    }
}
