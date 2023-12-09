
use clap::Parser;
use crate::counter::Counter;
use crate::latexcounter::LatexCounter;
use crate::standardcounter::StandardCounter;
mod standardcounter;
mod counter;
mod latexcounter;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
   file: String,
}
fn main() {
    let args = Cli::parse();
    let input: String = std::fs::read_to_string(&args.file).expect("Could not read file!");
    let counter = LatexCounter {
        content : input,
    };
    counter.word_counter_helper();
}