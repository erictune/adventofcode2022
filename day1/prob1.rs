fn main() {
    // Statements here are executed when the compiled binary is called
    let input = String::from("1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
");

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
