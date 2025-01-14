mod builtin_words;
mod sync;
mod run;
mod utils;
mod args;
mod json;
mod config;
mod tui;

use args::*;

/// The main function for the Wordle game, implement your own logic here
fn main() -> Result<(), Box<dyn std::error::Error>> {
    args_parse();
    if is_tui() {
        tui::run()
    } else {
        run::run()
    }
}
