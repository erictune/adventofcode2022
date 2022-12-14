/// Return a reference to the last item in a vector.
fn last<T>(v: &Vec<T>) -> &T {
    let lastidx: usize = v.len() - 1;
    &v[lastidx]
}

#[test]
fn last_returns_last_item() {
    let v: Vec<i32> = vec![2, 4, 5, 8];
    assert_eq!(*last(&v), 8);
}

/// Solve day5 problem from Advent of Code 2022.
/// reversed: true for part1, false for part 2 - controls whether move reverses order of chars moved.
pub fn do_day5(input: &str, reversed: bool) -> String {
    let mut stacks: Vec<Vec<char>> = Vec::new();
    // Assume width of first line determines number of columns (stacks). 4 chars per stack.
    let mut lines = input.split("\n").peekable();
    let firstlineifsome = lines.peek();
    let firstline = firstlineifsome.expect("expected first line present");
    let firstlinelen = firstline.len();
    let ncol = (firstlinelen as f64 / 4.0).ceil() as usize;
    dbg!(&ncol);
    for _i in 0..ncol {
        stacks.push(vec!['_']);
        stacks[_i].pop(); // How do i push an empty vector?
    }
    dbg!(&stacks);
    while let Some(line) = lines.next() {
        dbg!(&line);
        if line.find("[").is_none() {
            break;
        }
        let linechars: Vec<char> = line.chars().collect();
        for i in 0..ncol {
            let c = linechars[4 * i + 1];
            match c {
                'A'..='Z' => stacks[i].push(c),
                ' ' => (),
                _ => panic!("Unexpected character {}", c),
            }
        }
        dbg!(&stacks);
        // reverse the stacks so that the last thing in is at the bottom of the stack.
    }
    for i in 0..ncol {
        stacks[i].reverse()
    }
    dbg!(&stacks);

    // We are on line with 1 2 3 ...; now skip the blank line.
    lines.next();

    // Read move lines and do moves
    for line in lines {
        dbg!(line);
        if line == "" {
            break;
        }
        let words: Vec<&str> = line.split(" ").collect();
        assert!(words.len() == 6);
        assert!(words[0] == "move");
        let nmoves = words[1].parse::<usize>().unwrap();
        assert!(words[2] == "from");
        let srcidx = words[3].parse::<usize>().unwrap() - 1;
        assert!(words[4] == "to");
        let dstidx = words[5].parse::<usize>().unwrap() - 1;

        let i = stacks[srcidx].len() - nmoves;
        let mut tmp: Vec<char> = stacks[srcidx].drain(i..).collect();
        // Need tmp Vec since cannot convince Rust that the two mutable references to stacks are not aliased
        // (and they might be if the indexes are the same).
        if reversed {
            tmp.reverse();
        }
        stacks[dstidx].extend(tmp);
    }
    dbg!(&stacks);
    let mut output = String::new();
    for i in 0..ncol {
        let tmp = format!("{}", last(&stacks[i]));
        output.push_str(&tmp);
    }
    output.to_string()
}

#[test]
fn do_prob1_with_test_input() {
    let test_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    assert_eq!(do_day5(test_input, true), "CMZ");
}

#[test]
fn do_prob2_with_test_input() {
    let test_input = r#"    [D]    
[N] [C]    
[Z] [M] [P]
 1   2   3 

move 1 from 2 to 1
move 3 from 1 to 3
move 2 from 2 to 1
move 1 from 1 to 2
"#;
    assert_eq!(do_day5(test_input, false), "MCD");
}
