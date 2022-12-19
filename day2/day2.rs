use std::env;
use std::fs;
use std::process;

mod lib;

// Run as:
// day2 [prob1|prob2] input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {}  `prob1'|`prob2' FILENAME", args[0]);
        process::exit(1);
    }
    let probnum = match args[1].as_str() {
        "prob1" => Some(1),
        "prob2" => Some(2),
        _ => None,
    }
    .expect("Usage: <arg0>  `prob1'|`prob2' FILENAME");
    let file_path = &args[2];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let output = match probnum {
        1 => lib::do_day2_prob1(&input),
        2 => lib::do_day2_prob2(&input),
        _ => panic!("invalid problem number"),
    };
    println!("{}", output)
}
