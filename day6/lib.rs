use std::collections::HashSet;

// Return the character index of the last character of the marker (first k all-different chars).
fn find_marker(s: &str, k: usize) -> Option<usize> {
    assert!(k > 1);
    // Move window [starti..=endi] over char string finding first unique set of chars.
    let mut endi = k.checked_sub(1).expect("k must be greater than 0");
    let mut starti = 0;
    let mut shared = HashSet::new();
    loop {
        if endi >= s.len() {
            return None; // Window reached end, no match.
        }
        shared.extend(s[starti..=endi].chars());
        if shared.len() == k {
            dbg!(shared);
            return Some(endi);
        }
        shared.clear();
        starti += 1;
        endi += 1;
    }
}

#[test]
fn test_find_marker() {
    let cases = vec![
        ("zcf", 4, None, 1),                            // Not long enough
        ("zcfzcfzcfzcfzcfzcfzcfzcfzcfzcf", 4, None, 2), // Not 4 unique chars.
        //01234567890123456789012345678901
        ("zcfzcfxasbcd", 4, Some(6), 3), // When last window char is pointing to index 6, [6-3..6] has 4 unique values.
        ("zcfzcfzcfzcfzcfzcfzcfzcfzcfzcfab", 4, Some(30), 4),
        ("00000000001234567890000000000000", 10, Some(18), 5),
    ];
    for (s, k, expected, casenum) in cases {
        assert_eq!(find_marker(s, k), expected, "Failed test case {}", casenum);
    }
}

/// Solve day6 problem from Advent of Code 2022.
pub fn do_day6(input: &str, markerlen: usize) -> String {
    match find_marker(input, markerlen) {
        Some(n) => format!("{}", n + 1), // find_marker uses 0-based, problem uses 1-based indexing.
        _ => panic!("No match found"),
    }
}

#[test]
fn test_do_day6_with_prob1_test_inputs() {
    let cases = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7", 1), // First example given at https://adventofcode.com/2022/day/6
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5", 2),   // first marker after character 5
        ("nppdvjthqldpwncqszvftbrmjlhg", "6", 3),   // first marker after character 6
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10", 4), // first marker after character 10
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11", 5), // first marker after character 11
                                                    //012345678901234567890
    ];
    for (input, expected, casenum) in cases {
        assert_eq!(do_day6(input, 4), expected, "in case number {}", casenum);
    }
}

#[test]
fn test_do_day6_with_prob2_test_inputs() {
    let cases = vec![
        ("mjqjpqmgbljsphdztnvjfqwrcgsmlb", "7", 1), // First example given at https://adventofcode.com/2022/day/6
        ("bvwbjplbgvbhsrlpgdmjqwftvncz", "5", 2),   // first marker after character 5
        ("nppdvjthqldpwncqszvftbrmjlhg", "6", 3),   // first marker after character 6
        ("nznrnfrfntjfmvfwmzdfjlvtqnbhcprsg", "10", 4), // first marker after character 10
        ("zcfzfwzzqfrljwzlrfnpqdbhtmscgvjw", "11", 5), // first marker after character 11
                                                    //012345678901234567890
    ];
    for (input, expected, casenum) in cases {
        assert_eq!(do_day6(input, 4), expected, "in case number {}", casenum);
    }
}
