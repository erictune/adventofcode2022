use std::env;
use std::fs;

// Run as:
// prob1 input.txt
// prob1 test_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

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
    println!("{}", totalscore)
}

fn get_two_chars(s: &str) -> Option<(char, char)> {
    match s.len() {
        0 => return None,
        3 => return Some((s.chars().nth(0).unwrap(), s.chars().nth(2).unwrap())),
        _ => panic!("Malformed line"),
    }
}

#[derive(Debug)]
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

fn xyz_to_wld(c: char) -> Wld {
    match c {
        'X' => Wld::Lose,
        'Y' => Wld::Draw,
        'Z' => Wld::Win,
        _ => panic!("Wrong character: {}", c),
    }
}

#[derive(Debug)]
enum Wld {
    Win,
    Lose,
    Draw,
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
