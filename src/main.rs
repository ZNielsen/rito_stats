use std::collections::HashMap;
use serde::Deserialize;

#[derive(Deserialize, Debug, Clone)]
#[serde(untagged)]
enum Variant {
    Int(i64),
    Str(String),
    VecVar(std::vec::Vec<HashMap<String, Variant>>),
}
impl Into<String> for Variant {
    fn into(self) -> String {
        match self {
            Variant::Str(s) => s,
            _ => panic!("Not a string: {:?}", self),
        }
    }
}
impl Into<i64> for Variant {
    fn into(self) -> i64 {
        match self {
            Variant::Int(i) => i,
            _ => panic!("Not an int: {:?}", self),
        }
    }
}
impl Into<Vec<HashMap<String, Variant>>> for Variant {
    fn into(self) -> Vec<HashMap<String, Variant>> {
        match self {
            Variant::VecVar(v) => v,
            _ => panic!("Not a Vec: {:?}", self),
        }
    }
}

// {"id":"B1K11tBYT6OVo88bHODxC55XiWZEBvKcsi0koJe5SsGyh4c","accountId":"JZO25Xuf5Y0QHfEmXvsDGIc_Or4zB_wowN9ZhB3-nw1ljQ","puuid":"oq8loqZb6CYoxISQ6PK3FUyZuxnMqYwVw4VC1exqlbRTku0sjJTyNF1NH2AafbmyWXYi5Y7N4KEpVw","name":"Suq Mediq","profileIconId":1639,"revisionDate":1616733734000,"summonerLevel":71}

pub const ENDPOINT: &'static str = "https://na1.api.riotgames.com";

/// Function expects API key to be the only thing in the file
fn get_api_key() -> Result<String, std::io::Error> {
    let key = std::fs::read_to_string("api.key")?.replace("\n", "");
    println!("key: {}", key);
    return Ok(key);
}

fn get_encrypted_account_id(summ_name: &str) -> Result<String, Box<dyn std::error::Error>> {
    let slug = String::from("/lol/summoner/v4/summoners/by-name/") + summ_name +
        "?api_key=" + &get_api_key().unwrap();
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::blocking::get(request)?;
    let j = resp.json::<HashMap<String, Variant>>()?;
    return Ok(j["accountId"].clone().into());
}

fn get_matches(id: &str, start_idx: i32, end_idx: i32) -> Result<HashMap<String, Variant>, Box<dyn std::error::Error>> {
    let api_endpoint_base = "/lol/match/v4/matchlists/by-account".to_owned();
    let slug = api_endpoint_base + "/" + id +
        "?api_key=" + &get_api_key().unwrap();
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::blocking::get(request)?;
    let j = resp.json::<HashMap<String, Variant>>()?;
    return Ok(j);
}

fn get_game_info(id: i64) -> Result<HashMap<String, Variant>, Box<dyn std::error::Error>> {

}

fn main() -> Result<(), Box<dyn std::error::Error>> {

    // Get encrypted account Id
    println!("Getting account id");
    let enc_account_id = get_encrypted_account_id("Suq Mediq")?;
    println!("encrypted account id : {}" , enc_account_id);

    let mut more_matches = true;
    while more_matches {
        // get match history by encrypted account Id
            // Limit of 100 matches at a time. Grab 100 at a time until there are no more
        let mut start_idx = 0;
        let mut end_idx = 99;
        let matches = get_matches(&enc_account_id, start_idx, end_idx)?;
        println!("matches: {}", matches);
        let games: Vec<HashMap<String, Variant>> = matches["matches"].into();
        for game in games {
            let game_id: i64 = game["gameId"].into();
            // get /lol/match/v4/matches/{matchId}
            let game_info = get_game_info(game_id);
            // participantIdentities.player.summonerName (and ID)
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
