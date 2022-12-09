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
	let (them, us) = match parse_result {
		Some(t) => (abc_to_rps(t.0), xyz_to_rps(t.1)),
		None => break
	};
  	dbg!(&them);
  	dbg!(&us);
	let shapescore = match us {
		Rps::Rock => 1,
		Rps::Paper => 2,
		Rps::Scissors => 3
	};
	dbg!(shapescore);
	let result = battle(&us, &them);
	dbg!(&result);
	let winscore = match result {
		Wld::Win => 6,
		Wld::Draw => 3,
		Wld::Lose => 0
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
		_ => panic!("Malformed line")
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
       	_ => panic!("Wrong character: {}", c)
    }
}
fn xyz_to_rps(c: char) -> Rps {
    match c {
        'X' => Rps::Rock,
        'Y' => Rps::Paper,
        'Z' => Rps::Scissors,
       	_ => panic!("Wrong character: {}", c)
    }
}

#[derive(Debug)]
enum Wld {
    Win,
    Lose,
    Draw,
}

// Returns whether first argument won, lost, or it was a draw.
fn battle(them: &Rps, us: &Rps) -> Wld {
    match (them, us) {
        (Rps::Rock, Rps::Rock) |
        (Rps::Paper, Rps::Paper) |
        (Rps::Scissors, Rps::Scissors) => Wld::Draw,
        (Rps::Paper, Rps::Rock) |
        (Rps::Scissors, Rps::Paper) |
        (Rps::Rock, Rps::Scissors) => Wld::Win,
	_ => Wld::Lose,
    }
}
