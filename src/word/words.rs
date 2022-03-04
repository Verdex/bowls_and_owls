
use std::collections::HashMap;

// TODO it would be nice if the hash map get_words creates is also static

static WORDS : &'static str = include_str!("words.txt");

pub fn get_words() -> HashMap<usize, Vec<&'static str>> {
    let mut r : HashMap<usize, Vec<&'static str>> = HashMap::new();
    for word in WORDS.split('\n') {
        let w = word.trim_end();
        let length = w.len();
        match r.get_mut(&length) {
            Some(v) => v.push(w),
            None => { r.insert(length, vec![w]); },
        }
    }
    r
}