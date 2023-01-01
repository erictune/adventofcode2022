use cgmath::point2;
use cgmath::vec2;
use cgmath::Vector2;
use std::collections::HashSet;

/// Solve day9 problem from Advent of Code 2022.
pub fn do_day9(input: &str, mode: i32) -> String {
    if mode != 1 && mode != 2 {
        unimplemented!("Unknown mode");
    }
    
    let mut h = point2(0, 0);
    let mut t = point2(0, 0);
    let mut visited = HashSet::new();
    let mut saw_blank = false;

    println!("Tail starting at {:?}", t);
    for line in input.split("\n") {
        let lineparts: Vec<&str> = line.split(" ").collect();
        if line.len() == 0 {
            saw_blank = true;
            continue;
        }
        if saw_blank {
            panic!("Did not expect non-final blank line.")
        }
        if lineparts.len() != 2 {
            panic!("Too few tokens on line: {}", line);
        }
        let dir = match lineparts[0] {
            "R" => vec2(1, 0),
            "L" => vec2(-1, 0),
            "U" => vec2(0, 1),
            "D" => vec2(0, -1),
            _ => panic!(),
        };
        let steps_in_dir = lineparts[1].parse::<u8>().expect(format!("Should have parsed number of steps: {}", lineparts[1]).as_str());
        for _ in 0..steps_in_dir {
            // Move head.
            h += dir;
            // Compute new tail position.
            let tail_offset = t - h;

            let mvt = match tail_offset {
                // Tail is on top of head => no need to move it.
                Vector2{x: 0, y: 0} => vec2(0, 0),
                // Tail is an 8-neigbor of head => no need to move it.
                Vector2{x: 1, y:0} |
                Vector2{x: -1, y:0 } |
                Vector2{x: 0, y:1} |
                Vector2{x: 0, y:-1 } |
                Vector2{x: 1, y:1} |
                Vector2{x:  -1, y:1 } |
                Vector2{x: -1, y: -1} |
                Vector2{x:  1, y:-1 } => vec2(0, 0),
                // Tail is 2 steps behind in a cardinal direction => move 1 closer.
                Vector2{x: -2, y:0} => vec2(1, 0),
                Vector2{x: 2, y:0} => vec2(-1, 0),
                Vector2{x: 0, y:-2} => vec2(0, 1),
                Vector2{x: 0, y:2} => vec2(0,-1),
                // Tail is 2 steps back and over one. => move diagonally closer.
                Vector2{x: -2, y:-1} => vec2(1, 1),
                Vector2{x: -1, y:-2} => vec2(1, 1),
                Vector2{x: -2, y:1} => vec2(1, -1),
                Vector2{x: -1, y:2} => vec2(1, -1),
                Vector2{x: 2, y:-1} => vec2(-1, 1),
                Vector2{x: 1, y:-2} => vec2(-1, 1),
                Vector2{x: 2, y:1} => vec2(-1, -1),
                Vector2{x: 1, y:2} => vec2(-1, -1),
                _ => panic!("Tail unexpectedly far behind: {:?}", tail_offset)
            };
            let newt = t + mvt;
            if mvt == (Vector2{x: 0, y:0}) {
                println!("Head at {:?}, tail remains at {:?}", h, t);
            } else {
                println!("Head at {:?}, tail was at {:?}, snapped to  {:?}", h, t, newt);
            }
            t = newt;
            println!("Tail visited {:?}", t);
            visited.insert(t);
        }
    }

    // TODO: Visualize the visited graph in case it is some kind of easter egg.

    return String::from(format!("{}", visited.iter().count()));
}

#[cfg(test)]
const AOC_TEST_INPUT: &str = "\
R 4
U 4
L 3
D 1
R 4
D 1
L 5
R 2";

#[test]
fn test_do_day9_prob1_test_input() {
    let expected = format!("{}", 13);
    let actual = do_day9(AOC_TEST_INPUT, 1);
    assert_eq!(String::from(expected), actual);
}

//#[test]
//fn test_do_day8_prob2_test_input() {
//    let expected = format!("{}", 8);
//    let actual = do_day8(ADVENT_TEST_INPUT, 2);
//    assert_eq!(String::from(expected), actual);
//}
