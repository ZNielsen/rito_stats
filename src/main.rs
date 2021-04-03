use std::collections::HashMap;
use std::vec::Vec;

use serde_json::Value;
use reqwest::Client;
use serde::Deserialize;

type Json = HashMap<String, Value>;

pub const ENDPOINT: &'static str = "https://na1.api.riotgames.com";
pub const BLUE_SIDE: i64 = 100;
pub const RED_SIDE: i64 = 200;

// Interested in:
// Our teams role names -> get summoner name for each role
// Result of the game (win/loss/none)
// KDA of each person?

// My Data
#[derive(Debug)]
enum GameResultData {
    Win,
    Loss,
    Other
}
#[derive(Debug)]
struct PlayerData {
    lane: String,
    summ_name: String,
    summ_id: String,
}
#[derive(Debug)]
struct GameData {
    result: GameResultData,
    team: Vec<PlayerData>,
    team_of_interest: i64,
}

// Rito data for serde_json
#[derive(Debug, Deserialize)]
struct GameInfo {
    gameId: i64,
    teams: Vec<Win>,
    participantIdentities: Vec<ParticipantId>,
    participants: Vec<Participant>,
}

#[derive(Debug, Deserialize)]
struct Player {
    summonerName: String,
    summonerId: String,
}

#[derive(Debug, Deserialize)]
struct Timeline {
    lane: String
}

#[derive(Debug, Deserialize)]
struct Participant {
    teamId: i64,
    participantId: i64,
    timeline: Timeline
}

#[derive(Debug, Deserialize)]
struct ParticipantId {
    participantId: i64,
    player: Player,
}

#[derive(Debug, Deserialize)]
struct Win {
    win: String
}

#[derive(Debug, Deserialize)]
struct Match {
    gameId: i64
}

#[derive(Debug, Deserialize)]
struct Matches {
    startIndex: i64,
    endIndex: i64,
    matches: Vec<Match>,
}

/// Function expects API key to be the only thing in the file
/// Only read once, then store the string statically
fn get_api_key() -> Result<String, std::io::Error> {
    let key = std::fs::read_to_string("api.key")?.replace("\n", "");
    return Ok(key);
}

async fn get_encrypted_account_id(summ_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let slug = String::from("/lol/summoner/v4/summoners/by-name/") + summ_name +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::get(request).await?;
    let j = resp.json::<Value>().await?;
    return Ok(j["accountId"].to_string());
}

async fn get_matches(client: &Client, id: &str, start_idx: i64, end_idx: i64) -> Result<Matches, Box<dyn std::error::Error>> {
    let api_endpoint_base = String::from("/lol/match/v4/watchlists/by-account");
    let slug = api_endpoint_base + "/" + id +
        "?endIndex=" + &end_idx.to_string() +
        "&beginIndex=" + &start_idx.to_string() +
        "&api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = client.get(&request).send().await?;
    let j = resp.json::<Matches>().await?;
    return Ok(j);
}

async fn get_game_info(client: &Client, game_id: &str) -> Result<GameInfo, Box<dyn std::error::Error>> {
    let api_endpoint_base = String::from("/lol/match/v4/matches/");
    let slug = api_endpoint_base + game_id +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = client.get(request).send().await?;
    let j = resp.json::<GameInfo>().await?;
    return Ok(j);

}

async fn collect_data(summoner: &str) -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Vec::<GameData>::new();
    let summoner = String::from("Suq Mediq");

    let enc_account_id = get_encrypted_account_id(&summoner).await?;
    println!("encrypted account id : {}" , enc_account_id);

    // Create a reqwest Client
    let client = reqwest::Client::new();

    let mut more_matches = true;
    // The API has a limit of 100 matches at a time. Grab 100 at a time until there are no more
    let mut start_idx: i64 = 0;
    let mut end_idx: i64 = 100;
    while more_matches {
        let matches = get_matches(&client, &enc_account_id, start_idx, end_idx).await?;
        println!("matches: {:?}", matches);

        // Set up the next indexes
        start_idx = end_idx+1;
        end_idx = start_idx + 100;

        let range_start = matches.startIndex;
        let range_end = matches.endIndex;
        more_matches = range_end-range_start == 100;

        // TODO - Split this up into multiple threads
        // https://stackoverflow.com/questions/51044467/how-can-i-perform-parallel-asynchronous-http-get-requests-with-reqwest
        for a_match in matches.matches {
            let mut game = GameData {
                result: GameResultData::Other,
                team: Vec::new(),
                team_of_interest: 0,
            };

            let game_id = a_match.gameId;
            let game_info = get_game_info(&client, &game_id.to_string()).await?;

            assert!(game_info.participantIdentities.len() == game_info.participants.len());
            let iter = game_info.participantIdentities.iter()
                .zip(game_info.participants.iter())
                .map(|(x, y)| (x, y));

            // Get all the participants for this game
            let mut blue_team: Vec<PlayerData> = Vec::new();
            let mut red_team: Vec<PlayerData> = Vec::new();
            for it in iter {
                let (participant_identity, participant) = it;

                assert!(participant.participantId == participant_identity.participantId);

                if participant_identity.player.summonerName == summoner {
                    game.team_of_interest = participant.teamId;
                }

                let p = PlayerData {
                    lane: participant.timeline.lane.clone(),
                    summ_name: participant_identity.player.summonerName.clone(),
                    summ_id: participant_identity.player.summonerId.clone(),

                };

                match participant.teamId {
                    BLUE_SIDE => blue_team.push(p),
                    RED_SIDE => red_team.push(p),
                    _ => panic!("Got a team id of {}", participant.teamId),
                }
            }

            // TODO - track win here

            match game.team_of_interest {
                BLUE_SIDE => game.team = blue_team,
                RED_SIDE => game.team = red_team,
                _ => panic!("team of interest is {}", game.team_of_interest),
            }
            data.push(game);
        }
    }

    println!("end of main\n");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    Ok(())
}
