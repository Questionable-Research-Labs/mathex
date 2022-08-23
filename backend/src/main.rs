use actix_web::{web, App, HttpServer};
use api::{competition::*, docs::ApiDocs, team::*};
use dotenv::dotenv;
use sqlx::postgres::PgPool;
use std::env;
use tracing_actix_web::TracingLogger;
use tracing_subscriber::{fmt, prelude::*, EnvFilter};
use utoipa::OpenApi;
use utoipa_swagger_ui::SwaggerUi;

mod api;

mod database;

#[tokio::main]
async fn main() -> anyhow::Result<()> {
    dotenv().ok();

    tracing_subscriber::registry()
        .with(fmt::layer())
        .with(EnvFilter::from_default_env())
        .init();

    let pool = PgPool::connect(&env::var("DATABASE_URL").expect("Please set a valid database URL"))
        .await
        .expect("Could not connect to database");

    HttpServer::new(move || {
        App::new()
            .wrap(TracingLogger::default())
            .app_data(web::Data::new(pool.clone()))
            .service(get_competitions)
            .service(add_competition)
            .service(remove_competition)
            .service(get_competition)
            .service(get_competition_entries)
            .service(get_teams)
            .service(add_team)
            .service(remove_team)
            .service(get_team)
            .service(
                SwaggerUi::new("/swagger-ui/{_:.*}")
                    .url("/api-doc/openapi.json", ApiDocs::openapi()),
            )
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await?;

    Ok(())
}
