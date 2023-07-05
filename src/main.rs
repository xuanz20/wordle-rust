use console;
use std::io::{self, Write};

mod builtin_words;
mod sync;
mod run;
mod utils;
mod args;
mod json;

use args::*;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    // let is_tty = atty::is(atty::Stream::Stdout);
    let mut is_tty = IS_TTY.exclusive_access();
    *is_tty = atty::is(atty::Stream::Stdout);
    args_parse();
    if *is_tty {
        println!(
            "I am in a tty. Please print {}!",
            console::style("colorful characters").bold().blink().blue()
        );
        print!("{}", console::style("Your name: ").bold().red());
        io::stdout().flush().unwrap();
    } else {
        // println!("I am not in a tty. Please print according to test requirements!");
        run::run();
    }
    
    Ok(())
}
