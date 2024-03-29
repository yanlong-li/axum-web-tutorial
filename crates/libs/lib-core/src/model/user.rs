use serde::{Deserialize, Serialize};
use sqlx::Error;

use crate::model::store::DbPool;

#[derive(Deserialize, Serialize, sqlx::FromRow, Clone)]
pub struct User {
    pub id: u64,
    pub username: String,
}

pub async fn find_by_username(db: &DbPool, username: &str) -> Result<User, Error> {
    sqlx::query_as("select id,username from `user` where username = ?")
        .bind(username)
        .fetch_one(db)
        .await
}

pub async fn find_one(db: &DbPool, user_id: &u64) -> Result<User, Error> {
    sqlx::query_as("select id,username from `user` where id = ?")
        .bind(user_id)
        .fetch_one(db)
        .await
}

pub async fn find_all(db: &DbPool) -> Result<Vec<User>, Error> {
    sqlx::query_as("select id,username from `user`")
        .fetch_all(db)
        .await
}

pub async fn create(db: &DbPool, username: &str) -> Result<User, Error> {
    let id = sqlx::query("insert into `user`(username) values(?);")
        .bind(username).execute(db).await?.last_insert_id();

    Ok(User {
        id,
        username: String::from(username),
    })
}