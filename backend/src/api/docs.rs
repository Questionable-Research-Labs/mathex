use crate::{
    api::{competition, error::*, team},
    database::schema::*,
};
use utoipa::OpenApi;

#[derive(OpenApi)]
#[openapi(
    handlers(
        competition::get_competitions,
        competition::add_competition,
        // competition::remove_competition,
        // competition::get_competition,
        // competition::get_competition_entries,
        // team::get_teams,
        // team::add_team,
        // team::remove_team,
        // team::get_team,
    ),
    components(Competition, Team, Error, ErrorKind),
    tags(
        (name = "competition", description = "Competition management endpoints."),
        (name = "team", description = "Team management endpoints.")
    )
)]
pub struct ApiDocs;
