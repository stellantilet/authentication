use crate::{db, db::get_db_con, DBPool, Result};
use serde::{Deserialize, Serialize};
use std::str::FromStr;
use warp::{http::StatusCode, reject, reply::json, Reply};

pub const TABLE: &str = "users";

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginRequest{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupRequest{
    pub email: String,
    pub password: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct LoginResponse {
    pub result: String,
}

#[derive(Deserialize, Serialize, Debug)]
pub struct SignupResponse {
    result: String,
}

pub async fn login_handler(
    body: LoginRequest,
    db_pool: DBPool,
) -> Result<impl Reply> {
    let res;
    let mut check: bool = false;
    let users = db::users::fetch(&db_pool).await.map_err(reject::custom)?;

    for value in &users{
        if *value.email == body.email{
            if *value.password == body.password{
                check = true;
                break;
            } else {
                continue;
            }
        } else {
            continue;
        }
    }

    if check == true {
        res = "success".to_string();
    } else {
        res = "failed".to_string();
    }

    Ok(json(&LoginResponse{ result: res }))
}

pub async fn signup_handler(body: SignupRequest, db_pool:DBPool ) -> Result<impl Reply>{
    db::users::create(&db_pool, body).await;
    Ok(json(&SignupResponse{ result: "success".to_string()}
    ))
}