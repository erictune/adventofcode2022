use std::collections::HashSet;

fn prio(c: char) -> i32 {
	match c {
		'a'..='z' => 1 + c as i32 - 'a' as i32,
		'A'..='Z' => 27 + c as i32 - 'A' as i32,
		_ => panic!("Unexpected character {}", c),
	}
}

#[test]
fn test_prio() {
    assert_eq!(prio('a'), 1);
    assert_eq!(prio('Z'), 52);

}

// quadratic but fine for this input size
fn shared_chars(s: &str, t: &str) -> Vec<char> {
    let mut shared = HashSet::new();
    for c in s.chars() {
        if let Some(_idx) = t.chars().find(|&x| x == c) {
            shared.insert(c);
        }
    }
    let mut ret = Vec::<char>::new();
    for c in shared {
        ret.push(c);
    }
ret
}

#[test]
fn test_shared_chars() {
    let shr = shared_chars("wMqvLMZHhHMvwLH", "jbvcjnnSBnvTQFn");
    assert_eq!(shr, vec!['v']);
}

pub fn do_day3_prob1(input: &str) -> i32 {
    let mut total = 0;
    for line in input.split("\n") {
        let l = line.len();
        if l == 0 { break; }
        let i = (l / 2) as usize;
        assert!(l == 2*i);
        let t = shared_chars(&line[0..i], &line[i..l]);
        assert!(t.len() == 1);
        let p = prio(t[0]);
        total += p;
    }
    total
}

#[test]
fn test_do_day3_prob1() {
    let test_input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    assert_eq!(do_day3_prob1(test_input), 157);
}

pub fn do_day3_prob2(input: &str) -> i32 {
    let mut total = 0;
    let mut lines = input.split("\n").peekable();
    while !lines.peek().is_none() {
        let a = lines.next().expect("Expected more lines");
        if a == "" { break; }  // Last carriage return may return empty string when split.
        let b = lines.next().expect("Expected more lines");
        let c = lines.next().expect("Expected more lines");
        let ab: String = shared_chars(&a, &b).iter().collect();
        let abc = shared_chars(&ab, &c);
        assert!(abc.len() == 1);
        let p = prio(abc[0]);
        total += p;
    }
    total
}

#[test]
fn test_do_day3_prob2() {
    let test_input = "\
vJrwpWtwJgWrhcsFMMfFFhFp
jqHRNqRjqzjGDLGLrsFMfFZSrLrFZsSL
PmmdzqPrVvPwwTWBwg
wMqvLMZHhHMvwLHjbvcjnnSBnvTQFn
ttgJtRGJQctTZtZT
CrZsJsPPZsGzwwsLwLmpwMDw
";
    assert_eq!(do_day3_prob2(test_input), 70);
}