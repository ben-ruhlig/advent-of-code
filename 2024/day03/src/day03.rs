use std::{fs, path::Path};
use regex::Regex;

pub fn get_answer_p1(filepath: &Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let muls: Vec<(i32, i32)> = re.captures_iter(&input).map(|mul| {
        let (_, [x, y]) = mul.extract();
        (
            x.parse::<i32>().expect("Invalid number"),
            y.parse::<i32>().expect("Invalid number"),
        )
    }).collect();

    let multiplied: Vec<i32>  = muls.iter().map(|(x, y)| x * y).collect();
    multiplied.iter().sum()
}

pub fn get_answer_p2(filepath: &Path) -> i32 {
    println!("{:?}", filepath);
    1
}