use crate::latexcounter::LatexCounter;
use crate::standardcounter::StandardCounter;

pub trait Counter{
    // A function that calls word_counter
    fn word_counter_helper(&self);
    // The function that counts the words
    fn word_counter(&self, term: &str, content: &String) -> i32 ;
}

// Makes counter based on file extension
pub fn make_counter(input: String, file: String) -> Box<dyn Counter>{
    let mode =
        match file.rsplit_once(".").unwrap().1{
            "tex" => "latex",
            _ => ""
        };
    match mode{
        "latex" => {
            Box::new(LatexCounter {
                content: input,
            })
        },
        _ => {
            Box::new(StandardCounter {
                content: input,
            })
        }
    }
}