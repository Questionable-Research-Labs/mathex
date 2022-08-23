use serde::{Deserialize, Serialize};
use utoipa::Component;

#[derive(Serialize, Deserialize, Component)]
pub struct Competition {
    pub id: i32,
    pub level: i32,
}

#[derive(Serialize, Deserialize, Component)]
pub struct Team {
    pub id: i32,
    pub team_name: String,
    pub entered_competition: i32,
}
