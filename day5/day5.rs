use std::env;
use std::fs;
use std::process;

mod lib;

// Run as:
// day5 [prob1|prob2] input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {}  `prob1'|`prob2' FILENAME", args[0]);
        process::exit(1);
    }

    let reversed = match args[1].as_str() {
        "prob1" => Some(true),
        "prob2" => Some(false),
        _ => None
    }.expect("Command name must be prob1 or prob2");
    let file_path = &args[2];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let output = lib::do_day5(&input, reversed);
    println!("{}", output)
}