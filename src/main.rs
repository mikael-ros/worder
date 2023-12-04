mod standard_counter;
mod counter;

use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
   file: String,
}

fn main() {
    let args = Cli::parse();
    let content : String = std::fs::read_to_string(&args.file).expect("Could not read file!");
    let counter =
}