
// {"id":"B1K11tBYT6OVo88bHODxC55XiWZEBvKcsi0koJe5SsGyh4c","accountId":"JZO25Xuf5Y0QHfEmXvsDGIc_Or4zB_wowN9ZhB3-nw1ljQ","puuid":"oq8loqZb6CYoxISQ6PK3FUyZuxnMqYwVw4VC1exqlbRTku0sjJTyNF1NH2AafbmyWXYi5Y7N4KEpVw","name":"Suq Mediq","profileIconId":1639,"revisionDate":1616733734000,"summonerLevel":71}

/// Function expects API key to be the only thing in the file
fn get_api_key() -> Result<String, std::io::Error> {
    let key = std::fs::read_to_string("../api.key")?;
    println!("key: {}", key);
    return Ok(key);
}

fn get_encrypted_account_id(summ_name: &str) -> String {
    let region = String::from("na1");
    let endpoint = String::from("https://") + &region +
        ".api.riotgames.com/lol/summoner/v4/summoners/by-name/" +
        summ_name + "?api_key=" + &get_api_key().unwrap();

    // Make a request
    // parse output
    // return key

    // TODO
    return "B1K11tBYT6OVo88bHODxC55XiWZEBvKcsi0koJe5SsGyh4c".to_owned();
}

fn get_matches(id: &str, start_idx: i32, end_idx: i32) -> String {
    let api_endpoint_base = "/lol/match/v4/matchlists/by-account".to_owned();

    return "{}".to_owned();
}

fn main() {

    // Get encrypted account Id
    let enc_account_id = get_encrypted_account_id("Suq Mediq");

    while there_are_more_matches {
        // get match history by encrypted account Id
            // Limit of 100 matches at a time. Grab 100 at a time until there are no more
        let mut start_idx = 0;
        let mut end_idx = 99;
        let matches = get_matches(&enc_account_id, start_idx, end_idx);
        // for match in matches.gameId
        // get /lol/match/v4/matches/{matchId}
        // participantIdentities.player.summonerName (and ID)
    }

    // Interested in:
    // Our teams role names -> get summoner name for each role
    // Result of the game (win/loss/none)
    // KDA of each person?

    println!("end of main\n");
}
