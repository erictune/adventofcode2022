use std::env;
use std::fs;

// Run as:
// prob1 input.txt
// prob1 test_input.txt
fn main() {
    const TOPN: usize = 1;
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    
    let mut vec = Vec::<i32>::new();  
    let mut totalcal: i32 = 0;

    for found in input.split("\n") {
        if found == "" {
            vec.push(totalcal);
            totalcal = 0;
        } else {
            totalcal += found.parse::<i32>().unwrap();
        }
    }
    // Print calories per elf descending
    vec.sort();
    vec.reverse();
    //println!("{:?}", &vec[0..=2]);
    let vec2: Vec<i32> = vec[0..=TOPN].to_vec();
    // Calories of top n elves.
    println!("{}", vec2.iter().sum::<i32>());
}
