use std::{fs, path::Path};

pub fn get_answer_p1(filepath: &Path) -> u32 {
    let input = fs::read_to_string(filepath).expect("Couldn't read to string");
    let word_search: Vec<Vec<char>> = input.lines().map(| row | {
        row.chars().collect()
    }).collect();

    let row_max: usize;
    let col_max: usize;

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

    for row in 0..row_max {
        for col in 0..col_max {
            let letter: char = word_search[row][col];
            if letter == 'X' {
                println!("Letter X at row: {} and col: {}", row, col);
            }
        }
    }

    // println!("row_max: {}, col_max: {}", row_max, col_max);
    1
}

pub fn get_answer_p2(filepath: &Path) -> i32 {
    println!("{}", filepath.display());
    1
}
