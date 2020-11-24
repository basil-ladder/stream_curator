use rand::distributions::Open01;
use rand::{thread_rng, Rng};
use serde::Deserialize;
use std::env;
use std::fs;

#[derive(Debug, Deserialize)]
struct GameListing {
    bots: Vec<Bot>,
    maps: Vec<String>,
    results: Vec<GameResult>,
}

#[derive(Debug, Deserialize)]
struct Bot {
    name: String,
    rating: u16,
}

#[derive(Debug, Deserialize)]
struct GameResult {
    #[serde(rename(deserialize = "botA"))]
    bot_a: BotRef,
    #[serde(rename(deserialize = "botB"))]
    bot_b: BotRef,
    #[serde(rename(deserialize = "mapIndex"))]
    map_index: usize,
    #[serde(rename(deserialize = "gameHash"))]
    game_hash: String,
    #[serde(rename(deserialize = "invalidGame"))]
    invalid_game: bool,
    #[serde(rename(deserialize = "realTimeout"))]
    real_timeout: bool,
    #[serde(rename(deserialize = "frameTimeout"))]
    frame_timeout: bool,
    #[serde(rename(deserialize = "frameCount"))]
    frame_count: u32,
}

#[derive(Debug, Deserialize)]
struct BotRef {
    #[serde(rename(deserialize = "botIndex"))]
    bot_index: usize,
}

fn main() -> Result<(), String> {
    let mut args = env::args();
    args.next();
    let game_list_file = args.next().expect("Path to game list json required!");
    let mut game_listing: GameListing =
        serde_json::from_slice(&fs::read(game_list_file).map_err(|e| e.to_string())?)
            .map_err(|e| e.to_string())?;
    let mut rng = thread_rng();
    let bots = game_listing.bots;
    game_listing.results.truncate(400);
    game_listing
        .results
        .sort_by_key(|g| bots[g.bot_a.bot_index].rating);
    let maps = game_listing.maps;
    let mut replay_files: Vec<String> = game_listing
        .results
        .iter()
        .map(|game| {
            let bot_a = &bots[game.bot_a.bot_index].name;
            format!(
                "{}/{} vs {} {} {}.rep",
                bot_a, bot_a, bots[game.bot_b.bot_index].name, maps[game.map_index], game.game_hash
            )
        })
        .collect();
    replay_files.reverse();
    let mut candidates = vec![];
    let a = 1.161_f32; // 80-20 :D
    while candidates.len() < 40 && !replay_files.is_empty() {
        let h = replay_files.len() as f32;
        let rnd: f32 = rng.sample(Open01);
        let x = (-(rnd * h.powf(a) - rnd * 1_f32 - h.powf(a)) / (h.powf(a))).powf(-1_f32 / a)
            as usize
            - 1;
        candidates.push(replay_files.remove(x));
    }
    for c in candidates.iter() {
        println!("{}", c);
    }

    Ok(())
}
