// I'M KEEPING THIS AS INFORMATION EVEN THOUGH IT'S WRONG.
// THE REQUIREMENTS DICTATE THAT I CAN'T JUST REMOVE SEGEMENTS THE WAY I AM. I NEED TO TAKE AN ENABLE, DISABLE APPROACH

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
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");
    let re_exclude = Regex::new(r"(don't.*?do\(\))").unwrap();
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let mut excl_vec: Vec<(usize, usize)> = Vec::new();

    // populate excl_vec with match start/end
    for excl in re_exclude.find_iter(&input) {
        excl_vec.push((excl.start(), excl.end()));
    }

    if excl_vec.is_empty() {
        return 0;
    }

    let mut result = String::new();
    let mut prev_end = 0;

    for (start, end) in excl_vec {
        result.push_str(&input[prev_end..start]);
        prev_end = end;
    }
    result.push_str(&input[prev_end..]);

    println!("\nResult: {}", result);

    let muls: Vec<(i32, i32)> = re.captures_iter(&result).map(|mul| {
        let (_, [x, y]) = mul.extract();
        (
            x.parse::<i32>().expect("Invalid number"),
            y.parse::<i32>().expect("Invalid number"),
        )
    }).collect();

    println!("\nMuls: {:?}", muls);

    let multiplied: Vec<i32>  = muls.iter().map(|(x, y)| x * y).collect();
    multiplied.iter().sum()
}