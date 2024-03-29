
#[derive(Debug, PartialEq, Eq, Clone, Copy)]
pub enum Guess { 
    Correct(char),
    Present(char),
    Wrong(char),
}

fn transpose<T : Copy>(m : &Vec<Vec<T>>) -> Vec<Vec<T>> {
    if m.len() == 0 { 
        return vec![];
    }

    let l = m[0].len();

    let mut ret = vec![];
    for c in 0..l {
        let mut col = vec![];
        for r in 0..m.len() {
            col.push(m[r][c]);
        }
        ret.push(col);
    }

    ret
}

fn score_wrong(history : &Vec<Vec<Guess>>) -> f32 {
    use std::collections::HashSet;
    let mut wrong = HashSet::new();
    let mut total = 0.0;
    let mut index = 1;
    for row in history {
        for w in row.iter().filter(|x| match x { Guess::Wrong(_) => true, _ => false})
                           .map(|x| match x { Guess::Wrong(x) => x, _ => unreachable!(), }) {
            if wrong.insert(w) {
                total += (1f32 / 26.0) * 0.5f32.powi( index );
            }
        }
        index += 1;
    }
    total
}

fn score_present(columns : &Vec<Vec<Guess>>) -> f32 {
    let presents = columns.iter().map(|col| col.iter() 
                                               .enumerate()
                                               .filter(|(_, g)| match g { Guess::Present(_) => true, _ => false })
                                               .map(|(i, g)| match g { Guess::Present(c) => (i, c), _ => unreachable!() })
                                               .collect::<Vec<_>>());

    let mut tot = 0f32;
    for mut p in presents {
        p.sort_by_key(|(_, c)| *c);
        p.dedup_by_key(|(_, c)| *c);
        tot += p.iter().map(|(i, _)| (1f32 / 26f32) * (1f32 / 5f32) * 0.5f32.powi( *i as i32 ) ).sum::<f32>();
    }

    tot
}

pub fn score_history(history : &Vec<Vec<Guess>>) -> f32 {
    if history.len() == 0 {
        return 0.0;
    }

    let word_size = history[0].len() as f32;

    let columns = transpose(history);

    let correct_score : f32 
        = columns.iter().map(|col| col.iter()
                                      .enumerate()
                                      .find(|(_, g)| match g { Guess::Correct(_) => true, _ => false }))
                        .map(|x| x.map_or(0.0, |(i, _)| (1.0 / word_size) * 0.5f32.powi((i + 1) as i32)))
                        .sum();
    let wrong_score = score_wrong(&history);
    let present_score = score_present(&columns);

    wrong_score + correct_score + present_score
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
    fn should_transpose() {
        let m = vec![ vec![1, 2, 3], vec![4, 5, 6] ];
        let o = transpose(&m);
        assert_eq!( o.len(), 3 );
        assert_eq!( o[0].len(), 2 );
        assert_eq!( o[0][0], 1 );
        assert_eq!( o[0][1], 4 );
        assert_eq!( o[1][0], 2 );
        assert_eq!( o[1][1], 5 );
        assert_eq!( o[2][0], 3 );
        assert_eq!( o[2][1], 6 );
    }

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

    #[test]
    fn evaluate_guess_should_correctly_handle_correct_non_matching_repeat_letters() {
        use Guess::*;
        let guess = evaluate_guess("MOXAS", "MOSSY");

        assert_eq!( guess, [Correct('M'), Correct('O'), Present('S'), Wrong('S'), Wrong('Y')] );
    }
}