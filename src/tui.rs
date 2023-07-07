use crossterm::{
    event::{self, Event as CEvent, KeyCode},
    terminal::{disable_raw_mode, enable_raw_mode},
};
use crate::{utils::*, args::*, run::{ANSWER_ARR}, json::Game};
use rand::{prelude::*};
use std::sync::mpsc;
use std::thread;
use std::time::{Duration, Instant};
use tui::{
    backend::CrosstermBackend,
    layout::{Alignment, Constraint, Direction, Layout},
    style::{Color, Style},
    text::{Span, Spans},
    widgets::{
        Block, BorderType, Borders, Paragraph,
    },
    Terminal,
};

use crate::args::is_random;

enum Event<I> {
    Input(I),
    Tick,
}

enum State {
    Answer,
    Play,
}

pub fn run() -> Result<(), Box<dyn std::error::Error>> {
    enable_raw_mode().expect("can run in raw mode");
    let mut input = String::new();
    let mut output;
    let mut tui_guess: Vec<String> = Vec::new();
    let mut tui_result: Vec<Vec<Status>> = Vec::new();
    let mut tui_status: Vec<Status> = vec![Status::X; 26];
    let line1 = vec!["Q", "W", "E", "R", "T", "Y", "U", "I", "O", "P"];
    let line2 = vec!["A", "S", "D", "F", "G", "H", "J", "K", "L"];
    let line3 = vec!["Z", "X", "C", "V", "B", "N", "M"];
    let num1: [usize; 10] = [16, 22, 4, 17, 19, 24, 20, 8, 14, 15];
    let num2: [usize; 9] = [0, 18, 3, 5, 6, 7, 8, 10, 11];
    let num3: [usize; 7] = [25, 23, 2, 21, 1, 13, 12];
    let mut game: Game = Game { answer: String::new(), guesses: Vec::new() };
    let mut ans_str: String = String::new();
    // game.answer = ans_str.clone().to_ascii_uppercase();
    let mut answer:[usize; 5] = [0; 5];
    let mut ans_times: [i32; 26] = [0; 26];
    let mut result = vec![Status::X; 5];
    let mut times = 1;
    let mut last_guess = [0 as usize; 5];
    let mut success;

    let (tx, rx) = mpsc::channel();
    let tick_rate = Duration::from_millis(200);
    thread::spawn(move || {
        let mut last_tick = Instant::now();
        loop {
            let timeout = tick_rate
                .checked_sub(last_tick.elapsed())
                .unwrap_or_else(|| Duration::from_secs(0));

            if event::poll(timeout).expect("poll works") {
                if let CEvent::Key(key) = event::read().expect("can read events") {
                    tx.send(Event::Input(key)).expect("can send events");
                }
            }

            if last_tick.elapsed() >= tick_rate {
                if let Ok(_) = tx.send(Event::Tick) {
                    last_tick = Instant::now();
                }
            }
        }
    });

    let stdout =std::io::stdout();
    let backend = CrosstermBackend::new(stdout);
    let mut terminal = Terminal::new(backend)?;
    terminal.clear()?;

    // current state of game
    let mut state = {
        if is_word() || is_random() {
            if is_word() {
                ans_str = WORD.exclusive_access().as_ref().unwrap().clone();
                answer = str2arr(&ans_str);
                ans_times = count_times(&answer);
            } else {
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
                answer = str2arr(&ans_str);
                ans_times = count_times(&answer);
            }
            output = "Round 1".to_string();
            State::Play
        } else {
            output = "Enter the answer".to_string();
            State::Answer

        }
    };

    loop {
        terminal.draw(|rect| {
            let size = rect.size();
            let chunks = Layout::default()
                .direction(Direction::Vertical)
                .margin(2)
                .constraints(
                    [
                        Constraint::Length(3),
                        Constraint::Length(8),
                        Constraint::Min(3),
                        Constraint::Length(3),
                    ]
                    .as_ref(),
                )
                .split(size);
            let title = Paragraph::new(
                "Wordle in Rust"
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Blue))
                    .border_type(BorderType::Plain),
            );
            rect.render_widget(title, chunks[0]);

            let keyboard_chunks = Layout::default()
                .direction(Direction::Horizontal)
                .constraints(
                    [Constraint::Percentage(20), Constraint::Percentage(80)].as_ref(),
                )
                .split(chunks[1]);
            
            let mut left_vec: Vec<Spans> = Vec::new();
            let mut right_vec: Vec<Spans> = Vec::new();

            for i in 0..(times - 1) as usize {
                if tui_guess.len() > 0 {
                    let mut spans: Vec<Span> = Vec::new();
                    let guess = tui_guess[i].clone().to_ascii_uppercase();
                    let result = tui_result[i].clone();
                    for (index, cha) in guess.chars().enumerate() {
                        spans.push(
                            Span::styled(
                                cha.to_string(),
                                Style::default().fg(get_color(&result[index])))
                        )
                    }
                    left_vec.push(Spans::from(spans));
                }
            }

            let mut spans1:Vec<Span> = Vec::new();
            let mut spans2:Vec<Span> = Vec::new();
            let mut spans3:Vec<Span> = Vec::new();
            for i in 0..10 {
                spans1.push(Span::styled(
                    line1[i],
                    Style::default().fg(get_color(&tui_status[num1[i]]))
                ));
            }
            for i in 0..9 {
                spans2.push(Span::styled(
                    line2[i],
                    Style::default().fg(get_color(&tui_status[num2[i]]))
                ));
            }
            for i in 0..7 {
                spans3.push(Span::styled(
                    line3[i],
                    Style::default().fg(get_color(&tui_status[num3[i]]))
                ));
            }
            right_vec.push(Spans::from(spans1));
            right_vec.push(Spans::from(spans2));
            right_vec.push(Spans::from(spans3));

            let left = Paragraph::new(
                left_vec
            )
            .alignment(Alignment::Left)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Blue))
                    .title("guess history")
                    .border_type(BorderType::Plain),
            );
            rect.render_widget(left, keyboard_chunks[0]);

            let right = Paragraph::new(
                right_vec
            )
            .alignment(Alignment::Center)
            .block(
                Block::default()
                    .borders(Borders::ALL)
                    .style(Style::default().fg(Color::Blue))
                    .title("keyboard")
                    .border_type(BorderType::Plain),
            );
            rect.render_widget(right, keyboard_chunks[1]);

            let _output = Paragraph::new(output.clone())
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Blue))
                        .title("output")
                        .border_type(BorderType::Plain),
                );
            rect.render_widget(_output, chunks[2]);

            let _input = Paragraph::new(input.clone())
                .style(Style::default().fg(Color::White))
                .alignment(Alignment::Left)
                .block(
                    Block::default()
                        .borders(Borders::ALL)
                        .style(Style::default().fg(Color::Blue))
                        .title("input")
                        .border_type(BorderType::Plain),
                );
            rect.render_widget(_input, chunks[3]);
        })?;

        match rx.recv()? {
            Event::Input(event) => match event.code {
                KeyCode::Esc => {
                    disable_raw_mode()?;
                    terminal.show_cursor()?;
                    break;
                },
                KeyCode::Char(c) => input.push(c),
                KeyCode::Backspace => { input.pop(); },
                KeyCode::Enter => {
                    match state {
                        State::Answer => {
                            output = "Round 1".to_string();
                            ans_str = input.clone();
                            answer = str2arr(&ans_str);
                            ans_times = count_times(&answer);
                            state = State::Play;
                        },
                        State::Play => {
                            let guess_str = input.clone();
                            if !valid(&guess_str) {
                                output = "Invalid input. Try again.".to_string();
                                input.clear();
                                continue;
                            }
                            let guess = str2arr(&guess_str);
                            if is_difficult() && !difficult_valid(&last_guess, &guess, &result) {
                                output = "Use all your information. Try again.".to_string();
                                input.clear();
                                continue;
                            }
                            update_guess_arr(&guess_str);
                            game.guesses.push(guess_str.clone().to_ascii_uppercase());
                            let mut guess_times = [0; 26];
                            success = true;
                            for i in 0..5usize { // check for each char
                                let cha = guess[i];
                                guess_times[cha] += 1;
                                if cha == answer[i] {
                                    result[i] = Status::G;
                                    tui_status[cha].update(Status::G);
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
                                        tui_status[cha].update(Status::Y);
                                    } else {
                                        result[i] = Status::R;
                                        tui_status[cha].update(Status::R);
                                    }
                                }
                            }

                            tui_guess.push(guess_str.clone());
                            tui_result.push(result.clone());

                            if success {
                                times += 1;
                                output = "You win!".to_string();
                            } else if times == 6 {
                                output = format!("You lose! The answer is {}.", ans_str);
                            } else {
                                times += 1;
                                last_guess = guess;
                                output = format!("Round {}", times);
                            }
                        },
                    }
                    input.clear();
                }
                _ => {}
            },
            Event::Tick => {}
        }






    }
    Ok(())
}

fn get_color(s: &Status) -> Color {
    match s {
        Status::G => Color::Green,
        Status::R => Color::Red,
        Status::Y => Color::Yellow,
        Status::X => Color::White
    }
}