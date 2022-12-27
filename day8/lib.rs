mod loadgrid;
use crate::loadgrid::digit_text_to_u8_grid;
use crate::loadgrid::display_grid;

/// Solve day8 problem from Advent of Code 2022.
pub fn do_day8(input: &str, mode: i32) -> String {
    match mode {
        1 => (),
        _ => unimplemented!(),
    };
    let (rows, cols, grid) = digit_text_to_u8_grid(input).unwrap();
    let mut visible: Vec<Vec<u8>> = (0..rows).map(|_| vec![0; cols]).collect();
    // Edge trees are going to be visible because we will start the with max-height-seen value of -1.
    const INITIAL_MAX: i32 = -1;
    // for c in 1..=cols-1 {
    //     grid[0][c] = 1;
    //     grid[cols-1][c] = 1;
    // }
    // for r in 1..=rows-1 {
    //     grid[r][0] = 1;
    //     grid[r][cols-1] = 1;
    // }

    println!("from north to south");
    // Looking south from the north side, checking full column at a time.
    for c in 0..=cols - 1 {
        let mut max = INITIAL_MAX;
        for r in 0..=rows - 1 {
            if grid[r][c] as i32 > max {
                max = grid[r][c] as i32;
                visible[r][c] = 1;
            }
        }
    }
    println!("{}", display_grid(rows, cols, &grid));
    println!("{}", display_grid(rows, cols, &visible));

    println!("from south");
    // Looking north from the south side, checking full column at a time.
    // Skip the first and last column, as they are visible by problem definition.
    for c in 0..=cols - 1 {
        let mut max = INITIAL_MAX; // The first item is the highest seen so far.
                                   // Skip the first row and last row, as they are visible by problem definition.
        for r in (0..=rows - 1).rev() {
            if grid[r][c] as i32 > max {
                max = grid[r][c] as i32;
                visible[r][c] = 1;
            }
        }
    }
    println!("{}", display_grid(rows, cols, &grid));
    println!("{}", display_grid(rows, cols, &visible));

    println!("from west");
    // Looking east from the west side, checking full row at a time.
    // Skip the first and last row, as they are visible by problem definition.
    for r in 0..=rows - 1 {
        let mut max = INITIAL_MAX; // The first item is the highest seen so far.
                                   // Skip the first row and last row, as they are visible by problem definition.
        for c in 0..=cols - 1 {
            if grid[r][c] as i32 > max {
                max = grid[r][c] as i32;
                visible[r][c] = 1;
            }
        }
    }
    println!("{}", display_grid(rows, cols, &grid));
    println!("{}", display_grid(rows, cols, &visible));

    println!("from east");
    // Looking west from the east side, checking full row at a time.
    // Skip the first and last row, as they are visible by problem definition.
    for r in 0..=rows - 1 {
        let mut max = INITIAL_MAX; // The first item is the highest seen so far.
                                   // Skip the first row and last row, as they are visible by problem definition.
        for c in (0..=cols - 1).rev() {
            if grid[r][c] as i32 > max {
                max = grid[r][c] as i32;
                visible[r][c] = 1;
            }
        }
    }
    println!("{}", display_grid(rows, cols, &grid));
    println!("{}", display_grid(rows, cols, &visible));

    // The problem with using a u8 is that you can't use sum()
    let num_vis: u64 = visible
        .iter()
        .map(|v| v.iter().fold(0u64, |sum, n| sum + (*n as u64)))
        .sum();
    format!("{}", num_vis)
}

#[cfg(test)]
const ADVENT_TEST_INPUT: &str = "\
30373
25512
65332
33549
35390
";

#[test]
fn test_do_day8_prob1_test_input() {
    let expected = format!("{}", 21);
    let actual = do_day8(ADVENT_TEST_INPUT, 1);
    assert_eq!(String::from(expected), actual);
}
