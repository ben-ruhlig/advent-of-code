use std::fs;

pub fn solution(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Couldn't read to string");
    let word_search: Vec<Vec<char>> = input.lines().map(|row| row.chars().collect()).collect();

    let row_max: usize; // maximum index of outer Vec
    let col_max: usize; // maximum index of inner Vec

    // Set row_max and col_max
    if !word_search.is_empty() {
        row_max = word_search.len() - 1;
        // println!("Total Vec:\n{:?}", inp);
        let col_vec = word_search
            .first()
            .expect("Issue turning columns to vector.");
        col_max = col_vec.len() - 1;
    } else {
        panic!("Issue parsing rows.")
    }

    let mut words: i32 = 0;

    // iterate through word_search
    for row in 0..(row_max + 1) {
        for col in 0..(col_max + 1) {
            let letter: char = word_search[row][col];
            // If char is 'X', check for XMAS in all viable directions
            if letter == 'X' {
                // println!("\nLetter X @ {}, {}", row, col);
                // let start = words;

                words += check_leftup(&word_search, row, col);
                words += check_up(&word_search, row, col);
                words += check_rightup(&word_search, row, col, col_max);
                words += check_leftdown(&word_search, row, col, row_max);
                words += check_down(&word_search, row, col, row_max);
                words += check_rightdown(&word_search, row, col, row_max, col_max);
                words += check_left(&word_search, row, col);
                words += check_right(&word_search, row, col, col_max);

                // println!("Total XMAS found: {}", words - start);
            }
        }
    }

    // println!("expected answer: {}", x_cnt * 8);
    words
}

fn check_leftup(word_search: &[Vec<char>], x_row: usize, x_col: usize) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_row as i32 - 3 >= 0
        && x_col as i32 - 3 >= 0
        && (word_search[x_row - 1][x_col - 1] == 'M')
        && (word_search[x_row - 2][x_col - 2] == 'A')
        && (word_search[x_row - 3][x_col - 3] == 'S')
    {
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}

fn check_up(word_search: &[Vec<char>], x_row: usize, x_col: usize) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_row as i32 - 3 >= 0
        && (word_search[x_row - 1][x_col] == 'M')
        && (word_search[x_row - 2][x_col] == 'A')
        && (word_search[x_row - 3][x_col] == 'S')
    {
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}

fn check_rightup(word_search: &[Vec<char>], x_row: usize, x_col: usize, col_max: usize) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_row as i32 - 3 >= 0
        && x_col as i32 + 3 <= col_max as i32
        && (word_search[x_row - 1][x_col + 1] == 'M')
        && (word_search[x_row - 2][x_col + 2] == 'A')
        && (word_search[x_row - 3][x_col + 3] == 'S')
    {
        // println!("Found rightup");
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}

fn check_leftdown(word_search: &[Vec<char>], x_row: usize, x_col: usize, row_max: usize) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_row as i32 + 3 <= row_max as i32
        && x_col as i32 - 3 >= 0
        && (word_search[x_row + 1][x_col - 1] == 'M')
        && (word_search[x_row + 2][x_col - 2] == 'A')
        && (word_search[x_row + 3][x_col - 3] == 'S')
    {
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}

fn check_down(word_search: &[Vec<char>], x_row: usize, x_col: usize, row_max: usize) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_row as i32 + 3 <= row_max as i32
        && (word_search[x_row + 1][x_col] == 'M')
        && (word_search[x_row + 2][x_col] == 'A')
        && (word_search[x_row + 3][x_col] == 'S')
    {
        // println!("Found down");
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}

fn check_rightdown(
    word_search: &[Vec<char>],
    x_row: usize,
    x_col: usize,
    row_max: usize,
    col_max: usize,
) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_row as i32 + 3 <= row_max as i32
        && x_col as i32 + 3 <= col_max as i32
        && (word_search[x_row + 1][x_col + 1] == 'M')
        && (word_search[x_row + 2][x_col + 2] == 'A')
        && (word_search[x_row + 3][x_col + 3] == 'S')
    {
        // println!("Found rightdown");
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}

fn check_left(word_search: &[Vec<char>], x_row: usize, x_col: usize) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_col as i32 - 3 >= 0
        && (word_search[x_row][x_col - 1] == 'M')
        && (word_search[x_row][x_col - 2] == 'A')
        && (word_search[x_row][x_col - 3] == 'S')
    {
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}

fn check_right(word_search: &[Vec<char>], x_row: usize, x_col: usize, col_max: usize) -> i32 {
    // If XMAS would be in-bounds of word_search in this direction
    if x_col as i32 + 3 <= col_max as i32
        && (word_search[x_row][x_col + 1] == 'M')
        && (word_search[x_row][x_col + 2] == 'A')
        && (word_search[x_row][x_col + 3] == 'S')
    {
        return 1; // found XMAS!
    }
    0 // default return 0, not found
}
