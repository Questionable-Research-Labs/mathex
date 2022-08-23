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
pub struct AddTeamRequest {
    team_name: String,
    entered_competition: i32,
}

#[get("/team")]
pub async fn get_teams(pool: web::Data<PgPool>) -> impl Responder {
    match database::get_teams(pool.as_ref()).await {
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

#[put("/team")]
pub async fn add_team(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    request: Json<AddTeamRequest>,
) -> impl Responder {
    if let Some(err) = check_password_admin(req.headers()) {
        return err;
    }

    let comp = database::add_team(
        pool.as_ref(),
        request.entered_competition,
        request.team_name.clone(),
    )
    .await;

    match comp {
        Ok(comp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&comp).unwrap()),
        Err(ex) => {
            error!("Error creating team: {}", ex);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(to_json(&Error::kind(ErrorKind::InternalServer)).unwrap())
        }
    }
}

#[get("/team/{id}")]
pub async fn get_team(pool: web::Data<PgPool>, id: web::Path<i32>) -> impl Responder {
    let comp = database::get_team(pool.as_ref(), id.into_inner()).await;

    match comp {
        Ok(comp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&comp).unwrap()),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(to_json(&Error::kind(ErrorKind::NotFound)).unwrap()),
        Err(ex) => {
            error!("Error getting team: {}", ex);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(to_json(&Error::kind(ErrorKind::InternalServer)).unwrap())
        }
    }
}

#[delete("/team/{id}")]
pub async fn remove_team(
    pool: web::Data<PgPool>,
    req: HttpRequest,
    id: web::Path<i32>,
) -> impl Responder {
    if let Some(err) = check_password_admin(req.headers()) {
        return err;
    }

    let comp = database::remove_team(pool.as_ref(), id.into_inner()).await;

    match comp {
        Ok(comp) => HttpResponse::Ok()
            .content_type(ContentType::json())
            .body(to_json(&comp).unwrap()),
        Err(sqlx::Error::RowNotFound) => HttpResponse::NotFound()
            .content_type(ContentType::json())
            .body(to_json(&Error::kind(ErrorKind::NotFound)).unwrap()),
        Err(ex) => {
            error!("Error deleting team: {}", ex);
            HttpResponse::InternalServerError()
                .content_type(ContentType::json())
                .body(to_json(&Error::kind(ErrorKind::InternalServer)).unwrap())
        }
    }
}
