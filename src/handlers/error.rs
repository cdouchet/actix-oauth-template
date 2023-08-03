use actix_web::ResponseError;
use reqwest::StatusCode;
use serde::Serialize;
use std::fmt::Display;

#[derive(Debug)]
pub struct Error {
    name: String,
    description: Option<String>,
    status_code: StatusCode,
}

#[derive(Serialize)]
struct SerializableError {
    name: String,
    description: Option<String>,
}

impl Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let serializable = SerializableError {
            name: self.name.clone(),
            description: self.description.clone(),
        };
        let serialized = serde_json::to_string(&serializable).unwrap();
        write!(f, "{}", serialized)
    }
}

impl ResponseError for Error {
    fn status_code(&self) -> StatusCode {
        self.status_code
    }
}

impl Error {
    pub fn from_diesel_error(err: diesel::result::Error) -> Self {
        match err {
            diesel::result::Error::NotFound => Self::not_found(Some("Item in Database")),
            diesel::result::Error::DatabaseError(kind, info) => {
                Self::database_error(info.message(), info.details())
            }
            _ => Self::internal_server_error(),
        }
    }

    fn database_error<'a>(err: &'a str, desc: Option<&'a str>) -> Self {
        Self {
            name: String::from(err),
            description: match desc {
                Some(e) => Some(String::from(e)),
                None => None,
            },
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn internal_server_error() -> Self {
        Self {
            name: String::from("Internal Server Error"),
            description: None,
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }

    pub fn not_found<'a>(item: Option<&'a str>) -> Self {
        Self {
            name: String::from("Not Found"),
            description: match item {
                Some(e) => Some(format!("{} was not found", e)),
                None => None,
            },
            status_code: StatusCode::NOT_FOUND,
        }
    }

    pub fn expired_token() -> Self {
        Self {
            name: String::from("Token has expired"),
            description: Some(String::from("Try to reconnect")),
            status_code: StatusCode::UNAUTHORIZED,
        }
    }

    pub fn google_error(name: String, description: String) -> Self {
        Self {
            name,
            description: Some(description),
            status_code: StatusCode::INTERNAL_SERVER_ERROR,
        }
    }
}
