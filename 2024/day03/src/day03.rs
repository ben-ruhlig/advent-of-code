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
    let re = Regex::new(r"mul\((\d+),(\d+)\)").unwrap();
    // let re_do = Regex::new(r"|do\(\)").unwrap();
    // let re_dont = Regex::new(r"don't\(\)").unwrap();

    let mut enabled = true;
    let mut enabled_muls: Vec<(i32, i32)> = Vec::new();

    for mat in re.find_iter(&input) {
        let m = mat.as_str();
        if  m.starts_with("mul(") && m.ends_with(")") && m.contains(",") && enabled {
            println!("{}", m);
            let end = &m.len() - 1;
            let num_vec: Vec<&str> = m[4..end].split(',').collect();
            let x = num_vec[0].parse::<i32>().expect("Integer conversion failed.");
            let y = num_vec[1].parse::<i32>().expect("Integer conversion failed.");
            enabled_muls.push((x, y));
        } else if m.starts_with("don't") {
            enabled = false;
        } else {
            enabled = true;
        }
    }
    
    if enabled_muls.is_empty() {
        return 0;
    } else {
        let sum: i32 = enabled_muls.iter().map(|(x, y)| x * y).sum();
        return sum;
    }
}
