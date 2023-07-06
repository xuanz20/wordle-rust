use crate::json::{Game, write_state};
use crate::{utils::*, args::*, sync::UPSafeCell};
use rand::seq::SliceRandom;
use lazy_static::*;
use std::{collections::BTreeMap, io::{self, Write}};
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
    let mut tty_guess: Vec<[usize; 5]> = Vec::new();
    let mut tty_result: Vec<Vec<Status>> = Vec::new();
    let mut tty_status: Vec<Vec<Status>> = Vec::new();
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
        if is_tty() {
            print!("Enter the answer: ");
        }
        ans_str = text_io::read!();
    }
    game.answer = ans_str.clone().to_ascii_uppercase();
    let answer = str2arr(&ans_str);
    let ans_times = count_times(&answer);
    let mut status = vec![Status::X; 26];
    let mut result = vec![Status::X; 5];
    let mut times = 1;
    let mut last_guess = [0 as usize; 5];
    let mut valid_input = true;

    while times <= 6 { // each guess
        if valid_input {
            if is_tty() {
                println!("");
                println!("Round {}:", console::style(format!("{}", times)).blue());
            }
            if is_pos() || is_rec() {
                update_pos(&last_guess, &result);
            }
            if is_pos() {
                if times == 1 {
                    if is_tty() {
                        println!("{}", console::style("All words are possible.").blue());
                    } else {
                        println!("All words are possible.");
                    }
                } else {
                    print_pos();
                }
            }
            if is_rec() && times != 1 {
                print_rec();
            }
        }
        if is_tty() {
            print!("Enter your input: ");
            io::stdout().flush().unwrap();
        }
        let mut success = true;
        let mut guess_str = String::new();
        std::io::stdin().read_line(&mut guess_str).expect("INPUT ERROR");
        guess_str.pop();
        if !valid(&guess_str) {
            valid_input = false;
            continue;
        }
        let guess = str2arr(&guess_str);
        if is_difficult() && !difficult_valid(&last_guess, &guess, &result) {
            if is_tty() {
                println!("{}",
                console::style("Use all your information. Try again.").red()
            )
            } else {
                println!("INVALID");
            }
            valid_input = false;
            continue;
        }
        if !valid_input {
            valid_input = true;
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
        if is_tty() {
            tty_guess.push(guess);
            tty_result.push(result.clone());
            tty_status.push(status.clone());
            for i in 0..times as usize {
                tty_result[i].iter().enumerate().for_each(|(x, y)| y.printc(tty_guess[i][x]));
                print!(" ");
                tty_status[i].iter().enumerate().for_each(|(x, y)| y.printc(x));
                print!("\n");
            }
        } else {
            result.iter().for_each(|x| x.print());
            print!(" ");
            status.iter().for_each(|x| x.print());
            print!("\n");
        }

        if success {
            if is_tty() {
                println!(
                    "\n{} Your total guess time: {}.",
                    console::style("You win!").green(),
                    times
                );
            } else {
                println!("CORRECT {}", times);
            }
            return (true, times, game);
        }
        times += 1;
        last_guess = guess;
    }
    if is_tty() {
        println!(
            "\n{} The correct answer is {}.",
            console::style("You lose.").yellow(),
            console::style(format!("{}", ans_str.to_uppercase())).green()
        );
    } else {
        println!("FAILED {}", ans_str.to_uppercase());
    }
    (false, 0, game)
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    if is_tty() {
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
        let mut line = String::new();
        io::stdin().read_line(&mut line)?;
        println!("Welcome to wordle, {}!", line.trim());
    }

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

            if is_tty() {
                print!("{} ", console::style("Do you want to start another game? [Y/N]").on_red());
                io::stdout().flush().unwrap();
            }

            let mut next = String::new();
            std::io::stdin().read_line(&mut next).expect("INPUT ERROR");
            if next.len() > 1 {
                next.pop();
            }
            match next.as_str() {
                "N" => {break;},
                _ => {
                    if is_tty() {
                        println!("");
                    }
                },
            }
        }
    }

    if is_state() {
        write_state(&STATE_PATH.exclusive_access());
    }

    Ok(())
}