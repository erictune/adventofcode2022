mod loadgrid;
use crate::loadgrid::digit_text_to_u8_grid;
use crate::loadgrid::display_grid;
use grid::*;
use std::cmp::max;

// Count the number of items in iterator less than h and count the first item >= h.
/// Stops counting at the end of the iterator.
/// Counts and immediately returns on the first item >= h.
fn count_to_first_ge_incl<T: Iterator>(ii: T, h: T::Item) -> usize
where
    T::Item: PartialOrd,
{
    let mut n = 0;
    for t in ii {
        if t >= h {
            n += 1;
            return n;
        }
        n += 1;
    }
    n
}

#[test]
fn test_count_to_first_ge_incl() {
    assert_eq!(count_to_first_ge_incl(vec![1, 2, 3, 4, 5].iter(), &5), 5);
    assert_eq!(
        count_to_first_ge_incl(vec![1, 2, 1, 2, 1, 2, 3, 4, 5].iter(), &5),
        9
    );
    assert_eq!(count_to_first_ge_incl(vec![1, 2, 3, 4, 5].iter(), &6), 5);
    assert_eq!(count_to_first_ge_incl(vec![1, 2, 3, 4, 5].iter(), &4), 4);
    assert_eq!(count_to_first_ge_incl(vec![1, 2, 3, 4, 5].iter(), &0), 1);
    assert_eq!(count_to_first_ge_incl(vec![].iter(), &5), 0);
    assert_eq!(count_to_first_ge_incl(vec![6, 7, 8, 9, 10].iter(), &5), 1);
    assert_eq!(count_to_first_ge_incl(vec![5, 4, 3, 2, 1].iter(), &6), 5);
}

/// View distance from point (r,c) looking north (0,0 is NW-most point).
fn n_view(r: usize, c: usize, grid: &Grid<u8>) -> usize {
    let h = grid[r][c];
    let scan_start = grid.iter_col(c).rev().skip(grid.rows() - r);
    count_to_first_ge_incl(scan_start, &h)
}

/// View distance from point (r,c) looking south ((0,0 is NW-most point).).
fn s_view(r: usize, c: usize, grid: &Grid<u8>) -> usize {
    let h = grid[r][c];
    //[4,1,2,3,4,5] -> 4
    //[4,1] -> 1
    let scan_start = grid.iter_col(c).skip(r + 1);
    count_to_first_ge_incl(scan_start, &h)
}

/// View distance from point (r,c) looking west ((0,0 is NW-most point).).
fn e_view(r: usize, c: usize, grid: &Grid<u8>) -> usize {
    let h = grid[r][c];
    let scan_start = grid.iter_row(r).skip(c + 1);
    count_to_first_ge_incl(scan_start, &h)
}

/// View distance from point (r,c) looking east ((0,0 is NW-most point).).
fn w_view(r: usize, c: usize, grid: &Grid<u8>) -> usize {
    let h = grid[r][c];
    let scan_start = grid.iter_row(r).rev().skip(grid.cols() - c);
    count_to_first_ge_incl(scan_start, &h)
}

#[test]
fn test_view_aoc() {
    let aoc_test_input = grid![
    [3,0,3,7,3]
    [2,5,5,1,2]
    [6,5,30,3,2]
    [3,3,5,4,9]
    [3,5,33,9,0]
    ];
    assert_eq!(n_view(1, 2, &aoc_test_input), 1);
    //assert_eq!(w_view(1, 2, &aoc_test_input), 1);
    //assert_eq!(e_view(1, 2, &aoc_test_input), 2);
    //assert_eq!(s_view(1, 2, &aoc_test_input), 2);
}

#[test]
fn test_view_big() {
    let test_grid: Grid<u8> = grid![
    //   0 1 2 3 4 5 6 7 8 9 10
        [9,9,9,9,9,9,9,9,9,9,9]   // row 0
        [9,8,8,8,8,8,8,8,8,8,9]   //     1
        [9,8,7,7,7,7,7,7,7,8,9]   //     2
        [9,8,7,6,6,6,6,6,7,8,9]   //     3
        [9,8,7,6,7,7,7,6,7,8,9]   //     4
        [9,8,7,6,7,8,7,6,7,8,9]   //     5
        [9,8,7,6,7,7,7,6,7,8,9]   //     6
        [9,8,7,6,6,6,6,6,7,8,9]   //     7
        [9,8,7,7,7,7,7,7,7,8,9]   //     8
        [9,8,8,8,8,8,8,8,8,8,9]   //     9
        [9,9,9,9,9,9,9,9,9,9,9]]; //     10
    assert_eq!(n_view(5, 5, &test_grid), 4);
    assert_eq!(w_view(5, 5, &test_grid), 4);
    assert_eq!(s_view(5, 5, &test_grid), 4);
    assert_eq!(e_view(5, 5, &test_grid), 4);

    assert_eq!(n_view(4, 4, &test_grid), 2);
    assert_eq!(w_view(4, 4, &test_grid), 2);
    assert_eq!(s_view(4, 4, &test_grid), 1);
    assert_eq!(e_view(4, 4, &test_grid), 1);

    assert_eq!(n_view(4, 6, &test_grid), 2);
    assert_eq!(w_view(4, 6, &test_grid), 1);
    assert_eq!(s_view(4, 6, &test_grid), 1);
    assert_eq!(e_view(4, 6, &test_grid), 2);

    assert_eq!(n_view(6, 6, &test_grid), 1);
    assert_eq!(w_view(6, 6, &test_grid), 1);
    assert_eq!(s_view(6, 6, &test_grid), 2);
    assert_eq!(e_view(6, 6, &test_grid), 2);

    assert_eq!(n_view(6, 4, &test_grid), 1);
    assert_eq!(w_view(6, 4, &test_grid), 2);
    assert_eq!(s_view(6, 4, &test_grid), 2);
    assert_eq!(e_view(6, 4, &test_grid), 1);
}

fn scenic_score(r: usize, c: usize, grid: &Grid<u8>) -> usize {
    // TODO: compute best scenic score as product of n/e/s/w scores;
    n_view(r, c, grid) * s_view(r, c, grid) * e_view(r, c, grid) * w_view(r, c, grid)
}

#[test]
fn test_scenic_score() {
    let aoc_test_input = grid![
    [3,0,3,7,3]
    [2,5,5,1,2]
    [6,5,3,3,2]
    [3,3,5,4,9]
    [3,5,3,9,0]
    ];

    assert_eq!(scenic_score(1, 2, &aoc_test_input), 4);
}
/// Solve day8 problem from Advent of Code 2022.
pub fn do_day8(input: &str, mode: i32) -> String {
    if mode != 1 && mode != 2 {
        unimplemented!("Unknown mode");
    }
    let (rows, cols, grid) = digit_text_to_u8_grid(input).unwrap();
    let mut visible: Grid<u8> = Grid::new(rows, cols);
    // Edge trees are going to be visible because we will start the with max-height-seen value of -1.
    const INITIAL_MAX: i32 = -1;
    // Check each column north and south.
    for c in 0..=cols - 1 {
        {
            // Looking south along this column from the north side.
            let mut max = INITIAL_MAX;
            for r in 0..=rows - 1 {
                if grid[r][c] as i32 > max {
                    max = grid[r][c] as i32;
                    visible[r][c] = 1;
                }
            }
        }
        {
            // Looking north along this column from the south side
            let mut max = INITIAL_MAX;
            for r in (0..=rows - 1).rev() {
                if grid[r][c] as i32 > max {
                    max = grid[r][c] as i32;
                    visible[r][c] = 1;
                }
            }
        }
    }
    // Check each row east and west.

    // To debug intermediate results:
    // println!("{}", display_grid(rows, cols, &grid));
    // println!("{}", display_grid(rows, cols, &visible));

    for r in 0..=rows - 1 {
        {
            // Looking east from the west side, checking full row at a time.
            let mut max = INITIAL_MAX;
            for c in 0..=cols - 1 {
                if grid[r][c] as i32 > max {
                    max = grid[r][c] as i32;
                    visible[r][c] = 1;
                }
            }
        }
        {
            let mut max = INITIAL_MAX;
            for c in (0..=cols - 1).rev() {
                if grid[r][c] as i32 > max {
                    max = grid[r][c] as i32;
                    visible[r][c] = 1;
                }
            }
        }
    }
    println!("{}", display_grid(rows, cols, &grid));
    println!("{}", display_grid(rows, cols, &visible));

    let num_vis: u64 = visible.iter().fold(0u64, |sum, n| sum + (*n as u64));

    if mode == 1 {
        return String::from(format!("{}", num_vis));
    }

    let mut best_scenic_score: usize = 0;
    for r in 0..=rows - 1 {
        for c in (0..=cols - 1).rev() {
            if visible[r][c] == 0 {
                continue;
            }
            best_scenic_score = max(best_scenic_score, scenic_score(r, c, &grid));
        }
    }

    return String::from(format!("{}", best_scenic_score));
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

#[test]
fn test_do_day8_prob2_test_input() {
    let expected = format!("{}", 8);
    let actual = do_day8(ADVENT_TEST_INPUT, 2);
    assert_eq!(String::from(expected), actual);
}
