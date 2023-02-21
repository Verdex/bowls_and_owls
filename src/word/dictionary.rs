
use std::collections::HashMap;

use whim::random::*;
use whim::rng_algo::*;

use super::words::*;


static mut HASH : Option<HashMap<usize, Vec<&'static str>>> = None; 

unsafe fn raw_get_word(length : usize, rng : &mut Pcg32Shift) -> Option<&'static str> {

    fn g(h : &HashMap<usize, Vec<&'static str>>, length : usize, rng : &mut Pcg32Shift) -> Option<&'static str> {
        let words = h.get(&length)?;
        let index = rng.range(0..words.len() as u32) as usize;
        Some(words[index])
    }

    match &HASH {
        None => {
            let mut r : HashMap<usize, Vec<&'static str>> = HashMap::new();
            for word in CROSSWD.split('\n') {
                let w = word.trim_end();
                let length = w.len();
                match r.get_mut(&length) {
                    Some(v) => v.push(w),
                    None => { r.insert(length, vec![w]); },
                }
            }
            let ret = g(&r, length, rng);
            HASH = Some(r);
            ret
        },
        Some(h) => g(&h, length, rng),
    }
}

pub struct Standard {
    rng : Pcg32Shift,
}

impl Standard {
    pub fn new() -> Self {
        Standard { rng : Pcg32Shift::new()
                 }
    }

    pub fn get_word(&mut self, length : usize) -> Option<&'static str> {
        unsafe { raw_get_word(length, &mut self.rng) }
    }

    pub fn check_word_exists(&mut self, word : &str) -> bool {
        unsafe {
            match &HASH {
                None => false,
                Some(h) => {
                    let maybe_words = h.get(&word.len());
                    let target_words = match maybe_words {
                        Some(v) => v,
                        None => { return false; },
                    };
                    match target_words.iter().find(|w| **w == word) {
                        Some(_) => true,
                        None => false,
                    }
                },
            }
        }
    }
}
