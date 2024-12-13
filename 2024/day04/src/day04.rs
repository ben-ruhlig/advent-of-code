use std::{fs, path::Path};

pub fn get_answer_p1(filepath: &Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Couldn't read to string");
    let word_search: Vec<Vec<char>> = input.lines().map(| row | {
        row.chars().collect()
    }).collect();

    let row_max: usize;
    let col_max: usize;

    // Set row_max and col_max
    if !word_search.is_empty() {
        row_max = word_search.len();
        // println!("Total Vec:\n{:?}", inp);
        if word_search.get(0).is_some() {
            let col_vec = word_search.get(0).expect("Issue turning columns to vector.");
            col_max = col_vec.len();
        } else {
            panic!("Issue parsing columns.")
        }
    } else {
        panic!("Issue parsing rows.")
    }

    let mut words: i32 = 0;

    // iterate through word_search
    for row in 0..row_max {
        for col in 0..col_max {
            let letter: char = word_search[row][col];
            // If char is 'X', check for XMAS in all viable directions
            if letter == 'X' {
                words += check_leftup(&word_search, row, col);
                words += check_up(&word_search, row, col);
                words += check_rightup(&word_search, row, col);
                words += check_leftdown(&word_search, row, col);
                words += check_down(&word_search, row, col);
                words += check_rightdown(&word_search, row, col);
                words += check_left(&word_search, row, col);
                words += check_right(&word_search, row, col);
            }
        }
    }

    // println!("expected answer: {}", x_cnt * 8);
    words
}


pub fn get_answer_p2(filepath: &Path) -> i32 {
    println!("{}", filepath.display());
    1
}


fn check_leftup(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}


fn check_up(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}


fn check_rightup(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}


fn check_leftdown(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}


fn check_down(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}


fn check_rightdown(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}


fn check_left(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}       


fn check_right(word_search: &Vec<Vec<char>>, x_row: usize, x_col: usize) -> i32 {
    //if x_row as i32 - 3 < 0 || x_col as i32 - 3 < 0 {
    //    return 0;
    //}

    1
}
