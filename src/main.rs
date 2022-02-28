
use std::io;

fn read_line() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
}

enum Guess { 
    Correct(char),
    Present(char),
    Wrong,
}

fn format_guess(guess : Vec<Guess>) -> String {
    fn x(g : Guess) -> String {
        match g {
            Guess::Correct(c) => format!("|{c}|"),
            Guess::Present(c) => format!("~{c}~"),
            Guess::Wrong => " _ ".to_string(),
        }
    }

    guess.into_iter().map(x).collect()
}

fn evaluate_guess(user_guess : &str, right_answer : &str) -> Vec<Guess> {
    vec![]
}

fn main() -> io::Result<()> {
    loop {
        let x = read_line()?;

        println!("{}", x.len());

    }
    Ok(())
}
