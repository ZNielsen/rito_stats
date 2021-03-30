
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

fn main() {
    let encrypted_account_id = "JZO25Xuf5Y0QHfEmXvsDGIc_Or4zB_wowN9ZhB3-nw1ljQ".to_owned();
    let api_endpoint_base = "/lol/match/v4/matchlists/by-account".to_owned();
    println!("end of main\n");
}
