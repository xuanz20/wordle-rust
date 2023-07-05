use lazy_static::*;
use crate::sync::UPSafeCell;

lazy_static! {
    pub static ref IS_TTY: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_WORD: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_RANDOM: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_DIFFICULT: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };   
    pub static ref WORD: UPSafeCell<Option<String>> = unsafe { UPSafeCell::new(None) };
}

pub fn is_tty() -> bool { *IS_TTY.exclusive_access() }
pub fn is_word() -> bool { *IS_WORD.exclusive_access() }
pub fn is_random() -> bool { *IS_RANDOM.exclusive_access() }
pub fn is_difficult() -> bool { *IS_DIFFICULT.exclusive_access() }

pub fn args_parse() {
    let mut meet_word = false;
    for arg in std::env::args() {
        if meet_word {
            let mut word = WORD.exclusive_access();
            *word = Some(arg);
            meet_word = false;
            continue;
        }

        match arg.as_str() {
            "-w" | "--word" => {*IS_WORD.exclusive_access() = true; meet_word = true;},
            "-r" | "--random" => {*IS_RANDOM.exclusive_access() = true;},
            "-d" | "--difficult" => {*IS_DIFFICULT.exclusive_access() = true;}
            _ => (),
        }
    }
}