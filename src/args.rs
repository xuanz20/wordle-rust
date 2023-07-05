use lazy_static::*;
use crate::sync::UPSafeCell;

lazy_static! {
    pub static ref IS_TTY: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };
    pub static ref IS_WORD: UPSafeCell<bool> = unsafe { UPSafeCell::new(false) };   
    pub static ref WORD: UPSafeCell<Option<String>> = unsafe { UPSafeCell::new(None) };
}

pub fn is_tty() -> bool {
    *IS_TTY.exclusive_access()
}

pub fn is_word() -> bool {
    *IS_WORD.exclusive_access()
}

pub fn args_parse() {
    for arg in std::env::args() {
        if is_word() {
            let mut word = WORD.exclusive_access();
            *word = Some(arg);
            continue;
        }

        match arg.as_str() {
            "-w" | "--word" => {*IS_WORD.exclusive_access() = true;},
            _ => (),
        }
    }
}