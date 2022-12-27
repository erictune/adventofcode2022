#[derive(Debug, PartialEq)]
pub enum GridParseError {
    BadDigit,
    RaggedRows,
    NonFinalBlankLine,
}

// TODO: find a grid crate or make a struct out of (rows, cols, grid)

/// Read an array of digits in a text file to a Vec<Vec<u8>>.
/// Args:
/// input: string with newlines read from a file, containing array of digits.
/// Returns (r, c, grid) where:
/// r = number of rows
/// c = number of columns
/// grid[r-][c-1] is number in 0..=9 from row r, column c for text file(0-based)
pub fn digit_text_to_u8_grid(input: &str) -> Result<(usize, usize, Vec<Vec<u8>>), GridParseError> {
    let mut grid: Vec<Vec<u8>> = vec![];
    // I tried to do something functional like this:
    // input.split("\n").map(|line| line.map(|ch| ch.parse::<u8> ...))
    // but I could not find a clean way to return parse errors from the inner closure.
    // My conclusion is that functional is great when you don't have to handle errors, but
    // imperative code is fine when you have a lot of things to check.
    let mut saw_blank = false;
    for line in input.split("\n") {
        // A Final blank line is okay, but a non-final one is not.
        if saw_blank {
            return Err(GridParseError::NonFinalBlankLine);
        }
        if line.len() == 0 {
            saw_blank = true;
            continue;
        }

        let mut row: Vec<u8> = vec![];
        for ch in line.chars() {
            match ch.to_string().parse::<u8>() {
                Err(_) => return Err(GridParseError::BadDigit),
                Ok(digit) => row.push(digit),
            }
        }
        grid.push(row);
    }
    // Check all rows have ncols
    let rows = grid.len();
    let mincols = grid.iter().map(|r| r.len()).min().unwrap();
    let maxcols = grid.iter().map(|r| r.len()).max().unwrap();
    if mincols != maxcols {
        return Err(GridParseError::RaggedRows);
    }
    Ok((rows, mincols, grid))
}

#[test]
fn test_digit_text_to_u8_grid_good_nocr() {
    assert_eq!(
        digit_text_to_u8_grid(
            "\
12
12"
        ),
        Ok((2, 2, vec![vec![1, 2], vec![1, 2]]))
    );
}
#[test]
fn test_digit_text_to_u8_grid_bad_middle_cr() {
    assert_eq!(
        digit_text_to_u8_grid(
            "\
12

12
"
        ),
        Err(GridParseError::NonFinalBlankLine)
    );
}
#[test]
fn test_digit_text_to_u8_grid_bad_char() {
    assert_eq!(
        digit_text_to_u8_grid(
            "\
12
x2"
        ),
        Err(GridParseError::BadDigit)
    );
}
#[test]
fn test_digit_text_to_u8_grid_bad_ragged() {
    assert_eq!(
        digit_text_to_u8_grid(
            "\
12
123
12"
        ),
        Err(GridParseError::RaggedRows)
    );
}

pub fn display_grid(rows: usize, cols: usize, grid: &Vec<Vec<u8>>) -> String {
    let mut s = String::new();
    for r in 0..rows {
        for c in 0..cols {
            s.push_str(format!("{}", grid[r][c]).as_str());
        }
        s.push_str("\n");
    }
    s
}

#[test]
fn test_display_grid() {
    assert_eq!(
        "12\n34\n",
        display_grid(2, 2, &vec![vec![1, 2], vec![3, 4]])
    );
}
