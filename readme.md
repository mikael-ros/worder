This is a super basic word counter I made as a learning project. It counts the frequencies of each word and then summarizes that into a total word count. Nothing huge at all, but wanted to publicize it just to show some learning milestones.

It auto detects mode based on the file extension. The only currently supported (that is, it filters out syntax) file extension is .tex (LaTeX). I'm probably not going to make more, since it would be trivial. 
Any other file extensions get treated as any old text-file.

Given that this is a tiny hobby thing, I'm not going to spend too much time on how to setup, but you'll need Rust to build and run the file

### Usage
With Rust installed, run (in the directory):

cargo run [file to be word-counted]

### Output
The program should spit out a list of words sorted by frequency, then a number that corresponds to the total word count.

