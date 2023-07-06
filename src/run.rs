use crate::json::{Game, write_state};
use crate::{utils::*, args::*, sync::UPSafeCell};
use rand::seq::SliceRandom;
use lazy_static::*;
use std::collections::{BTreeMap, BTreeSet};
use std::cmp::Reverse;
use rand::SeedableRng;

lazy_static! {
    pub static ref ANSWER_ARR: UPSafeCell<Vec<String>> = unsafe { UPSafeCell::new(Vec::new()) };
    pub static ref GUESS_ARR: UPSafeCell<BTreeMap<String, i32>> = unsafe { UPSafeCell::new(BTreeMap::new()) };
    pub static ref TOTAL_SUCCESS: UPSafeCell<i32> = unsafe { UPSafeCell::new(0) };
    pub static ref TOTAL_FAILURE: UPSafeCell<i32> = unsafe { UPSafeCell::new(0) };
    pub static ref TOTAL_SUCCESS_GUESS_TIMES: UPSafeCell<i32> = unsafe { UPSafeCell::new(0) };
}

pub fn run_one_time() -> (bool, i32, Game) {
    let mut game: Game = Game { answer: String::new(), guesses: Vec::new() };
    let mut ans_str: String;
    if is_word() {
        let word = WORD.exclusive_access();
        ans_str = word.as_ref().unwrap().clone();
    } else if is_random() {
        if is_seed() { // with seed
            let seed = get_seed();
            let mut rng = rand::rngs::StdRng::seed_from_u64(seed);
            let len = final_len();
            let mut arr = Vec::with_capacity(len);
            for i in 0 .. len {
                arr.push(i);
            }
            arr.shuffle(& mut rng);
            let index = get_day();
            ans_str = get_final_index(arr[index as usize]);
        } else {
            loop {
                ans_str = get_final_random();
                let mut random_answer = ANSWER_ARR.exclusive_access();
                if !random_answer.contains(&ans_str) {
                    random_answer.push(ans_str.clone());
                    break;
                }
            }
        }
    } else {
        ans_str = text_io::read!();
    }
    game.answer = ans_str.clone().to_ascii_uppercase();
    let answer = str2arr(&ans_str);
    let ans_times = count_times(&answer);
    let mut status = vec![Status::X; 26];
    let mut result = vec![Status::X; 5];
    let mut times = 1;
    let mut last_guess = [0 as usize; 5];
    while times <= 6 { // each guess
        let mut success = true;
        let mut guess_str = String::new();
        std::io::stdin().read_line(&mut guess_str).expect("INPUT ERROR");
        guess_str.pop();
        if !valid(&guess_str) {
            println!("INVALID");
            continue;
        }
        let guess = str2arr(&guess_str);
        if is_difficult() && !difficult_valid(&last_guess, &guess, &result) {
            println!("INVALID");
            continue;
        }
        update_guess_arr(&guess_str);
        game.guesses.push(guess_str.clone().to_ascii_uppercase());
        let mut guess_times = [0; 26];
        for i in 0..5usize { // check for each char
            let cha = guess[i];
            guess_times[cha] += 1;
            if cha == answer[i] {
                result[i] = Status::G;
                status[cha].update(Status::G);
                // update the previous status
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
                success = false;
                if guess_times[cha] <= ans_times[cha] {
                    result[i] = Status::Y;
                    status[cha].update(Status::Y);
                } else {
                    result[i] = Status::R;
                    status[cha].update(Status::R);
                }
            }
        }
        // print the guess result
        result.iter().for_each(|x| x.print());
        print!(" ");
        status.iter().for_each(|x| x.print());
        print!("\n");
        if success {
            println!("CORRECT {}", times);
            return (true, times, game);
        }
        if times != 6 && is_pos() {
            print_pos(&guess, &result);
        }
        times += 1;
        last_guess = guess;
    }
    println!("FAILED {}", ans_str.to_uppercase());
    (false, 0, game)
}

pub fn run() {
    if is_word() {
        let (success, times, game) = run_one_time();
        update_stats(success, times);
        update_state(game);
    } else {
        loop {
            let (success, times, game) = run_one_time();
            update_stats(success, times);

            if is_day() {
                *DAY.exclusive_access() += 1;
            }

            update_state(game);

            let mut next = String::new();
            std::io::stdin().read_line(&mut next).expect("INPUT ERROR");
            if next.len() > 1 {
                next.pop();
            }
            match next.as_str() {
                "N" => {break;},
                _ => (),
            }
        }
    }

    if is_state() {
        write_state(&STATE_PATH.exclusive_access());
    }
}

fn print_top_five() {
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
            print!("{} {}", key.to_uppercase(), value);
            count += 1;
            if count == 5 {
                break;
            }
        }
    }
    println!("");
}

fn update_stats(success: bool, times: i32) {
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
        println!("{} {} {:.2}", 
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

pub fn print_pos(guess: &[usize; 5], result: &Vec<Status>) {
    let mut pos = POSSIBLE_SET.exclusive_access();
    if pos.len() == 0 {
        *pos = ACCEPTABLE_SET.exclusive_access().clone();
    }
    pos.retain(|w| is_pos_word(&w, guess, result));
    println!("All possible words:");
    pos.iter().for_each(|w| print!("{} ", w));
    println!("");
    println!("{} possible words in total", pos.len());
}

fn is_pos_word(w: &String, guess: &[usize; 5], result: &Vec<Status>) -> bool {
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