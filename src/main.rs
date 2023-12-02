use clap::Parser;

/// Search for a pattern in a file and display the lines that contain it.
#[derive(Parser)]
struct Cli {
    /// The pattern to look for
   file: String,
   term: String,
}

fn main() {
    let args = Cli::parse();
    let content = std::fs::read_to_string(&args.file).expect("Could not read file!");
    let mut counter = 0;
    for line in content.lines(){
        for word in line.split(" "){
            if word == &args.term {
                counter += 1;
            }
        }
    }
    println!("{}", counter);
}
