
use crate::game::{Guess, Game};
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
        let a = self.current.chars().collect::<Vec<char>>();
        let u = user_guess.chars().collect::<Vec<char>>();
        let mut r = a.iter().map(|x| Guess::Wrong(*x)).collect::<Vec<Guess>>();

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

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_evaluate_wrong_guess() {
        let mut g = G { length: 4, current: "RRBB", words: Standard::new()};
        let guess = g.evaluate_guess("XXXX");

        assert!( matches!(&guess[..], [Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_one_correct() {
        let mut g = G { length: 4, current: "XAAA", words: Standard::new()};
        let guess = g.evaluate_guess("XBBB");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_two_correct() {
        let mut g = G { length: 4, current: "XAAY", words: Standard::new()};
        let guess = g.evaluate_guess("XBBY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_three_correct() {
        let mut g = G { length: 4, current: "XAZY", words: Standard::new()};
        let guess = g.evaluate_guess("XBZY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong(_), Guess::Correct(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_four_correct() {
        let mut g = G { length: 4, current: "XWZY", words: Standard::new()};
        let guess = g.evaluate_guess("XWZY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Correct(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_one_present() {
        let mut g = G { length: 4, current: "ZAZZ", words: Standard::new()};
        let guess = g.evaluate_guess("XXXA");

        assert!( matches!(&guess[..], [Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_two_present() {
        let mut g = G { length: 4, current: "ZAZB", words: Standard::new()};
        let guess = g.evaluate_guess("BXXA");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_three_present() {
        let mut g = G { length: 4, current: "CAZB", words: Standard::new()};
        let guess = g.evaluate_guess("BCXA");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Present(_), Guess::Wrong(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_four_present() {
        let mut g = G { length: 4, current: "CADB", words: Standard::new()};
        let guess = g.evaluate_guess("BCAD");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Present(_), Guess::Present(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_with_no_present_for_extra() {
        let mut g = G { length: 4, current: "RRBB", words: Standard::new()};
        let guess = g.evaluate_guess("RRRX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Wrong(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_with_present_for_extra() {
        let mut g = G { length: 4, current: "RRBR", words: Standard::new()};
        let guess = g.evaluate_guess("RRRX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Present(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_with_present_and_correct() {
        let mut g = G { length: 4, current: "ABDB", words: Standard::new()};
        let guess = g.evaluate_guess("ABBX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Present(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_with_correct_but_no_present_for_consumed_correct() {
        let mut g = G { length: 4, current: "RRBB", words: Standard::new()};
        let guess = g.evaluate_guess("RRRB");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Wrong(_), Guess::Correct(_)]) );
    }
}