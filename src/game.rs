
#[derive(Debug, PartialEq, Eq)]
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

pub fn evaluate_guess(answer : &str, user_guess : &str) -> Vec<Guess> {

    if answer.len() != user_guess.len() {
        panic!("evaluate guess must verify that answer and user_guess are the same length");
    }

    let answer = answer.chars().collect::<Vec<char>>();
    let user_guess = user_guess.chars().collect::<Vec<char>>();
    let mut result = user_guess.iter().map(|x| Guess::Wrong(*x)).collect::<Vec<Guess>>();

    let mut present = vec![];
    for i in 0..answer.len() {
        if answer[i] == user_guess[i] {
            result[i] = Guess::Correct(user_guess[i])
        }
        else {
            present.push(answer[i]);
        }
    }

    for i in 0..answer.len() {
        if answer[i] != user_guess[i] && present.contains(&user_guess[i]) {
            let (index, _) = present.iter().enumerate().find(|(_, v)| **v == user_guess[i]).unwrap();
            present.remove(index);
            result[i] = Guess::Present(user_guess[i])
        }
    }

    result
}


#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn should_evaluate_wrong_guess() {
        let guess = evaluate_guess("RRBB", "XXXX");

        assert!( matches!(&guess[..], [Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_one_correct() {
        let guess = evaluate_guess("XAAA", "XBBB");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_two_correct() {
        let guess = evaluate_guess("XAAY", "XBBY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_three_correct() {
        let guess = evaluate_guess("XAZY", "XBZY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Wrong(_), Guess::Correct(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_four_correct() {
        let guess = evaluate_guess("XWZY", "XWZY");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Correct(_), Guess::Correct(_)]) );
    }

    #[test]
    fn should_evaluate_one_present() {
        let guess = evaluate_guess("ZAZZ", "XXXA");

        assert!( matches!(&guess[..], [Guess::Wrong(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_two_present() {
        let guess = evaluate_guess("ZAZB", "BXXA");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Wrong(_), Guess::Wrong(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_three_present() {
        let guess = evaluate_guess("CAZB", "BCXA");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Present(_), Guess::Wrong(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_four_present() {
        let guess = evaluate_guess("CADB", "BCAD");

        assert!( matches!(&guess[..], [Guess::Present(_), Guess::Present(_), Guess::Present(_), Guess::Present(_)]) );
    }

    #[test]
    fn should_evaluate_with_no_present_for_extra() {
        let guess = evaluate_guess("RRBB", "RRRX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Wrong(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_with_present_for_extra() {
        let guess = evaluate_guess("RRBR", "RRRX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Present(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_with_present_and_correct() {
        let guess = evaluate_guess("ABDB", "ABBX");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Present(_), Guess::Wrong(_)]) );
    }

    #[test]
    fn should_evaluate_with_correct_but_no_present_for_consumed_correct() {
        let guess = evaluate_guess("RRBB", "RRRB");

        assert!( matches!(&guess[..], [Guess::Correct(_), Guess::Correct(_), Guess::Wrong(_), Guess::Correct(_)]) );
    }
}