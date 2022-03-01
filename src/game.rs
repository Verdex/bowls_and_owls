
pub enum Guess { 
    Correct(char),
    Present(char),
    Wrong,
}

pub fn format_guess(guess : Vec<Guess>) -> String {
    fn x(g : Guess) -> String {
        match g {
            Guess::Correct(c) => format!("|{c}|"),
            Guess::Present(c) => format!("~{c}~"),
            Guess::Wrong => " _ ".to_string(),
        }
    }

    guess.into_iter().map(x).collect()
}

pub trait Game {
    fn evaluate_guess(&mut self, user_guess : &str, right_answer : &str) -> Vec<Guess>;
}