use clap::Parser;
use std::collections::HashMap;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
   file: String,
   term: String,
}

const content : String;

fn main() {
    let args = Cli::parse();
    content = std::fs::read_to_string(&args.file).expect("Could not read file!");
    let mut amounts = HashMap::new();

    for line in content.lines(){
        for word in line.split(" "){
            if amounts.get(&word).copied().unwrap_or(0) == 0 {
                amounts.insert(word, wordCounter(word));
            }
        }
    }

    for key in amounts {
        println!("{}, {}", &key, amounts.get(&key));
    }
}

fn wordCounter(term: String) -> i32 {
    let mut counter = 0;
    for line in content.lines(){
        for word in line.split(" "){
            if word == term {
                counter += 1;
            }
        }
    }
    counter
}
