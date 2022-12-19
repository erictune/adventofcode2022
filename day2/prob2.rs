use std::env;
use std::fs;

mod lib;

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
        let parse_result = lib::get_two_chars(line);
        let them = match parse_result {
            Some(t) => lib::abc_to_rps(t.0),
            None => break,
        };
        let result = lib::xyz_to_wld(parse_result.unwrap().1);
        let us = lib::infer_our_move(&them, &result);
        dbg!(&them);
        dbg!(&us);
        dbg!(&result);
        let shapescore = match us {
            lib::Rps::Rock => 1,
            lib::Rps::Paper => 2,
            lib::Rps::Scissors => 3,
        };
        dbg!(shapescore);
        let winscore = match result {
            lib::Wld::Win => 6,
            lib::Wld::Draw => 3,
            lib::Wld::Lose => 0,
        };
        dbg!(winscore);
        totalscore += shapescore;
        totalscore += winscore;
        dbg!(totalscore);
        dbg!("----");
    }
    println!("{}", totalscore)
}

