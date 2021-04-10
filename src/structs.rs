use std::path::Path;
use std::vec::Vec;
use serde::{Deserialize, Serialize};

/// Overlapping information with Player, obtained via a different API call
#[derive(Debug, Deserialize, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct Account {
    pub account_id: String,
    pub id: String,
    pub name: String,
}

/// Overlapping information with Account, obtained via a different API call
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

pub type GamesData = Vec<GameInfo>;
struct PrintItem {
    title: &'static str,
    field: String,
}

impl GameInfo {
    fn order(&self, player_idx: usize) -> Vec<PrintItem> {
        vec![
            PrintItem{ title: "Game ID",     field: self.game_id.to_string() },
            PrintItem{ title: "Player Name", field: self.participant_identities[player_idx].player.summoner_name.clone() },
            PrintItem{ title: "Player ID",   field: self.participant_identities[player_idx].participant_id.to_string() },
            PrintItem{ title: "Player Team", field: self.participants[player_idx].team_id.to_string() },
            PrintItem{ title: "Team Result", field: self.teams[(self.participants[player_idx].team_id / 100) as usize].win.clone() },
        ]
    }
}

pub trait CSVable {
    fn write_to_csv(&self, path: &Path, separator: &str) -> Result<(), Box<dyn std::error::Error>>;
}
impl CSVable for GameInfo {
    fn write_to_csv(&self, path: &Path, separator: &str) -> Result<(), Box<dyn std::error::Error>> {
        let num_players = self.participants.len();
        for player_idx in 0..=num_players {
            let order = &self.order(player_idx);
            let mut s = String::new();
            for field in order {
                s += &field.field;
                s += separator;
            }
            // Trim the last separator
            s.truncate(s.len() - separator.len());
            std::fs::write(path, &s)?;
        }

        Ok(())
    }
}
impl CSVable for GamesData {
    fn write_to_csv(&self, path: &Path, separator: &str) -> Result<(), Box<dyn std::error::Error>> {
        assert!(self.len() > 0);

        // Write out first line
        let order = &self[0].order(0);
        let mut s = String::new();
        for field in order {
            s += field.title;
            s += separator;
        }
        // Trim the last separator
        s.truncate(s.len() - separator.len());
        std::fs::write(path, &s)?;

        // Write out each row
        for game in self {
            game.write_to_csv(path, separator)?;
        }

        Ok(())
    }
}
