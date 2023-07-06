use std::{collections::{BTreeMap, BTreeSet}, cmp::Reverse};

use rand::{seq::SliceRandom};

use crate::{args::*, run::*, json::Game};

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

    pub fn printc(&self, num: usize) {
        let cha = (b'A' + num as u8) as char;
        match &self {
            Status::G => print!("{}", console::style(format!("{}", cha)).green()),
            Status::Y => print!("{}", console::style(format!("{}", cha)).yellow()),
            Status::R => print!("{}", console::style(format!("{}", cha)).red()),
            Status::X => print!("{}", cha),
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
        if is_tty() {
            println!(
                "{}",
                console::style("Enter a 5-length lowercase word. Try again.").red()
            )
        } else {
            println!("INVALID");
        }
        return false;
    }
    for ch in input.chars() {
        if !ch.is_ascii_lowercase() {
            if is_tty() {
                println!(
                    "{}",
                    console::style("Enter a 5-length lowercase word. Try again.").red()
                )
            } else {
                println!("INVALID");
            }
            return false;
        }
    }
    if !ACCEPTABLE_SET.exclusive_access().contains(input) {
        if is_tty() {
            println!(
                "{}",
                console::style("Your input is not in the acceptable list. Try again.").red()
            )
        } else {
            println!("INVALID");
        }
        return false;
    }
    true
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

pub fn final_len() -> usize {
    return FINAL_SET.exclusive_access().len();
}

pub fn get_final_index(index: usize) -> String {
    return FINAL_SET.exclusive_access()[index].clone();
}

pub fn get_final_random() -> String {
    return FINAL_SET.exclusive_access().choose(&mut rand::thread_rng()).unwrap().clone();
}

pub fn print_top_five() {
    if is_tty() {
        println!("Your favorite guess words:");
    }
    let mut sorted_map: BTreeMap<_, BTreeSet<_>> = BTreeMap::new();
    let guess_arr_ref = &*GUESS_ARR.exclusive_access();
    for (key, value) in guess_arr_ref.iter() {
        sorted_map
            .entry(Reverse(value))
            .or_insert_with(BTreeSet::new)
            .insert(key.clone());
    }
    let mut count = 0;
    let mut first = true;
    for (Reverse(value), keys) in sorted_map.iter() {
        for key in keys {
            if first {
                first = false;
            } else {
                print!(" ");
            }
            if is_tty() {
                print!(
                    "{} {}",
                    console::style(format!("{}", key.to_uppercase())).blue(),
                    value
                );
            } else {
                print!("{} {}", key.to_uppercase(), value);
            }
            count += 1;
            if count == 5 {
                break;
            }
        }
    }
    println!("");
}

pub fn update_stats(success: bool, times: i32) {
    let mut total_sucess = TOTAL_SUCCESS.exclusive_access();
    let mut total_failure= TOTAL_FAILURE.exclusive_access();
    let mut total_success_guess_times = TOTAL_SUCCESS_GUESS_TIMES.exclusive_access();
    if success {
        *total_sucess += 1;
        *total_success_guess_times += times;
    } else {
        *total_failure += 1;
    }
    if is_stats() {
        if is_tty() {
            println!(
                "Total success: {}. Total failure: {}. Average success guess time: {}",
                console::style(format!("{}", *total_sucess)).green(),
                console::style(format!("{}", *total_failure)).yellow(),
                if *total_sucess > 0 {
                    *total_success_guess_times as f64 / *total_sucess as f64
                } else {
                    0.00
                }
            );
            print_top_five();
        } else {
            println!(
                "{} {} {:.2}", 
                *total_sucess, 
                *total_failure, 
                if *total_sucess > 0 {
                    *total_success_guess_times as f64 / *total_sucess as f64
                } else {
                    0.00
                }
            );
            print_top_five();
        }
    }
}

pub fn update_guess_arr(guess_str: &String) {
    GUESS_ARR.exclusive_access().entry(guess_str.clone()).or_insert(0);
    if let Some(value) = GUESS_ARR.exclusive_access().get_mut(guess_str) {
        *value += 1;
    }
}

pub fn update_state(game: Game) {
    let mut state = STATE.exclusive_access();
    state.total_rounds += 1;
    state.games.push(game);
}

pub fn update_pos(guess: &[usize; 5], result: &Vec<Status>) {
    let mut pos = POSSIBLE_SET.exclusive_access();
    if pos.len() == 0 {
        *pos = ACCEPTABLE_SET.exclusive_access().clone();
    }
    pos.retain(|w| is_pos_word(&w, guess, result));
}

pub fn print_pos() {
    if is_tty() {
        println!("{}", console::style("All possible words:").blue())
    } else {
        println!("All possible words:");
    }
    let pos = POSSIBLE_SET.exclusive_access();
    pos.iter().for_each(|w| print!("{} ", w));
    println!("");
    if is_tty() {
        println!("{} {}", console::style(format!("{}", pos.len())).green(), console::style("possible words in total").blue())
    } else {
        println!("{} possible words in total", pos.len());
    }
}

pub fn is_pos_word(w: &String, guess: &[usize; 5], result: &Vec<Status>) -> bool {
    let word  = str2arr(w);
    let mut word_times = count_times(&word);
    for i in 0..5usize {
        if result[i] == Status::G {
            if word[i] != guess[i] {
                return false;
            }
            word_times[guess[i]] -= 1;
        }
    }
    for i in 0..5usize {
        if result[i] == Status::Y {
            if word_times[guess[i]] == 0 {
                return false;
            }
            if word[i] == guess[i] {
                return false;
            }
            word_times[guess[i]] -= 1;
        }
    }
    for i in 0..5usize {
        if result[i] == Status::R {
            if word_times[guess[i]] > 0 {
                return false;
            }
        }
    }
    true
}

pub fn print_rec() {
    let pos = POSSIBLE_SET.exclusive_access().clone();
    let mut entropy: Vec<(&str, f64)> = pos.iter().map(|s| (s.as_str(), cal_entropy(s, &pos))).collect();
    entropy.sort_by(|(_, a), (_, b)| b.partial_cmp(a).unwrap());
    if is_tty() {
        println!("{}", console::style("Some recommend words:").blue());
    }
    for (key, value) in entropy.iter().take(10) {
        print!("{}: {:.2} ", key, value);
    }
    println!("");
}

pub fn cal_entropy(w: &String, pos: &Vec<String>) -> f64 {
    let total_num  = pos.len() as f64;
    let mut entropy: f64 = 0.0;
    let mut all_times: [i32; 1024] = [0; 1024];
    let word = str2arr(w);
    let word_times = count_times(&word);
    pos.iter().for_each(|s| {
        let index = get_index(&word, s, &word_times);
        all_times[index] += 1;
    });
    for i in all_times.iter() {
        let pos = *i as f64 / total_num;
        if pos > 0.0 {
            entropy -= pos * pos.log2();
        }
    }
    entropy
}

pub fn get_index(answer: &[usize; 5], s: &String, ans_times: &[i32; 26]) -> usize {
    let mut index = 0;
    let mut result: [Status; 5] = [Status::X; 5];
    let guess = str2arr(s);
    let mut guess_times = [0; 26];
    for i in 0..5usize { // check for each char
        let cha = guess[i];
        guess_times[cha] += 1;
        if cha == answer[i] {
            result[i] = Status::G;
            if i > 0 {
                let mut cu_cha_times = guess_times[cha];
                for j in (0..=(i - 1)).rev() {
                    if guess[j] == cha {
                        if result[j] == Status::G {
                            continue;
                        } else if cu_cha_times > ans_times[cha] {
                            result[j] = Status::R;
                            cu_cha_times -= 1;
                        }
                    }
                }
            }
        } else {
            if guess_times[cha] <= ans_times[cha] {
                result[i] = Status::Y;
            } else {
                result[i] = Status::R;
            }
        }
    }
    for i in 0..5usize {
        match result[i] {
            Status::G => index += 0 * 4_usize.pow(i as u32),
            Status::Y => index += 1 * 4_usize.pow(i as u32),
            Status::R => index += 2 * 4_usize.pow(i as u32),
            Status::X => index += 3 * 4_usize.pow(i as u32),
        }
    }
    index
}