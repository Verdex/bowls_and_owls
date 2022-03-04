
use std::collections::HashMap;
use rand::prelude::*;
use super::words;

pub struct Standard {
    rng : ThreadRng,
    words : HashMap<usize, Vec<&'static str>>,
}

impl Standard {
    pub fn new() -> Self {
        Standard { rng : thread_rng()
                 , words : words::get_words()
                 }
    }

    pub fn word(&mut self, length : usize) -> Option<&'static str> {
        let words = self.words.get(&length)?;
        let index = self.rng.gen_range(0..words.len());
        Some(words[index])
    }
}
