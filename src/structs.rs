use std::vec::Vec;
use serde::{Deserialize, Serialize};

// // My Data
// #[derive(Debug, Deserialize, Serialize, PartialEq)]
// pub enum GameResultData {
//     Win,
//     Other
// }
// #[derive(Debug, Deserialize, Serialize)]
// pub struct PlayerData {
//     pub lane: String,
//     pub summ_name: String,
//     pub summ_id: String,
// }
// #[derive(Debug, Deserialize, Serialize)]
// pub struct GameData {
//     pub result: GameResultData,
//     pub teams: HashMap<i64, Vec<PlayerData>>,
//     pub team_of_interest: i64,
//     pub game_id: i64,
//     pub game_duration: i64,
//     pub game_mode: String,
//     pub game_type: String,
// }

// Riot data for serde_json
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Player {
    pub summoner_name: String,
    pub summoner_id: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Timeline {
    pub lane: String
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Participant {
    pub team_id: i64,
    pub participant_id: i64,
    pub timeline: Timeline
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct ParticipantId {
    pub participant_id: i64,
    pub player: Player,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct TeamStats {
    pub win: String,
    pub team_id: i64,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct GameInfo {
    pub game_id: i64,
    pub teams: Vec<TeamStats>,
    pub participant_identities: Vec<ParticipantId>,
    pub participants: Vec<Participant>,
    pub game_creation: i64,
    pub game_duration: i64,
    pub season_id: i64,
    pub game_version: String,
    pub map_id: i64,
    pub game_mode: String,
    pub game_type: String,
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Match {
    pub game_id: i64
}

#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Matches {
    pub start_index: i64,
    pub end_index: i64,
    pub matches: Vec<Match>,
}
