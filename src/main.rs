mod variant;
use crate::variant::Variant;
use std::collections::HashMap;

type Json = HashMap<String, Variant>;

pub const ENDPOINT: &'static str = "https://na1.api.riotgames.com";

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
    let j = resp.json::<Json>()?;
    return Ok(j["accountId"].clone().into());
}

fn get_matches(id: &str, start_idx: i32, end_idx: i32) -> Result<Json, Box<dyn std::error::Error>> {
    // TODO put range into request
    let api_endpoint_base = String::from("/lol/match/v4/matchlists/by-account");
    let slug = api_endpoint_base + "/" + id +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::blocking::get(request)?;
    let j = resp.json::<Json>()?;
    return Ok(j);
}

fn get_game_info(game_id: &str) -> Result<Json, Box<dyn std::error::Error>> {
    let api_endpoint_base = String::from("/lol/match/v4/matches/");
    let slug = api_endpoint_base + game_id +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::blocking::get(request)?;
    let j = resp.json::<Json>()?;
    return Ok(j);

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // TODO - make internal data representation

    let enc_account_id = get_encrypted_account_id("Suq Mediq")?;
    println!("encrypted account id : {}" , enc_account_id);

    let mut more_matches = true;
    while more_matches {
        // get match history by encrypted account Id
            // Limit of 100 matches at a time. Grab 100 at a time until there are no more
        let mut start_idx = 0;
        let mut end_idx = 99;
        let matches = get_matches(&enc_account_id, start_idx, end_idx)?;
        println!("matches: {:?}", matches);
        let games: Vec<Json> = matches["matches"].clone().into();
        for game in games {
            let game_id: i64 = game["gameId"].clone().into();
            let game_info = get_game_info(&game_id.to_string())?;

            // TODO - zip all these together and iterate over them at once

            let participant_identities: Vec<Json> = game_info["participantIdentities"].clone().into();
            for participant_identity in participant_identities {
                let participant_id: i64 = participant_identity["participantId"].clone().into();
                let player: Json = participant_identity["player"].clone().into();
                let summoner_name: String = player["summonerName"].clone().into();
                let summoner_id: String = player["summonerId"].clone().into();
            }

            let stats: Vec<Json> = game_info["teams"].clone().into();
            for stat in stats {
                let win: String = stat["win"].clone().into();
            }

            let participants: Vec<Json> = game_info["participants"].clone().into();
            for participant in participants {
                let team_id: i64 = participant["teamId"].clone().into();
                let participant_id: i64 = participant["participantId"].clone().into();
            }
        }
        more_matches = false;
    }

    // Interested in:
    // Our teams role names -> get summoner name for each role
    // Result of the game (win/loss/none)
    // KDA of each person?

    println!("end of main\n");
    Ok(())
}
