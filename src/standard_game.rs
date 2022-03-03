
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
        let current = words.word(5).to_string();
        G { words
          , current 
          , length: 5
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
                r[i] = Guess::Correct(u[i])
            }
            else {
                present.push(a[i]);
            }
        }

        for i in 0..a.len() {
            if a[i] != u[i] && present.contains(&u[i]) {
                r[i] = Guess::Present(u[i])
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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_evaluate_wrong_guess() {
        let mut g = G { length: 4, current: "RRBB".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("XXXX");

        assert!( matches!(&guess[..], [Guess::Wrong, Guess::Wrong, Guess::Wrong, Guess::Wrong]) );
    }

    #[test]
    fn should_evaluate_one_correct() {
        let mut g = G { length: 4, current: "XAAA".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("XBBB");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong, Guess::Wrong, Guess::Wrong]) );
    }

    #[test]
    fn should_evaluate_two_correct() {
        let mut g = G { length: 4, current: "XAAY".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("XBBY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong, Guess::Wrong, Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_three_correct() {
        let mut g = G { length: 4, current: "XAZY".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("XBZY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong, Guess::Correct(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_four_correct() {
        let mut g = G { length: 4, current: "XWZY".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("XWZY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Correct(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_one_present() {
        let mut g = G { length: 4, current: "ZAZZ".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("XXXA");

        assert!( matches!(&guess[..], [Guess::Wrong, Guess::Wrong, Guess::Wrong, Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_two_present() {
        let mut g = G { length: 4, current: "ZAZB".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("BXXA");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Wrong, Guess::Wrong, Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_three_present() {
        let mut g = G { length: 4, current: "CAZB".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("BCXA");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Present(_), Guess::Wrong, Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_four_present() {
        let mut g = G { length: 4, current: "CADB".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("BCAD");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Present(_), Guess::Present(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_with_no_present_for_extra() {
        let mut g = G { length: 4, current: "RRBB".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("RRRX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Wrong, Guess::Wrong]) );
    }

    #[test]
    fn should_evaluate_with_present_for_extra() {
        let mut g = G { length: 4, current: "RRBR".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("RRRX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Present(_), Guess::Wrong]) );
    }

    #[test]
    fn should_evaluate_with_present_and_correct() {
        let mut g = G { length: 4, current: "ABDB".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("ABBX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Present(_), Guess::Wrong]) );
    }

    #[test]
    fn should_evaluate_with_correct_but_no_present_for_consumed_correct() {
        let mut g = G { length: 4, current: "RRBB".to_string(), words: Standard::new()};
        let guess = g.evaluate_guess("RRRB");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Wrong, Guess::Correct(_)]) );
    }
}