/// Return the top n elf calorie counts.
pub fn do_day1(input: &str, topn: usize) -> i32 {
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
    let vec2: Vec<i32> = vec[0..topn].to_vec();
    // Calories of top n elves.
    vec2.iter().sum::<i32>()
}


#[test]
fn test_do_day1() {
    let test_input = "\
1000
2000
3000

4000

5000
6000

7000
8000
9000

10000
";
    assert_eq!(do_day1(test_input, 1), 24000);
    assert_eq!(do_day1(test_input, 3), 45000);
}

