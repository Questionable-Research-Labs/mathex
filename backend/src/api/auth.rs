use std::env;

use actix_web::{
    http::header::{ContentType, HeaderMap},
    HttpResponse,
};

use super::error::{Error, ErrorKind};

pub fn check_password_admin(headers: &HeaderMap) -> Option<HttpResponse> {
    let password = env::var("ADMIN_PASSWORD").ok()?;

    if let Some(usr_password) = headers.get("Authorization") {
        if let Ok(usr_password) = usr_password.to_str() {
            if usr_password == &password {
                None
            } else {
                Some(
                    HttpResponse::Unauthorized()
                        .content_type(ContentType::json())
                        .body(
                            serde_json::to_string(&Error::kind(ErrorKind::InvalidPassword))
                                .unwrap(),
                        ),
                )
            }
        } else {
            Some(
                HttpResponse::BadRequest()
                    .content_type(ContentType::json())
                    .body(
                        serde_json::to_string(&Error::message(
                            ErrorKind::MissingPassword,
                            "Password can't be a string?".into(),
                        ))
                        .unwrap(),
                    ),
            )
        }
    } else {
        Some(
            HttpResponse::Unauthorized()
                .content_type(ContentType::json())
                .body(serde_json::to_string(&Error::kind(ErrorKind::MissingPassword)).unwrap()),
        )
    }
}
