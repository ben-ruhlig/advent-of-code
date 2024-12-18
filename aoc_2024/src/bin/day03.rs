use regex::Regex;
use std::{fs, path};

pub fn get_answer_p1(filepath: &path::Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");
    let re = Regex::new(r"mul\((\d{1,3}),(\d{1,3})\)").unwrap();

    let muls: Vec<(i32, i32)> = re
        .captures_iter(&input)
        .map(|mul| {
            let (_, [x, y]) = mul.extract();
            (
                x.parse::<i32>().expect("Invalid number"),
                y.parse::<i32>().expect("Invalid number"),
            )
        })
        .collect();

    let multiplied: Vec<i32> = muls.iter().map(|(x, y)| x * y).collect();
    multiplied.iter().sum()
}

pub fn get_answer_p2(filepath: &path::Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // let re_do = Regex::new(r"|do\(\)").unwrap();
    // let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut enabled = true;
    let mut enabled_muls: Vec<(i32, i32)> = Vec::new();

    for mat in re.find_iter(&input) {
        let m = mat.as_str();
        if m.starts_with("mul(") && m.ends_with(")") && m.contains(",") && enabled {
            println!("{}", m);
            let end = &m.len() - 1;
            let num_vec: Vec<&str> = m[4..end].split(',').collect();
            let x = num_vec[0]
                .parse::<i32>()
                .expect("Integer conversion failed.");
            let y = num_vec[1]
                .parse::<i32>()
                .expect("Integer conversion failed.");
            enabled_muls.push((x, y));
        } else {
            enabled = !m.starts_with("don't");
        }
    }

    if enabled_muls.is_empty() {
        0
    } else {
        enabled_muls.iter().map(|(x, y)| x * y).sum()
    }
}

fn main() {
    let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day03.txt");
    println!("Day03 P1: {}", get_answer_p1(&filepath));
    // println!("Day03 P2: {}", get_answer_p2(&filepath));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day03_p1() {
        let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day03_test.txt");
        assert_eq!(161, get_answer_p1(&filepath));
    }
    // #[test]
    // fn day03_p2() {
    //     let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day03p2_test.txt");
    //     assert_eq!(48, get_answer_p2(&filepath));
    // }
}
