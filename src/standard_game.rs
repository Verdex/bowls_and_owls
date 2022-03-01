
use crate::game::{Guess, Game, Score};
use crate::word::Standard;

pub struct G {
    words : Standard,
    current : String,
    length : usize,
}

impl G {
    pub fn new() -> Self {
        let mut words = Standard::new();
        let current = words.word(3).to_string();
        G { words
          , current 
          , length: 3
          }
    }
}

impl Game for G {
    fn evaluate_guess(&mut self, user_guess : &str) -> Vec<Guess> {
        let a = self.current.chars().collect::<Vec<char>>();
        let u = user_guess.chars().collect::<Vec<char>>();
        let mut r = (0..a.len()).map(|_| Guess::Wrong).collect::<Vec<Guess>>();

        let mut present = vec![];
        for i in 0..a.len() {
            if a[i] == u[i] {
                r[i] = Guess::Correct(a[i])
            }
            else {
                present.push(a[i]);
            }
        }

        for i in 0..a.len() {
            if a[i] != u[i] {
                present.contains(&u[i]);
            }
        }

        r
    }

    fn letter_count(&self) -> usize {
        self.current.len()
    }

    fn next_word(&mut self) {
        self.length += 1;
        self.current = self.words.word(self.length).to_string();
    }
}

pub struct S {
    retries : isize,
}

impl S { 
    pub fn new(retries : isize) -> Self {
        S { retries
   
          }
    }
}

impl Score for S {
    fn score_guess(&mut self, guess : &Vec<Guess>) {

    }

    fn should_continue(&self) -> bool {
        self.retries > 0
    }
    
    fn score(&self) -> i64 {
        self.retries.try_into().unwrap()
    }
}