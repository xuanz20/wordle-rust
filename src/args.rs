use lazy_static::*;
use crate::sync::UPSafeCell;

lazy_static! {
    pub static ref IS_TTY: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_WORD: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_RANDOM: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_DIFFICULT: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_STATS: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_DAY: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_SEED: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref WORD: UPSafeCell<Option<String>> = unsafe { UPSafeCell::new(None) };
    pub static ref DAY: UPSafeCell<usize> = unsafe { UPSafeCell::new(1) };
    pub static ref SEED: UPSafeCell<u64> = unsafe { UPSafeCell::new(0) };
}

pub fn is_tty() -> bool { *IS_TTY.exclusive_access() }
pub fn is_word() -> bool { *IS_WORD.exclusive_access() }
pub fn is_random() -> bool { *IS_RANDOM.exclusive_access() }
pub fn is_difficult() -> bool { *IS_DIFFICULT.exclusive_access() }
pub fn is_stats() -> bool { *IS_STATS.exclusive_access() }
pub fn is_day() -> bool { *IS_DAY.exclusive_access() }
pub fn is_seed() -> bool { *IS_SEED.exclusive_access() }
pub fn get_day() -> usize { *DAY.exclusive_access() - 1 }
pub fn get_seed() -> u64 { *SEED.exclusive_access() }

pub fn args_parse() {
    let mut meet_word = false;
    let mut meet_day = false;
    let mut meet_seed = false;

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

        match arg.as_str() {
            "-w" | "--word" => { *IS_WORD.exclusive_access() = true; meet_word = true; },
            "-r" | "--random" => { *IS_RANDOM.exclusive_access() = true; },
            "-D" | "--difficult" => { *IS_DIFFICULT.exclusive_access() = true; },
            "-t" | "--stats" => { *IS_STATS.exclusive_access() = true; },
            "-d" | "--day" => { *IS_DAY.exclusive_access() = true; meet_day = true; },
            "-s" | "--seed" => { *IS_SEED.exclusive_access() = true; meet_seed = true; },
            _ => (),
        }
    }
    if (is_random() && is_word()) || (!is_random() && is_day()) || (!is_random() && is_seed()) {
        panic!();
    }
}