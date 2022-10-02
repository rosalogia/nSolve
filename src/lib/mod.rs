pub mod util;
pub mod models;
pub mod schema;
mod config;
use actix_web::http::StatusCode;
pub use config::Config;
use actix_web::{error::ResponseError, HttpResponse, http::header::ContentType};
use std::fmt::Display;
use std::collections::BTreeMap;
use argon2::password_hash;

#[derive(Debug)]
pub enum Error {
    IoError(std::io::Error),
    DieselError(diesel::result::Error),
    SerdeError(serde_json::Error),
    JwtError(jwt::Error),
    PasswordHashError(password_hash::Error)
}

impl From<std::io::Error> for Error {
    fn from(err: std::io::Error) -> Self {
        Error::IoError(err)
    }
}

impl From<diesel::result::Error> for Error {
    fn from(err: diesel::result::Error) -> Self {
        Error::DieselError(err)
    }
}

impl From<serde_json::Error> for Error {
    fn from(err: serde_json::Error) -> Self {
        Error::SerdeError(err)
    }
}

impl From<jwt::Error> for Error {
    fn from(err: jwt::Error) -> Self {
        Error::JwtError(err)
    }
}

impl From<password_hash::Error> for Error {
    fn from(err: password_hash::Error) -> Self {
        Error::PasswordHashError(err)
    }
}

// impl Error {
//     pub fn from<T>(io_result: std::io::Result<T>) -> Result<T, Error> {
//         match io_result {
//             Ok(res) => Ok(res),
//             Err(err) => Err(Error::IoError(err))
//         }
//     }

//     pub fn from_diesel_query<T>(diesel_result: diesel::result::QueryResult<T>) -> Result<T, Error> {
//         match diesel_result {
//             Ok(res) => Ok(res),
//             Err(err) => Err(Error::DieselError(err))
//         }
//     }
    
//     pub fn from_serde<T>(serde_result: serde_json::Result<T>) -> Result<T, Error> {
//         match serde_result {
//             Ok(res) => Ok(res),
//             Err(err) => Err(Error::SerdeError(err))
//         }
//     }
    
//     pub fn from_jwt<T>(jwt_result: Result<T, jwt::Error>) -> Result<T, Error> {
//         match jwt_result {
//             Ok(res) => Ok(res),
//             Err(err) => Err(Error::JwtError(err))
//         }
//     }
    
//     pub fn from_pwh<T>(pwh_result: Result<T, password_hash::Error>) -> Result<T, Error> {
//         match pwh_result {
//             Ok(res) => Ok(res),
//             Err(err) => Err(Error::PasswordHashError(err))
//         }
//     }
// }

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        use Error::*;
        match &self {
            IoError(ioe) => ioe.fmt(f),
            DieselError(de) => de.fmt(f),
            SerdeError(se) => se.fmt(f),
            JwtError(je) => je.fmt(f),
            PasswordHashError(ae) => ae.fmt(f)
        }
    }
}

impl ResponseError for Error {
    fn error_response(&self) -> actix_web::HttpResponse<actix_web::body::BoxBody> {
        use Error::*;
        let err_str = match &self {
            IoError(_) => "Internal IO error",
            DieselError(_) =>  "Internal database error",
            SerdeError(_) => "Serialization error: was your input correctly formatted?",
            JwtError(_) => "Internal signing error",
            PasswordHashError(password_hash::Error::Password) => "Invalid password submitted",
            PasswordHashError(_) => "Internal password hashing error"
        };

        println!("{}", self);

        HttpResponse::build(self.status_code())
            .insert_header(ContentType::json())
            .json(BTreeMap::from([("error", err_str)]))
    }

    fn status_code(&self) -> actix_web::http::StatusCode {
        StatusCode::INTERNAL_SERVER_ERROR
    }
}