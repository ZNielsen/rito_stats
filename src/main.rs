use std::collections::HashMap;
use std::vec::Vec;

// type Json = HashMap<String, Variant>;
type Json = HashMap<String, serde_json::Value>;

pub const ENDPOINT: &'static str = "https://na1.api.riotgames.com";
pub const BLUE_SIDE: i64 = 100;
pub const RED_SIDE: i64 = 200;

// Interested in:
// Our teams role names -> get summoner name for each role
// Result of the game (win/loss/none)
// KDA of each person?

// My Data
enum GameResult {
    Win,
    Loss,
    Other
}
struct Player {
    lane: String,
    summ_name: String,
    summ_id: String,
}
struct Game {
    result: GameResult,
    team: Vec<Player>,
    team_of_interest: i64,
}

// Rito data for serde_json
// struct JsonGame {
//     gameId: i64
// }

//             let stats: Vec<Json> = game_info["teams"];
//             let participant_identities: Vec<Json> = game_info["participantIdentities"];
//             let participants: Vec<Json> = game_info["participants"];
//                 let participant_id_id: i64 = participant_identity["participantId"];
//                 let player: Json = participant_identity["player"];
//                 let summoner_name: String = player["summonerName"];
//                 let summoner_id: String = player["summonerId"];

//                 let team_id: i64 = participant["teamId"];
//                 let participant_id: i64 = participant["participantId"];
//                 let timeline: Json = participant["timeline"];
//                 let lane: String = timeline["lane"];

/// Function expects API key to be the only thing in the file
fn get_api_key() -> Result<String, std::io::Error> {
    let key = std::fs::read_to_string("api.key")?.replace("\n", "");
    println!("key: {}", key);
    return Ok(key);
}

fn get_encrypted_account_id(summ_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let slug = String::from("/lol/summoner/v4/summoners/by-name/") + summ_name +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::blocking::get(request)?;
    let j = resp.json::<serde_json::Value>()?;
    return Ok(j["accountId"].to_string());
}

fn get_matches(id: &str, start_idx: i32, end_idx: i32) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let api_endpoint_base = String::from("/lol/match/v4/matchlists/by-account");
    let slug = api_endpoint_base + "/" + id +
        "?endIndex=" + &end_idx.to_string() +
        "&beginIndex=" + &start_idx.to_string() +
        "&api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::blocking::get(&request)?;
    let j = resp.json::<serde_json::Value>()?;
    return Ok(j);
}

fn get_game_info(game_id: &str) -> Result<serde_json::Value, Box<dyn std::error::Error>> {
    let api_endpoint_base = String::from("/lol/match/v4/matches/");
    let slug = api_endpoint_base + game_id +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::blocking::get(request)?;
    let j = resp.json::<serde_json::Value>()?;
    return Ok(j);

}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let mut data = Vec::<Game>::new();
    let summoner = String::from("Suq Mediq");

    let enc_account_id = get_encrypted_account_id(&summoner)?;
    println!("encrypted account id : {}" , enc_account_id);

    let mut more_matches = true;
    // Limit of 100 matches at a time. Grab 100 at a time until there are no more
    let mut start_idx = 0;
    let mut end_idx = 100;
    while more_matches {
        let matches = get_matches(&enc_account_id, start_idx, end_idx)?;
        println!("matches: {:?}", matches);

        // Set up the next indexes
        start_idx = end_idx+1;
        end_idx = start_idx + 100;

        let range_start: i64 = matches["startIndex"].as_i64()?;
        let range_end: i64 = matches["endIndex"].as_i64()?;
        more_matches = range_end-range_start == 100;

        let games: Vec<serde_json::Value> = matches["matches"];
        for a_game in games {
            let mut game = Game {
                result: GameResult::Other,
                team: Vec::new(),
                team_of_interest: 0,
            };

            let game_id: i64 = a_game["gameId"];
            let game_info = get_game_info(&game_id.to_string())?;

            let stats: Vec<Json> = game_info["teams"];
            let participant_identities: Vec<Json> = game_info["participantIdentities"];
            let participants: Vec<Json> = game_info["participants"];
            assert!(participant_identities.len() == participants.len());
            let iter = participant_identities.iter()
                .zip(participants.iter())
                .map(|(x, y)| (x, y));


            // Get all the participants for this game
            let mut blue_team: Vec<Player> = Vec::new();
            let mut red_team: Vec<Player> = Vec::new();
            for it in iter {
                let (participant_identity, participant) = it;

                let participant_id_id: i64 = participant_identity["participantId"];
                let player: Json = participant_identity["player"];
                let summoner_name: String = player["summonerName"];
                let summoner_id: String = player["summonerId"];

                let team_id: i64 = participant["teamId"];
                let participant_id: i64 = participant["participantId"];
                let timeline: Json = participant["timeline"];
                let lane: String = timeline["lane"];

                assert!(participant_id_id == participant_id);

                if summoner_name == summoner {
                    game.team_of_interest = team_id;
                }

                let p = Player {
                    lane: lane,
                    summ_name: summoner_name,
                    summ_id: summoner_id,

                };

                match team_id {
                    BLUE_SIDE => blue_team.push(p),
                    RED_SIDE => red_team.push(p),
                    _ => panic!("Got a team id of {}", team_id),
                }
            }

            for stat in stats {
                let win: String = stat["win"];
            }

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
