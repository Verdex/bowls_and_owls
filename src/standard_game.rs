
use crate::game::{self, Guess, Game};
use crate::word::Standard;

pub struct G<'a> {
    words : Standard,
    current : &'a str,
    length : usize,
}

impl<'a> G<'a> {
    pub fn new(initial_word : &'a str) -> Self {
        let length = initial_word.len();
        G { words: Standard::new()
          , current: initial_word
          , length
          }
    }
}

impl<'a> Game for G<'a> {
    fn evaluate_guess(&mut self, user_guess : &str) -> Vec<Guess> {
        game::evaluate_guess(self.current, user_guess)
    }

    fn letter_count(&self) -> usize {
        self.current.len()
    }

    fn next_word(&mut self) -> bool {
        self.length += 1;

        let next = match self.words.get_word(self.length) {
            Some(w) => w,
            None => { return false; },
        };

        self.current = next; 
        true
    }
}
