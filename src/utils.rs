use crate::builtin_words::ACCEPTABLE;

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

#[derive(Clone, Copy, Debug)]
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