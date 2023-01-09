/// Solve day11 problem from Advent of Code 2022.
mod monkeys;
use crate::monkeys::Monkeys;
//use crate::monkeys::Monkey;

pub fn do_day11(input: &str, mode: i32) -> String {
    let iters = match mode {
        1 => 20,
        2 => 10_000,
        _ => panic!("Unknown mode"),
    };
    let mut m = Monkeys::new_from_file(input);

    // Run the program 20 steps.
    for _ in 1..=iters {
        print!("{}", m.pretty());
        m.do_round();
    }
    return String::from(format!("{}", m.monkey_business()));
}

#[cfg(test)]
const AOC_TEST_INPUT: &str = "\
Monkey 0:
  Starting items: 79, 98
  Operation: new = old * 19
  Test: divisible by 23
    If true: throw to monkey 2
    If false: throw to monkey 3

Monkey 1:
  Starting items: 54, 65, 75, 74
  Operation: new = old + 6
  Test: divisible by 19
    If true: throw to monkey 2
    If false: throw to monkey 0

Monkey 2:
  Starting items: 79, 60, 97
  Operation: new = old * old
  Test: divisible by 13
    If true: throw to monkey 1
    If false: throw to monkey 3

Monkey 3:
  Starting items: 74
  Operation: new = old + 3
  Test: divisible by 17
    If true: throw to monkey 0
    If false: throw to monkey 1
";

#[cfg(test)]
const AOC_TEST_MONKEY_BUSINESS_LEVEL: u32 = 10605;

// Test the example from AoC Day 11 including parsing input.
#[test]
fn test_do_day11_prob1_test_input() {
    let expected = format!("{}", AOC_TEST_MONKEY_BUSINESS_LEVEL);
    let actual = do_day11(AOC_TEST_INPUT, 1);
    assert_eq!(expected, actual);
}

#[test]
fn test_do_day11_prob2_test_input() {
    //let actual = do_day11(AOC_TEST_INPUT, 2);
    //let expected = String::from("blah");
    //println!("actual:\n{}", actual);
    //println!("expected:\n{}", expected);

    //assert_eq!(expected, actual);
}
