
mod game;
mod word;

mod standard_game;

use std::io;
use crate::game::{Game, Guess};

fn read_line() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
}



fn main() -> io::Result<()> {
    let mut game = standard_game::G::new();

    loop {
        let letter_count = game.letter_count();
        println!("word size: {}", letter_count);
        loop {
            let mut line = read_line()?;
            let mut input = line.trim_end();
            while input.len() != letter_count {
                line = read_line()?;
                input = line.trim_end();
            }

            let guess = game.evaluate_guess(input);
            let guess_output = game::format_guess(&guess);

            println!("\t\t{}", guess_output);

            if guess.iter().all(|x| matches!(x, Guess::Correct(_))) {
                break;
            }
        }
        game.next_word();
    }
    Ok(())
}
