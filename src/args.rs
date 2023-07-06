use lazy_static::*;
use crate::{sync::UPSafeCell, json::{parse_json, State}, config::parse_config, builtin_words::{FINAL, ACCEPTABLE}};
use std::{io::Read, collections::HashSet};

lazy_static! {
    pub static ref IS_TTY: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_WORD: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_RANDOM: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_DIFFICULT: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_STATS: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_DAY: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_SEED: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_FINAL: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_ACCEPTABLE: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_STATE: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_CONFIG: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_POSSIBLE: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_RECOMMEND: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref WORD: UPSafeCell<Option<String>> = unsafe { UPSafeCell::new(None) };
    pub static ref DAY: UPSafeCell<usize> = unsafe { UPSafeCell::new(1) };
    pub static ref SEED: UPSafeCell<u64> = unsafe { UPSafeCell::new(0) };
    pub static ref FINAL_PATH: UPSafeCell<String> = unsafe { UPSafeCell::new(String::new()) };
    pub static ref ACCEPTABLE_PATH: UPSafeCell<String> = unsafe { UPSafeCell::new(String::new()) };
    pub static ref FINAL_SET: UPSafeCell<Vec<String>> = unsafe {
        UPSafeCell::new(FINAL.iter().map(|&s| s.to_owned()).collect())
    };
    pub static ref ACCEPTABLE_SET: UPSafeCell<Vec<String>> = unsafe {
        UPSafeCell::new(ACCEPTABLE.iter().map(|&s| s.to_owned()).collect())
    };
    pub static ref POSSIBLE_SET: UPSafeCell<Vec<String>> = unsafe { UPSafeCell::new(Vec::new()) };
    pub static ref STATE_PATH: UPSafeCell<String> = unsafe { UPSafeCell::new(String::new()) };
    pub static ref STATE: UPSafeCell<State> = unsafe { UPSafeCell::new(State { total_rounds: 0, games: Vec::new() }) };
}

pub fn is_tty() -> bool { *IS_TTY.exclusive_access() }
pub fn is_word() -> bool { *IS_WORD.exclusive_access() }
pub fn is_random() -> bool { *IS_RANDOM.exclusive_access() }
pub fn is_difficult() -> bool { *IS_DIFFICULT.exclusive_access() }
pub fn is_stats() -> bool { *IS_STATS.exclusive_access() }
pub fn is_day() -> bool { *IS_DAY.exclusive_access() }
pub fn is_seed() -> bool { *IS_SEED.exclusive_access() }
pub fn is_final() -> bool { *IS_FINAL.exclusive_access() }
pub fn is_acceptable() -> bool { *IS_ACCEPTABLE.exclusive_access() }
pub fn is_state() -> bool { *IS_STATE.exclusive_access() }
pub fn is_config() -> bool { *IS_CONFIG.exclusive_access() }
pub fn is_pos() -> bool { *IS_POSSIBLE.exclusive_access() }
pub fn is_rec() -> bool { *IS_RECOMMEND.exclusive_access() }
pub fn get_day() -> usize { *DAY.exclusive_access() - 1 }
pub fn get_seed() -> u64 { *SEED.exclusive_access() }

pub fn args_parse() {
    *IS_TTY.exclusive_access() = atty::is(atty::Stream::Stdout);

    let mut meet_word = false;
    let mut meet_day = false;
    let mut meet_seed = false;
    let mut meet_final = false;
    let mut meet_acceptable = false;
    let mut meet_state = false;
    let mut meet_config = false;
    let mut config_path: String = String::new();

    for arg in std::env::args() {
        if meet_word {
            let mut word = WORD.exclusive_access();
            *word = Some(arg);
            meet_word = false;
            continue;
        }
        if meet_day {
            let mut day = DAY.exclusive_access();
            *day = arg.parse::<usize>().unwrap();
            meet_day = false;
            continue;
        }
        if meet_seed {
            let mut seed = SEED.exclusive_access();
            *seed = arg.parse::<u64>().unwrap();
            meet_seed = false;
            continue;
        }
        if meet_final {
            *FINAL_PATH.exclusive_access() = arg;
            meet_final = false;
            continue;
        }
        if meet_acceptable {
            *ACCEPTABLE_PATH.exclusive_access() = arg;
            meet_acceptable = false;
            continue;
        }
        if meet_state {
            let mut state_path = STATE_PATH.exclusive_access();
            *state_path = arg;
            meet_state = false;
            continue;
        }
        if meet_config {
            config_path = arg;
            meet_config = false;
            continue;
        }

        match arg.as_str() {
            "-w" | "--word" => { *IS_WORD.exclusive_access() = true; meet_word = true; },
            "-r" | "--random" => { *IS_RANDOM.exclusive_access() = true; },
            "-D" | "--difficult" => { *IS_DIFFICULT.exclusive_access() = true; },
            "-t" | "--stats" => { *IS_STATS.exclusive_access() = true; },
            "-d" | "--day" => { *IS_DAY.exclusive_access() = true; meet_day = true; },
            "-s" | "--seed" => { *IS_SEED.exclusive_access() = true; meet_seed = true; },
            "-f" | "--final-set" => { *IS_FINAL.exclusive_access() = true; meet_final = true; },
            "-a" | "--acceptable-set" => { *IS_ACCEPTABLE.exclusive_access() = true; meet_acceptable = true; },
            "-S" | "--state" => { *IS_STATE.exclusive_access() = true; meet_state = true; },
            "-c" | "--config" => { *IS_CONFIG.exclusive_access() = true; meet_config = true; }
            "-p" | "--possible" => { *IS_POSSIBLE.exclusive_access() = true; }
            "-R" | "--recommend" => { *IS_RECOMMEND.exclusive_access() = true; }
            _ => (),
        }
    }

    if is_config() {
        parse_config(&config_path);
    }

    if (is_random() && is_word()) || (!is_random() && is_day()) || (!is_random() && is_seed()) {
        panic!();
    }

    if is_final() && is_acceptable() {
        let mut file = std::fs::File::open(FINAL_PATH.exclusive_access().clone()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut final_set = FINAL_SET.exclusive_access();
        *final_set = contents.lines().map(|x| x.to_string()).collect();
        final_set.iter_mut().for_each(|s| s.make_ascii_lowercase());
        final_set.sort();
        let set: HashSet<&str> = final_set.iter().map(|s| s.as_str()).collect();
        if set.len() != final_set.len() { // contains repeated words
            panic!();
        }

        let mut file = std::fs::File::open(ACCEPTABLE_PATH.exclusive_access().clone()).unwrap();
        let mut contents = String::new();
        file.read_to_string(&mut contents).unwrap();
        let mut acceptable_set = ACCEPTABLE_SET.exclusive_access();
        *acceptable_set = contents.lines().map(|x| x.to_string()).collect();
        acceptable_set.iter_mut().for_each(|s| s.make_ascii_lowercase());
        acceptable_set.sort();
        let set: HashSet<&str> = acceptable_set.iter().map(|s| s.as_str()).collect();
        if set.len() != acceptable_set.len() { // contains repeated words
            panic!();
        }
        
        let set1: HashSet<&str> = final_set.iter().map(|s| s.as_str()).collect();
        let set2: HashSet<&str> = acceptable_set.iter().map(|s| s.as_str()).collect();
        if set1.len() != final_set.len() || set2.len() != acceptable_set.len() { // contains repeated words
            panic!();
        }
        let all_in = set1.iter().all(|&s| set2.contains(s));
        if !all_in { // FINAL_SET not all in ACCEPTABLE_SET
            panic!();
        }
    }
    
    if is_state() {
        parse_json(&STATE_PATH.exclusive_access());
    }
}