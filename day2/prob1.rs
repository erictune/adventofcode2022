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
    let output = do_day2_prob1(&input);
    println!("{}", output);
}

fn do_day2_prob1(input: & str) -> i32 {
    let mut totalscore = 0;
    for line in input.split("\n") {
        //dbg!(line.len());
        let parse_result = lib::get_two_chars(line);
        let (them, us) = match parse_result {
            Some(t) => (lib::abc_to_rps(t.0), lib::xyz_to_rps(t.1)),
            None => break,
        };
        dbg!(&them);
        dbg!(&us);
        let shapescore = match us {
            lib::Rps::Rock => 1,
            lib::Rps::Paper => 2,
            lib::Rps::Scissors => 3,
        };
        dbg!(shapescore);
        let result = lib::battle(&us, &them);
        dbg!(&result);
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
    totalscore

}

