use super::{get_db_con, Result};
use crate::{error::Error::*, DBPool, handler::SignupRequest};
use serde::{Deserialize, Serialize};
use mobc_postgres::tokio_postgres::Row;

#[derive(Deserialize, Serialize, Debug)]
pub struct User {
    pub id: i32,
    pub email: String,
    pub password: String,
}

pub const TABLE: &str = "users";
const SELECT_FIELDS: &str = "id, email, password";

pub async fn fetch(db_pool: &DBPool) -> Result<Vec<User>> {
    let con = get_db_con(db_pool).await?;
    let query = format!("SELECT {} FROM {}", SELECT_FIELDS, TABLE);
    let rows = con.query(query.as_str(), &[]).await.map_err(DBQueryError)?;

    Ok(rows.iter().map(|r| row_to_user(&r)).collect())
}

pub async fn create(db_pool: &DBPool, body: SignupRequest) -> Result<User> {
    let con = get_db_con(db_pool).await?;
    let query = format!("INSERT INTO {} (email, password) VALUES ($1, $2)", TABLE);
    let row = con
        .query_one(query.as_str(), &[&body.email, &body.password])
        .await
        .map_err(DBQueryError)?;
    Ok(row_to_user(&row))
}

fn row_to_user(row: &Row) -> User {
    let id: i32 = row.get(0);
    let email: String = row.get(1);
    let password: String = row.get(2);
    User { id, email, password }
}