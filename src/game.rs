
pub enum Guess { 
    Correct(char),
    Present(char),
    Wrong(char),
}

pub fn format_guess(guess : &Vec<Guess>) -> String {
    fn x(g : &Guess) -> String {
        match g {
            Guess::Correct(c) => format!("|{c}|"),
            Guess::Present(c) => format!("~{c}~"),
            Guess::Wrong(_) => " _ ".to_string(),
        }
    }

    guess.iter().map(x).collect()
}

pub trait Game {
    fn evaluate_guess(&mut self, user_guess : &str) -> Vec<Guess>;
    fn letter_count(&self) -> usize;
    fn next_word(&mut self);
}
