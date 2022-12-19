#[derive(Debug, PartialEq)]
enum Rps {
    Rock,
    Paper,
    Scissors,
}

fn abc_to_rps(c: char) -> Rps {
    match c {
        'A' => Rps::Rock,
        'B' => Rps::Paper,
        'C' => Rps::Scissors,
        _ => panic!("Wrong character: {}", c),
    }
}

#[test]
fn test_abc_to_rps() {
    assert_eq!(abc_to_rps('A'), Rps::Rock);
    assert_eq!(abc_to_rps('B'), Rps::Paper);
    assert_eq!(abc_to_rps('C'), Rps::Scissors);
}

#[derive(Debug, PartialEq)]
enum Wld {
    Win,
    Lose,
    Draw,
}

fn xyz_to_rps(c: char) -> Rps {
    match c {
        'X' => Rps::Rock,
        'Y' => Rps::Paper,
        'Z' => Rps::Scissors,
        _ => panic!("Wrong character: {}", c),
    }
}

#[test]
fn test_xyz_to_rps() {
    assert_eq!(xyz_to_rps('X'), Rps::Rock);
    assert_eq!(xyz_to_rps('Y'), Rps::Paper);
    assert_eq!(xyz_to_rps('Z'), Rps::Scissors);
}

// Returns whether first argument won, lost, or it was a draw.
fn battle(them: &Rps, us: &Rps) -> Wld {
    match (them, us) {
        (Rps::Rock, Rps::Rock) | (Rps::Paper, Rps::Paper) | (Rps::Scissors, Rps::Scissors) => {
            Wld::Draw
        }
        (Rps::Paper, Rps::Rock) | (Rps::Scissors, Rps::Paper) | (Rps::Rock, Rps::Scissors) => {
            Wld::Win
        }
        _ => Wld::Lose,
    }
}

#[test]
fn test_battle() {
    assert_eq!(battle(&Rps::Rock, &Rps::Rock), Wld::Draw); // Rock vs Rock is draw.
    assert_eq!(battle(&Rps::Paper, &Rps::Rock), Wld::Win); // Paper beats Rock.
    assert_eq!(battle(&Rps::Scissors, &Rps::Rock), Wld::Lose); // Scissors lose to Rock.
}
fn xyz_to_wld(c: char) -> Wld {
    match c {
        'X' => Wld::Lose,
        'Y' => Wld::Draw,
        'Z' => Wld::Win,
        _ => panic!("Wrong character: {}", c),
    }
}

// If they move "them" and we want to us to get "outcome", then what should we play?
fn infer_our_move(them: &Rps, outcome: &Wld) -> Rps {
    match (them, outcome) {
        (Rps::Rock, Wld::Draw) => Rps::Rock,
        (Rps::Paper, Wld::Draw) => Rps::Paper,
        (Rps::Scissors, Wld::Draw) => Rps::Scissors,
        (Rps::Rock, Wld::Win) => Rps::Paper,
        (Rps::Paper, Wld::Win) => Rps::Scissors,
        (Rps::Scissors, Wld::Win) => Rps::Rock,
        (Rps::Rock, Wld::Lose) => Rps::Scissors,
        (Rps::Paper, Wld::Lose) => Rps::Rock,
        (Rps::Scissors, Wld::Lose) => Rps::Paper,
    }
}

#[test]
fn test_infer_our_move() {
    assert_eq!(infer_our_move(&Rps::Rock, &Wld::Draw), Rps::Rock); // If they play Rock, and we want to Draw, we play Rock.
}

fn get_two_chars(s: &str) -> Option<(char, char)> {
    match s.len() {
        0 => return None,
        3 => return Some((s.chars().nth(0).unwrap(), s.chars().nth(2).unwrap())),
        _ => panic!("Malformed line"),
    }
}

#[test]
fn test_get_two_chars() {
    assert_eq!(get_two_chars(""), None); // Blank line
                                         // assert_eq!(get_two_chars("IJK"), None); // Not handled well
    assert_eq!(get_two_chars("I J"), Some(('I', 'J'))); // Good line
}

#[test]
#[should_panic]
fn test_get_two_chars_panics_on_one_char() {
    assert_eq!(get_two_chars("I"), None);
}

#[test]
#[should_panic]
fn test_get_two_chars_panics_on_two_chars() {
    assert_eq!(get_two_chars("I "), None);
}

#[test]
#[should_panic]
fn test_get_two_chars_panics_on_four_chars() {
    assert_eq!(get_two_chars("I J "), None);
}

pub fn do_day2_prob1(input: &str) -> i32 {
    let mut totalscore = 0;
    for line in input.split("\n") {
        //dbg!(line.len());
        let parse_result = get_two_chars(line);
        let (them, us) = match parse_result {
            Some(t) => (abc_to_rps(t.0), xyz_to_rps(t.1)),
            None => break,
        };
        dbg!(&them);
        dbg!(&us);
        let shapescore = match us {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };
        dbg!(shapescore);
        let result = battle(&us, &them);
        dbg!(&result);
        let winscore = match result {
            Wld::Win => 6,
            Wld::Draw => 3,
            Wld::Lose => 0,
        };
        dbg!(winscore);
        totalscore += shapescore;
        totalscore += winscore;
        dbg!(totalscore);
        dbg!("----");
    }
    totalscore
}

#[test]
fn test_do_day2_prob1() {
    let test_input = "\
A Y
B X
C Z
";
    let output = do_day2_prob1(&test_input);
    assert_eq!(output, 15);
}

pub fn do_day2_prob2(input: &str) -> i32 {
    let mut totalscore = 0;
    for line in input.split("\n") {
        //dbg!(line.len());
        let parse_result = get_two_chars(line);
        let them = match parse_result {
            Some(t) => abc_to_rps(t.0),
            None => break,
        };
        let result = xyz_to_wld(parse_result.unwrap().1);
        let us = infer_our_move(&them, &result);
        dbg!(&them);
        dbg!(&us);
        dbg!(&result);
        let shapescore = match us {
            Rps::Rock => 1,
            Rps::Paper => 2,
            Rps::Scissors => 3,
        };
        dbg!(shapescore);
        let winscore = match result {
            Wld::Win => 6,
            Wld::Draw => 3,
            Wld::Lose => 0,
        };
        dbg!(winscore);
        totalscore += shapescore;
        totalscore += winscore;
        dbg!(totalscore);
        dbg!("----");
    }
    totalscore
}

#[test]
fn test_do_day2_prob2() {
    let test_input = "\
A Y
B X
C Z
";
    let output = do_day2_prob2(&test_input);
    assert_eq!(output, 12);
}
