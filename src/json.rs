use serde_derive::{ Serialize, Deserialize };

use crate::{run::{ANSWER_ARR, update_guess_arr, TOTAL_SUCCESS, TOTAL_FAILURE, TOTAL_SUCCESS_GUESS_TIMES}, args::STATE};

#[derive(Serialize, Deserialize, Debug)]
pub struct Game {
    pub answer: String,
    pub guesses: Vec<String>
}

#[derive(Serialize, Deserialize, Debug)]
pub struct State {
    pub total_rounds: i32,
    pub games: Vec<Game>
}

pub fn parse_json(path: &String) {
    let mut state = STATE.exclusive_access();
    let contents = std::fs::read_to_string(path).unwrap();
    if contents.clone() != "{}" {
        *state = serde_json::from_str::<State>(&contents).unwrap();
    }
    parse_state(&state);
}

fn parse_state(state: &State) {
    let total_rounds = state.total_rounds;
    for i in 0..total_rounds as usize {
        let answer = state.games[i].answer.clone().to_ascii_lowercase();
        let len = state.games[i].guesses.len();
        ANSWER_ARR.exclusive_access().push(answer.clone());
        for j in 0..len {
            let guess = state.games[i].guesses[j].clone().to_ascii_lowercase();
            update_guess_arr(&guess);
            if j == len - 1 {
                if guess == answer {
                    *TOTAL_SUCCESS.exclusive_access() += 1;
                    *TOTAL_SUCCESS_GUESS_TIMES.exclusive_access() += len as i32;
                } else {
                    *TOTAL_FAILURE.exclusive_access() += 1;
                }
            }
        }
    }
}

pub fn write_state(path: &String) {
    std::fs::write(
        path,
        serde_json::to_string(&*STATE.exclusive_access()).unwrap()
    ).unwrap();
}