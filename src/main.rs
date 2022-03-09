
mod game;
mod word;

mod standard_game;

use std::collections::HashSet;
use std::io::{self, Error, ErrorKind};
use crate::game::{Game, Guess};
use crate::word::Standard;

fn read_line() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
}

fn main() -> io::Result<()> {
    let mut words = Standard::new();
    let mut game = standard_game::G::new(words.get_word(4).ok_or(Error::new(ErrorKind::Other, "failed to get word"))?);

    loop {
        let letter_count = game.letter_count();
        println!("word size: {}", letter_count);
        let mut wrong_letters = HashSet::new();
        loop {
            let mut line = read_line()?;
            let mut input = line.trim_end();
            while input.len() != letter_count {
                println!("expected word size: {}", letter_count);
                line = read_line()?;
                input = line.trim_end();
            }

            // TODO need to reject user guesses which do not exist in the word list

            let guess = game.evaluate_guess(input);
            let guess_output = game::format_guess(&guess);

            for g in guess.iter() {
                match g { 
                    Guess::Wrong(letter) => { wrong_letters.insert(*letter); },
                    _ => { },
                }
            }

            let wrong_letter_output = wrong_letters.iter().map(|c| c.to_string()).collect::<Vec<_>>().join(" ");

            println!("\t\t{}", guess_output);

            println!("\n{}", wrong_letter_output);

            if guess.iter().all(|x| matches!(x, Guess::Correct(_))) {
                break;
            }
        }
        game.next_word();
    }
    Ok(())
}
