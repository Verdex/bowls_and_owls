
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

    let a = answer.chars().collect::<Vec<char>>();
    let u = user_guess.chars().collect::<Vec<char>>();
    let mut r = u.iter().map(|x| Guess::Wrong(*x)).collect::<Vec<Guess>>();

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