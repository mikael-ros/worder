
pub trait Counter{
    fn word_counter_helper(&self);
    fn word_counter(&self, term: &str, content: &String) -> i32 ;
}