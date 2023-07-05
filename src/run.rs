use crate::{utils::*, args::*, sync::UPSafeCell};
use rand::seq::SliceRandom;
use lazy_static::*;
use std::collections::{BTreeMap, BTreeSet};
use std::cmp::Reverse;
use rand::SeedableRng;

lazy_static! {
    static ref ANSWER_ARR: UPSafeCell<Vec<String>> = unsafe { UPSafeCell::new(Vec::new()) };
    static ref GUESS_ARR: UPSafeCell<BTreeMap<String, i32>> = unsafe { UPSafeCell::new(BTreeMap::new()) };
}

pub fn run_one_time() -> (bool, i32) {
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
        GUESS_ARR.exclusive_access().entry(guess_str.clone()).or_insert(0);
        if let Some(value) = GUESS_ARR.exclusive_access().get_mut(&guess_str) {
            *value += 1;
        }
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
            return (true, times);
        }
        times += 1;
        last_guess = guess;
    }
    println!("FAILED {}", ans_str.to_uppercase());
    (false, 0)
}

pub fn run() {
    let mut total_sucess: i32 = 0;
    let mut total_failure: i32 = 0;
    let mut total_success_guess_times: i32 = 0;
    if is_word() { 
        let (success, times) = run_one_time();
        if is_stats() {
            if success {
                println!("1 0 {:.2}", times as f64 / 6.0);
            } else {
                println!("0 1 0.00");
            }
            print_top_five();
        }
    }
    else {
        loop {
            let (success, times) = run_one_time();
            if is_stats() {
                if success {
                    total_sucess += 1;
                    total_success_guess_times += times;
                } else {
                    total_failure += 1;
                }
                println!("{} {} {:.2}", 
                    total_sucess, 
                    total_failure, 
                    if total_sucess > 0 {
                        total_success_guess_times as f64 / total_sucess as f64
                    } else {
                        0.00
                    }
                );
                print_top_five();
            }
            if is_day() {
                *DAY.exclusive_access() += 1;
            }

            let mut next = String::new();
            std::io::stdin().read_line(&mut next).expect("INPUT ERROR");
            next.pop();
            match next.as_str() {
                "N" => {break;},
                _ => (),
            }
        }
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