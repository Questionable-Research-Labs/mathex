use super::error::Error;
use crate::{
    api::{auth::check_password_admin, error::ErrorKind},
    database,
};
use actix_web::{
    delete, get,
    http::header::ContentType,
    put,
    web::{self, Json},
    HttpRequest, HttpResponse, Responder,
};
use serde::{Deserialize, Serialize};
use serde_json::to_string as to_json;
use sqlx::PgPool;
use tracing::log::error;

#[derive(Serialize, Deserialize)]
pub struct AddCompetitionRequest {
    level: i32,
}

#[get("/competition")]
pub async fn get_competitions(pool: web::Data<PgPool>) -> impl Responder {
    match database::get_competitions(pool.as_ref()).await {
        Ok(entries) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&entries).unwrap()),
        Err(problem) => HttpResponse::InternalServerError()
            .content_type(ContentType::json())
            .body(
                to_json(&Error::message(
                    crate::api::error::ErrorKind::InternalServer,
                    format!("{problem}"),
                ))
                .unwrap(),
            ),
    }
}

#[put("/competition")]
pub async fn add_competition(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    request: Json<AddCompetitionRequest>,
) -> impl Responder {
    if let Some(err) = check_password_admin(req.headers()) {
        return err;
    }

    let comp = database::add_competition(pool.as_ref(), request.level).await;

    match comp {
        Ok(comp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&comp).unwrap()),
        Err(ex) => {
            error!("Error creating competition: {}", ex);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(to_json(&Error::kind(ErrorKind::InternalServer)).unwrap())
        }
    }
}

#[get("/competition/{id}")]
pub async fn get_competition(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let comp = database::get_competition(pool.as_ref(), id.into_inner()).await;

    match comp {
        Ok(comp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&comp).unwrap()),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(to_json(&Error::kind(ErrorKind::NotFound)).unwrap()),
        Err(ex) => {
            error!("Error getting competition: {}", ex);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(to_json(&Error::kind(ErrorKind::InternalServer)).unwrap())
        }
    }
}

#[delete("/competition/{id}")]
pub async fn remove_competition(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> impl Responder {
    if let Some(err) = check_password_admin(req.headers()) {
        return err;
    }

    let comp = database::remove_competition(pool.as_ref(), id.into_inner()).await;

    match comp {
        Ok(comp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&comp).unwrap()),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(to_json(&Error::kind(ErrorKind::NotFound)).unwrap()),
        Err(ex) => {
            error!("Error deleting competition: {}", ex);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(to_json(&Error::kind(ErrorKind::InternalServer)).unwrap())
        }
    }
}

#[get("/competition/{id}/entries")]
pub async fn get_competition_entries(
    pool: web::Data<PgPool>,
    id: web::Path<i32>,
) -> impl Responder {
    let comp = database::get_competition_entries(pool.as_ref(), id.into_inner()).await;

    match comp {
        Ok(comp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&comp).unwrap()),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(to_json(&Error::kind(ErrorKind::NotFound)).unwrap()),
        Err(ex) => {
            error!("Error deleting competition: {}", ex);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(to_json(&Error::kind(ErrorKind::InternalServer)).unwrap())
        }
    }
}
