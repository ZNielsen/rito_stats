#![allow(non_snake_case)]
use reqwest::Client;
use std::vec::Vec;
use std::path::Path;

pub mod structs;
use structs::*;


pub const ENDPOINT: &'static str = "https://na1.api.riotgames.com";
pub const BLUE_SIDE: i64 = 100;
pub const RED_SIDE: i64 = 200;
pub const OUT_DIR: &'static str = "output";

/// Function expects API key to be the only thing in the file
/// Only read once, then store the string statically
fn get_api_key() -> Result<String, std::io::Error> {
    let key = std::fs::read_to_string("api.key")?.replace("\n", "");
    return Ok(key);
}

async fn get_account_info(summ_name: &str) -> Result<Account, Box<dyn std::error::Error>> {
    let slug = String::from("/lol/summoner/v4/summoners/by-name/") + summ_name +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = reqwest::get(request).await?;
    let str = resp.text().await?;
    println!("Account: {}", str);
    let j = serde_json::from_str(&str)?;
    return Ok(j);
}

async fn get_matches(client: &Client, id: &str, start_idx: i64, end_idx: i64) -> Result<Matches, Box<dyn std::error::Error>> {
    let api_endpoint_base = String::from("/lol/match/v4/matchlists/by-account");
    let slug = api_endpoint_base + "/" + id +
        "?endIndex=" + &end_idx.to_string() +
        "&beginIndex=" + &start_idx.to_string() +
        "&api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = client.get(&request).send().await?;

    Ok(resp.json::<Matches>().await?)
}

async fn get_game_info(client: &Client, game_id: &str) -> Result<GameInfo, Box<dyn std::error::Error>> {
    let api_endpoint_base = String::from("/lol/match/v4/matches/");
    let slug = api_endpoint_base + game_id +
        "?api_key=" + &get_api_key()?;
    let request = String::from(ENDPOINT) + &slug;
    println!("Sending reqwest: {}", request);
    let resp = client.get(&request).send().await?;
    let str = resp.text().await?;

    let game = match serde_json::from_str(&str) {
        Ok(g) => g,
        Err(e) => {
            println!("Error decoding match into json: {}", e);
            let file = String::from(OUT_DIR) + "/bad_game.json";
            std::fs::write(file, str)?;
            println!("Bad game written to bad_game.json for inspection");
            return Err(std::boxed::Box::new(e));
        },
    };

    Ok(game)
}

fn data_has_game_info(data: &Vec<GameInfo>, game_id: i64) -> bool {
    for game_data in data {
        if game_data.game_id == game_id {
            return true;
        }
    }
    false
}

/// Wait until the specified amount of time has passed since the last call
fn rate_limiter(last_call: &std::time::Instant, api_rate: &std::time::Duration) {
    let mut time_to_wait: i128 = api_rate.as_millis() as i128 - last_call.elapsed().as_millis() as i128;
    while time_to_wait > 0 {
        let sleep_time = std::time::Duration::from_millis(time_to_wait as u64);
        println!("rate_limiter: Sleeping for {}ms", time_to_wait);
        std::thread::sleep(sleep_time);
        time_to_wait = api_rate.as_millis() as i128 - last_call.elapsed().as_millis() as i128;
    }
}

async fn collect_data(summoner: &str) -> Result<Vec<GameInfo>, Box<dyn std::error::Error>> {
    let mut data = Vec::<GameInfo>::new();

    let account_info = get_account_info(&summoner).await?;
    println!("encrypted account id : {}" , account_info.account_id);

    // See if there is a cache of this summoner
    let cache_file = String::from("cache/") + &account_info.account_id;
    if std::path::Path::new(&cache_file).exists() {
        // Load cached data
        data = serde_json::from_str(&std::fs::read_to_string(&cache_file)?)?;
    }

    // Create a reqwest Client
    let client = reqwest::Client::new();

    let mut more_matches = true;
    // The API has a limit of 100 matches at a time. Grab 100 at a time until there are no more
    let mut start_idx: i64 = 0;
    let mut end_idx: i64 = 100;
    // The API has a limit of 100 calls in 2 minutes
    // 120 sec / 100 calls == 1.2 seconds between calls
    let mut last_call = std::time::Instant::now();
    let api_rate = std::time::Duration::from_millis(1300);
    while more_matches {
        println!("fetching matches with start_idx: {}, end_idx: {}", start_idx, end_idx);
        let matches = get_matches(&client, &account_info.account_id, start_idx, end_idx).await?;
        println!("matches: {:?}", matches);

        // Set up the next indexes
        start_idx = end_idx+1;
        end_idx = start_idx + 100;

        let range_start = matches.start_index;
        let range_end = matches.end_index;
        // TODO - does this have a 1% chance of causing an error?
        more_matches = range_end-range_start == 100;

        println!("range_start: {}, range_end: {}, diff: {}",
            range_start, range_end, range_end-range_start);
        println!("more matches: {}", more_matches);

        for this_match in matches.matches {
            if data_has_game_info(&data, this_match.game_id) {
                // We already have the info for this game, skip making the request
                continue;
            }

            // API has a quota limit, pause so we don't get an error
            rate_limiter(&last_call, &api_rate);

            let game_info = match get_game_info(&client, &this_match.game_id.to_string()).await {
                Ok(gi) => gi,
                Err(e) => {
                    println!("Error in get_game_info: {}", e);
                    more_matches = false;
                    break;
                },
            };
            last_call = std::time::Instant::now();
            data.push(game_info);
            // Overwrite data file on each call
            let serialized = serde_json::to_string(&data)?;
            std::fs::write(&cache_file, serialized)?;
        }
    }

    println!("Writing out data to cache dir");
    let serialized = serde_json::to_string(&data)?;
    std::fs::write(&cache_file, serialized)?;
    Ok(data)
}

fn is_valid_game(game: &GameInfo) -> bool {
    // Has 10 players
    let mut player_count = 0;
    for _ in &game.participant_identities {
        player_count += 1;
    }
    if player_count != 10 {
        // println!("Invalid game: {} players", player_count);
        return false;
    }

    // run time of over 10 min
    if game.game_duration < (10 * 60) {
        // println!("Invalid game: game only {} min", game.game_duration as f64 / 60.0);
        return false;
    }

    // is just a normal game (Summoner's Rift)
    if game.game_mode != "CLASSIC" || game.game_type != "MATCHED_GAME" {
        // println!("Invalid game: mode/type: {}/{}", game.game_mode, game.game_type);
        return false;
    }

    true
}

fn analyze_data(data: &Vec<GameInfo>, summoner_id: &str, counterpart_id: &str) {
    // Assert data is sorted
    assert!(data.len() > 1);
    assert!(data[0].game_creation > data[1].game_creation);
    println!("data.len(): {}", data.len());

    let game_limit = 50;
    let with_idx = 0;
    let without_idx = 1;
    let mut wins: Vec<u32> = [0, 0].to_vec();
    let mut matches: Vec<u32> = [0, 0].to_vec();
    for game in data {

        if !is_valid_game(&game) {
            continue;
        }

        if matches[with_idx] >= game_limit && matches[without_idx] >= game_limit {
            break;
        }

        // Make the zipped iterator
        let iter = game.participants.iter()
            .zip(game.participant_identities.iter())
            .map(|(x, y)| (x, y));
        let mut idx = without_idx;
        let mut win = false;
        for (participant, participant_identity) in iter {
            if &participant_identity.player.summoner_id == counterpart_id {
                idx = with_idx;
            }

            if participant_identity.player.summoner_id == summoner_id {
                for team in &game.teams {
                    if team.team_id == participant.team_id {
                        if team.win == "Win" {
                            win = true;
                        }
                        break;
                    }
                }
            }
        }

        if matches[idx] < game_limit {
            if win {
                wins[idx] += 1;
            }
            matches[idx] += 1;
        }
    }

    println!("Win % with:    {}/{}: {:.2}%",
        wins[with_idx], matches[with_idx], (wins[with_idx] as f64 / matches[with_idx] as f64) * 100.0);
    println!("Win % without: {}/{}: {:.2}%",
        wins[without_idx], matches[without_idx], (wins[without_idx] as f64 / matches[without_idx] as f64) * 100.0);
}

fn print_to_csv(data: &impl CSVable, summoner: &Account) -> Result<(), Box<dyn std::error::Error>>{
    // Make file (with summoner's name)
    let file_name = String::from(OUT_DIR) + "/" + &summoner.name + "_stats.csv";
    data.write_to_csv(Path::new(&file_name), "|")?;
    println!("Wrote csv to output directory");
    Ok(())
}

#[tokio::main]
async fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let _args: Vec<String> = std::env::args().collect();
    std::fs::create_dir(OUT_DIR)?;

    let summoner_name = "Suq Mediq".to_owned();
    let summoner = get_account_info(&summoner_name).await?;

    let counterpart_name = "l Bang Hot Men".to_owned();
    let counterpart = get_account_info(&counterpart_name).await?;

    let mut data: Vec<GameInfo> = collect_data(&summoner_name).await?;
    data.sort_by(|a, b| b.game_creation.cmp(&a.game_creation));

    analyze_data(&data, &summoner.id, &counterpart.id);
    print_to_csv(&data, &summoner)?;

    println!("Done");
    Ok(())
}

