use day8::do_day8;

#[cfg(test)]
const ADVENT_TEST_INPUT: &str = "\
30373
25512
65332
33549
35390
";

// Test `do_day8` with the full problem 1 input, as a regression test.
#[test]
fn test_do_day8_prob1_test_input() {
    let my_advent_input_bytes = include_bytes!("input.txt");
    let my_advent_input_str = String::from_utf8(my_advent_input_bytes.to_vec())
            .expect("Should have converted input.txt to string");

    let expected = format!("{}", 1736);
    let actual = do_day8(&my_advent_input_str.as_str(), 1);
    assert_eq!(String::from(expected), actual);
}
