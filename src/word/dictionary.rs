
use std::collections::HashMap;
use rand::prelude::*;

static WORDS : &'static str = include_str!("words.txt");
static mut HASH : Option<HashMap<usize, Vec<&'static str>>> = None; 

unsafe fn raw_get_word(length : usize, rng : &mut ThreadRng) -> Option<&'static str> {

    fn g(h : &HashMap<usize, Vec<&'static str>>, length : usize, rng : &mut ThreadRng) -> Option<&'static str> {
        let words = h.get(&length)?;
        let index = rng.gen_range(0..words.len());
        Some(words[index])
    }

    match &HASH {
        None => {
            let mut r : HashMap<usize, Vec<&'static str>> = HashMap::new();
            for word in WORDS.split('\n') {
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
    rng : ThreadRng,
}

impl Standard {
    pub fn new() -> Self {
        Standard { rng : thread_rng()
                 }
    }

    pub fn get_word(&mut self, length : usize) -> Option<&'static str> {
        unsafe { raw_get_word(length, &mut self.rng) }
    }
}
