use serde_derive::{ Deserialize };

use crate::args::*;

#[derive(Deserialize)]
struct Config {
    random: Option<bool>,
    difficult: Option<bool>,
    stats: Option<bool>,
    day: Option<i32>,
    seed: Option<u64>,
    final_set: Option<String>,
    acceptable_set: Option<String>,
    state: Option<String>,
    word: Option<String>
}

pub fn parse_config(path: &String) {
    let contents = std::fs::read_to_string(path).unwrap();
    let config = serde_json::from_str::<Config>(&contents).unwrap();
    if config.random.is_some() && !is_random() {
        *IS_RANDOM.exclusive_access() = config.random.unwrap();
    }
    if config.difficult.is_some() && !is_difficult() {
        *IS_DIFFICULT.exclusive_access() = config.difficult.unwrap();
    }
    if config.stats.is_some() && !is_stats() {
        *IS_STATS.exclusive_access() = config.stats.unwrap();
    }
    if config.day.is_some() && !is_day() {
        *IS_DAY.exclusive_access() = true;
        *DAY.exclusive_access() = config.day.unwrap() as usize;
    }
    if config.seed.is_some() && !is_seed() {
        *IS_SEED.exclusive_access() = true;
        *SEED.exclusive_access() = config.seed.unwrap();
    }
    if config.final_set.is_some() && !is_final() {
        *IS_FINAL.exclusive_access() = true;
        *FINAL_PATH.exclusive_access() = config.final_set.unwrap();
    }
    if config.acceptable_set.is_some() && !is_acceptable() {
        *IS_ACCEPTABLE.exclusive_access() = true;
        *ACCEPTABLE_PATH.exclusive_access() = config.acceptable_set.unwrap();
    }
    if config.state.is_some() && !is_state() {
        *IS_STATE.exclusive_access() = true;
        *STATE_PATH.exclusive_access() = config.state.unwrap();
    }
    if config.word.is_some() && !is_word() {
        *IS_WORD.exclusive_access() = true;
        *WORD.exclusive_access() = Some(config.word.unwrap());
    }
}