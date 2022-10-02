use diesel::prelude::*;
use super::schema::{problems, user_accounts};
use serde::{Serialize, Deserialize};

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct Problem {
    pub id: i32,
    pub title: String,
    pub content_path: String,
    pub author_id: i32
}

#[derive(Insertable)]
#[diesel(table_name = problems)]
pub struct NewProblem {
    pub title: String,
    pub content_path: String
}

#[derive(Serialize, Deserialize, Debug, Queryable)]
pub struct UserAccount {
    pub id: i32,
    pub display_name: String,
    pub email: String,
    pub password_hash: String
}

#[derive(Insertable)]
#[diesel(table_name = user_accounts)]
pub struct NewUser {
    pub display_name: String,
    pub email: String,
    pub password_hash: String
}