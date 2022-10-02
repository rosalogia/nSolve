use crate::lib::{models::*, schema, util::establish_connection, Config, Error};
use actix_web::{post, web, Responder, Result, Scope};
use argon2::{
    password_hash::{rand_core::OsRng, PasswordHash, PasswordHasher, PasswordVerifier, SaltString},
    Argon2,
};
use diesel::prelude::*;
use hmac::{Hmac, Mac};
use jwt::{SignWithKey};
use serde::{Deserialize, Serialize};
use sha2::Sha256;
use std::collections::BTreeMap;

#[derive(Serialize, Deserialize)]
struct InnerUser {
    pub display_name: String,
    pub email: String,
    pub password: String,
}

#[derive(Serialize, Deserialize)]
struct LoginRequest {
    pub email: String,
    pub password: String,
}

fn generate_jwt(user_id: i32) -> Result<String, Error> {
    let Config { secret, .. } = Config::load_unsafe();
    let key: Hmac<Sha256> = Hmac::new_from_slice(&secret.as_bytes()).unwrap();
    let mut claims = BTreeMap::new();
    claims.insert("sub", user_id.to_string());
    let signed_token = claims.sign_with_key(&key)?;
    Ok(signed_token)
}

#[post("/register")]
async fn register_user(body: String) -> Result<impl Responder, Error> {
    let db_conn = &mut establish_connection(&Config::load_unsafe());
    let InnerUser {
        display_name,
        email,
        password,
    } = serde_json::from_str::<InnerUser>(&body)?;

    let salt = SaltString::generate(&mut OsRng);
    let argon2 = Argon2::default();
    let password_hash = argon2
        .hash_password(password.as_bytes(), &salt)?
        .to_string();
    {
        use crate::lib::schema::user_accounts;
        let user = diesel::insert_into(user_accounts::table)
            .values(&NewUser {
                display_name,
                email,
                password_hash,
            })
            .get_result::<UserAccount>(db_conn)?;
        let token = generate_jwt(user.id)?;
        Ok(web::Json(BTreeMap::from([("token", token)])))
    }
}

#[post("/login")]
async fn login_user(body: String) -> Result<impl Responder, Error> {
    let login_request = serde_json::from_str::<LoginRequest>(&body)?;
    let db_conn = &mut establish_connection(&Config::load_unsafe());
    use schema::user_accounts::dsl::*;

    let user = user_accounts
        .filter(email.eq(login_request.email))
        .first::<UserAccount>(db_conn)?;

    let parsed_hash = PasswordHash::new(&user.password_hash)?;
    Argon2::default().verify_password(&login_request.password.as_bytes(), &parsed_hash)?;

    let token = generate_jwt(user.id)?;

    Ok(web::Json(BTreeMap::from([("token", token)])))
}

pub fn service_group() -> Scope {
    web::scope("/user").service(register_user)
}
