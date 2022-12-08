use std::fs;
use std::env;


// Run as:
// prob1 input.txt 
// prob1 test_input.txt
fn main() {
    let args: Vec<String> = env::args().collect();
    //dbg!(args);
    let file_path = &args[1];
    let input = fs::read_to_string(file_path).expect("Should have been able to read the file");
    let mut elf = 1;
    let mut totalcal: i32 = 0;
    let mut maxcal: i32 = 0; 

    for found in input.split("\n"){
      if found == "" {
          elf += 1;
          totalcal = 0;
      } else {
          totalcal += found.parse::<i32>().unwrap();
          if totalcal > maxcal { maxcal = totalcal }
          //println!("{} {} {}", found, elf, totalcal);
      }
    }

    println!("elf {} has {} calories", elf, maxcal);
}
