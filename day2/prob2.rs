use std::env;
use std::fs;

mod lib;

// Run as:
// prob2 input.txt
// prob2 test_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let output = lib::do_day2_prob2(&input);
    println!("{}", output);
}
