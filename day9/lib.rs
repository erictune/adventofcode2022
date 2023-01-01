use cgmath::point2;
use cgmath::Point2;
use cgmath::vec2;
use cgmath::Vector2;
use std::collections::HashSet;

/// Helper function to model rope segments as a trailing segment (t) which stays within an 8-neighbor of the leader (h).
/// Must call after ever single-step move of the head point. 
/// Panics if h is 3 or more steps away from h (Euclidean distance).
fn get_follower_move(h: Point2<i32>, t: Point2<i32>) -> Vector2<i32> {
    match t - h {
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
        _ => panic!("Tail unexpectedly far behind: t:{:?} h:{:?} t-h:{:?}", t, h, t-h)
    }
}

/// Solve day9 problem from Advent of Code 2022.
pub fn do_day9(input: &str, mode: i32) -> String {
    let ropelen = match mode {
        1 => 2,
        2 => 10,
        _ => panic!("Unknown mode"),
    };
    let tailidx = ropelen - 1;
    let mut segments = vec![point2(0,0); ropelen];
    let mut visited = HashSet::new();
    let mut saw_blank = false;

    println!("Tail starting at {:?}", segments[tailidx]);
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
            segments[0] += dir;
            // Compute new tail position for each following item.
            for i in 1..=tailidx {
                let old_follower_pos = segments[i];
                let follower_move = get_follower_move(segments[i-1], segments[i]);
                segments[i] += follower_move;
                if follower_move == (Vector2{x: 0, y:0}) {
                    println!("Leader[index={}] at {:?}, follower[index={}] remains at {:?}", i-1, segments[i-1], i, segments[i]);
                } else {
                    println!("Leader[index={}] at {:?}, follower[index={}] was at {:?}, snapped to {:?}", i-1, segments[i-1], i, old_follower_pos, segments[i]);
                }
            }
            println!("Tail visited {:?}", segments[tailidx]);
            visited.insert(segments[tailidx]);
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
