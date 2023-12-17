
use clap::Parser;
use crate::counter::make_counter;
use crate::counter::Counter;
use crate::latexcounter::LatexCounter;
use crate::standardcounter::StandardCounter;
mod standardcounter;
mod counter;
mod latexcounter;

#[derive(Parser)]
struct Cli {
    file: String,
}
fn main() {
    let args = Cli::parse(); // Parses input
    let input: String = std::fs::read_to_string(&args.file).expect("Could not read file!");
    let count = make_counter(input, args.file); // Selects counter type based on file extension
    count.word_counter_helper(); // Calls the helper function which will eventually spit out the output
}