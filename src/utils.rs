use crate::builtin_words::ACCEPTABLE;

#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Status {
    G,
    Y,
    R,
    X
}

impl Status {
    pub fn update(&mut self, new: Status) {
        match (&self, new) {
            (Status::Y, Status::G) | 
            (Status::R, Status::G) |
            (Status::X, Status::G) |
            (Status::R, Status::Y) |
            (Status::X, Status::Y) |
            (Status::X, Status::R) => *self = new,
            (_, _) => (),
        }
    }

    pub fn print(&self) {
        match &self {
            Status::G => print!("G"),
            Status::Y => print!("Y"),
            Status::R => print!("R"),
            Status::X => print!("X"),
        }
    }
}

pub fn str2arr(str: &String) -> [usize; 5] {
    assert_eq!(str.len(), 5);
    let mut arr = [0; 5];
    for (i, char) in str.chars().enumerate() {
        arr[i] = cha2num(char);
    }
    arr
}

pub fn cha2num(cha: char) -> usize {
    let num = cha as i32 - b'a' as i32;
    assert!(num >= 0 && num < 26);
    num as usize
}

pub fn count_times(arr: &[usize; 5]) -> [i32; 26] {
    let mut res = [0; 26];
    for i in 0..5usize {
        res[arr[i]] += 1;
    }
    res
}

pub fn valid(input: &String) -> bool {
    if input.len() != 5 {
        return false;
    }
    for ch in input.chars() {
        if !ch.is_ascii_lowercase() {
            return false;
        }
    }
    return ACCEPTABLE.contains(&input.as_str());
}

pub fn difficult_valid(last_guess: &[usize; 5], guess: &[usize; 5], result: &Vec<Status>) -> bool {
    let mut guess_times = count_times(guess);
    for i in 0..5usize {
        if result[i] == Status::G {
            if guess[i] != last_guess[i] {
                return false;
            }
            guess_times[last_guess[i]] -= 1;
        }
    }
    for i in 0..5usize {
        if result[i] == Status::Y {
            if guess_times[last_guess[i]] == 0 {
                return false;
            }
            guess_times[last_guess[i]] -= 1;
        }
    }
    true
}