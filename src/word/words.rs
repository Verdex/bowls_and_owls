
use std::collections::HashMap;

static WORDS : &'static str = include_str!("words.txt");

pub fn get_words() -> HashMap<usize, Vec<&'static str>> {
    let mut r : HashMap<usize, Vec<&'static str>> = HashMap::new();
    for word in WORDS.split('\n') {
        let l = word.len();
        match r.get_mut(&l) {
            Some(v) => v.push(word),
            None => { r.insert(l, vec![]); },
        }
    }
    r
}