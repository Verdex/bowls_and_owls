
extern crate rwords;

use std::collections::HashSet;
use std::io;

mod game;
mod word;

use crate::game::Guess;
use crate::word::Standard;

fn read_line() -> io::Result<String> {
    let mut s = String::new();
    io::stdin().read_line(&mut s)?;
    Ok(s.trim_end().to_string())
}

fn main() -> io::Result<()> {
    let mut words = Standard::new();

    let mut letter_count = 4;

    loop {
        println!("word size: {}", letter_count);
        let answer = words.get_word(letter_count).expect("failed to get word");
        let mut wrong_letters = HashSet::new();
        loop {
            let mut line = read_line()?;
            let mut input = line.trim_end();
            loop {
                
                if input.len() != letter_count {
                    println!("expected word size: {}", letter_count);
                }
                else if !words.check_word_exists(input) {
                    println!("word not found in word list");
                }
                else {
                    break;
                }

                line = read_line()?;
                input = line.trim_end();
            }

            let guess = game::evaluate_guess(answer, input);
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
        letter_count += 1;
    }
    Ok(())
}
