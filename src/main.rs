use clap::Parser;
use std::collections::HashMap;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
   file: String,
}



fn main() {
    let args = Cli::parse();
    let content : String = std::fs::read_to_string(&args.file).expect("Could not read file!");
    let mut amounts = HashMap::new();

    for line in content.lines(){
        for word in line.split(" "){
            if amounts.get(word).copied().unwrap_or(0) == 0 {
                amounts.insert(word, word_counter(word, &content));
            }
        }
    }

    let mut amount_vec: Vec<(&&str, &i32)> = amounts.iter().collect();
    amount_vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());

    for entry in amount_vec{
        println!("{}, {}", entry.0, entry.1);
    }
}

fn word_counter(term: &str, content: &String) -> i32 {
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
