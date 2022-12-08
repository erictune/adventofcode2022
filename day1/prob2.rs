use std::fs;
use std::env;

// Run as:
// prob2 input.txt
// prob2 test_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");

    let mut vec = Vec::<i32>::new();
    let mut _elf = 1;
    let mut totalcal: i32 = 0;

    for found in input.split("\n") {
        if found == "" {
            _elf += 1;
            vec.push(totalcal);
            totalcal = 0;
        } else {
            totalcal += found.parse::<i32>().unwrap();
        }
    }
    vec.sort();
    vec.reverse();
    //println!("{:?}", &vec[0..=2]);
    let vec2: Vec<i32> = vec[0..=2].to_vec();
    //println!("{:#?}", &vec2);
    println!("{}", vec2.iter().sum::<i32>());
}
