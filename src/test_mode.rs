use crate::{utils::*, args::*, builtin_words::FINAL};
use rand::seq::SliceRandom;

pub fn run() {
    let mut ans_str: String = String::new();
    if is_word() {
        let word = WORD.exclusive_access();
        ans_str = word.as_ref().unwrap().clone();
    } else if is_random() {
        ans_str = FINAL.choose(&mut rand::thread_rng()).unwrap().to_string();
    } else {
        ans_str = text_io::read!();
    }
    let answer = str2arr(&ans_str);
    let ans_times = count_times(&answer);
    let mut status = vec![Status::X; 26];
    let mut result = vec![Status::X; 5];
    let mut times = 1;
    while times <= 6 { // each guess
        let mut success = true;
        let mut guess = String::new();
        std::io::stdin().read_line(&mut guess).expect("INPUT ERROR");
        guess.pop();
        if !valid(&guess) {
            println!("INVALID");
            continue;
        }
        let guess = str2arr(&guess);
        if is_difficult() && !difficult_valid(&answer, &guess, &result) {
            println!("INVALID");
            continue;
        }
        let mut guess_times = [0; 26];
        for i in 0..5usize { // check for each char
            let cha = guess[i];
            guess_times[cha] += 1;
            if cha == answer[i] {
                result[i] = Status::G;
                status[cha].update(Status::G);
                // update the previous status
                
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
            return;
        }
        times += 1;
    }
    println!("FAILED {}", ans_str.to_uppercase());
}