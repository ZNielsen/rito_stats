use std::vec::Vec;
use serde::{Deserialize, Serialize};

// My Data
#[derive(Debug, Deserialize, Serialize)]
pub enum GameResultData {
    Win,
    Other
}
#[derive(Debug, Deserialize, Serialize)]
pub struct PlayerData {
    pub lane: String,
    pub summ_name: String,
    pub summ_id: String,
}
#[derive(Debug, Deserialize, Serialize)]
pub struct GameData {
    pub result: GameResultData,
    pub team: Vec<PlayerData>,
    pub team_of_interest: i64,
    pub game_id: i64,
}

// Rito data for serde_json
#[derive(Debug, Deserialize)]
pub struct Account {
    pub accountId: String,
    pub id: String,
}

#[derive(Debug, Deserialize)]
pub struct Player {
    pub summonerName: String,
    pub summonerId: String,
}

#[derive(Debug, Deserialize)]
pub struct Timeline {
    pub lane: String
}

#[derive(Debug, Deserialize)]
pub struct Participant {
    pub teamId: i64,
    pub participantId: i64,
    pub timeline: Timeline
}

#[derive(Debug, Deserialize)]
pub struct ParticipantId {
    pub participantId: i64,
    pub player: Player,
}

#[derive(Debug, Deserialize)]
pub struct TeamStats {
    pub win: String,
    pub teamId: i64,
}

#[derive(Debug, Deserialize)]
pub struct GameInfo {
    pub gameId: i64,
    pub teams: Vec<TeamStats>,
    pub participantIdentities: Vec<ParticipantId>,
    pub participants: Vec<Participant>,
}

#[derive(Debug, Deserialize)]
pub struct Match {
    pub gameId: i64
}

#[derive(Debug, Deserialize)]
pub struct Matches {
    pub startIndex: i64,
    pub endIndex: i64,
    pub matches: Vec<Match>,
}
