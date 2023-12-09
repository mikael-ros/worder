
use std::collections::HashMap;
use crate::counter::Counter;
pub struct LatexCounter {
    pub content : String,
}

impl Counter for LatexCounter {
    fn word_counter_helper(&self) {
        let mut amounts = HashMap::new();
        let mut depth = 0;

        for line in self.content.lines(){
            for word in line.split(" "){
                if amounts.get(&word).copied().unwrap_or(0) == 0 && !(word.contains("\\") || word.len() == 1 || line.starts_with("%") || word.contains("{") || word.contains("}")){
                    amounts.insert(word, self.word_counter(&word, &self.content));
                }
            }
        }

        let mut amount_vec: Vec<(&&str, &i32)> = amounts.iter().collect();
        amount_vec.sort_by(|a, b| a.1.partial_cmp(b.1).unwrap());
        let sum : i32 = amount_vec.iter().map(|a| {a.1}).sum();

        for entry in amount_vec{
            println!("{}, {}", entry.0, entry.1);
        }
        println!("{}", sum);
    }

    fn word_counter(&self, term: &str, content: &String) -> i32 {
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
}
