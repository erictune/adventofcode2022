use std::env;
use std::fs;
use std::process;

// Run as:
// day6 [prob1|prob2] input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    if args.len() != 3 {
        println!("Usage: {}  `prob1'|`prob2' FILENAME", args[0]);
        process::exit(1);
    }

    let markerlen = match args[1].as_str() {
        "prob1" => 4,
        "prob2" => 14,
        _ => panic!("unspecified problem number"),
    };
    let file_path = &args[2];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let output = day6::do_day6(&input, markerlen);
    println!("{}", output)
}
