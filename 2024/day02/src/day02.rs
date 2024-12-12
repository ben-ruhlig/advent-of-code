use std::{fs, path::Path};


pub fn get_answer_p1(filepath: &Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");

    let reports: Vec<Vec<i32>> = input.lines()
        .map(| report | {
            report.split_whitespace().map( |score| {
                score.parse::<i32>().expect("i32")
            }).collect()
        }).collect();

    println!("{:?}", reports);
    1
}

pub fn get_answer_p2(filepath: &Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");
    println!("{}", input);
    1
}