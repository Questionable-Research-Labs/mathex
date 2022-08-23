use serde::Serialize;

#[derive(Serialize)]
pub enum ErrorKind {
    InternalServer,
    MissingPassword,
    InvalidPassword,
    NotFound,
    #[allow(dead_code)]
    Todo,
}

#[derive(Serialize)]
pub struct Error {
    #[serde(rename = "errorKind")]
    kind: ErrorKind,
    #[serde(skip_serializing_if = "Option::is_none")]
    message: Option<String>,
}

impl Error {
    pub fn kind(kind: ErrorKind) -> Self {
        Error {
            kind,
            message: None,
        }
    }

    pub fn message(kind: ErrorKind, message: String) -> Self {
        Error {
            kind,
            message: Some(message),
        }
    }
}
