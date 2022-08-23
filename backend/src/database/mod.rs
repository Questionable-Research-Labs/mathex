use self::schema::{Competition, Team};
use crate::PgPool;
pub mod schema;

pub async fn get_competitions(pool: &PgPool) -> anyhow::Result<Vec<Competition>> {
    Ok(
        sqlx::query_as!(Competition, r#"SELECT * FROM competitions"#)
            .fetch_all(pool)
            .await?,
    )
}

pub async fn add_competition(pool: &PgPool, level: i32) -> anyhow::Result<Competition> {
    Ok(sqlx::query_as!(
        Competition,
        r#"INSERT INTO competitions ("level") VALUES ($1) RETURNING *"#,
        level
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_competition(pool: &PgPool, id: i32) -> sqlx::Result<Competition> {
    sqlx::query_as!(
        Competition,
        r#"SELECT * FROM "competitions" WHERE id = $1"#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn remove_competition(pool: &PgPool, id: i32) -> sqlx::Result<Competition> {
    sqlx::query_as!(
        Competition,
        r#"DELETE FROM "competitions" WHERE id = $1 RETURNING *"#,
        id
    )
    .fetch_one(pool)
    .await
}

pub async fn get_competition_entries(pool: &PgPool, id: i32) -> sqlx::Result<Vec<Team>> {
    sqlx::query_as!(
        Team,
        r#"SELECT * FROM "teams" WHERE "entered_competition" = $1"#,
        id
    )
    .fetch_all(pool)
    .await
}

pub async fn get_teams(pool: &PgPool) -> anyhow::Result<Vec<Team>> {
    Ok(sqlx::query_as!(Team, r#"SELECT * FROM teams"#)
        .fetch_all(pool)
        .await?)
}

pub async fn add_team(
    pool: &PgPool,
    entered_competition: i32,
    team_name: String,
) -> anyhow::Result<Team> {
    Ok(sqlx::query_as!(
        Team,
        r#"INSERT INTO teams ("entered_competition", "team_name") VALUES ($1, $2) RETURNING *"#,
        entered_competition,
        team_name
    )
    .fetch_one(pool)
    .await?)
}

pub async fn get_team(pool: &PgPool, id: i32) -> sqlx::Result<Team> {
    sqlx::query_as!(Team, r#"SELECT * FROM "teams" WHERE id = $1"#, id)
        .fetch_one(pool)
        .await
}

pub async fn remove_team(pool: &PgPool, id: i32) -> sqlx::Result<Team> {
    sqlx::query_as!(Team, r#"DELETE FROM "teams" WHERE id = $1 RETURNING *"#, id)
        .fetch_one(pool)
        .await
}
