use std::{fs, path::Path};

pub fn get_answer_p1(filepath: &Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");

    let reports: Vec<Vec<i32>> = input.lines()
        .map(| report | {
            report.split_whitespace()
            .map( |score| {
                score.parse::<i32>().expect("i32")
            }).collect()
        }).collect();

    let safety_score = reports.iter().map(| report | {

        let mut last_diff: Option<i32> = None;
        let mut score = 1;

        for window in report.windows(2) {
            let diff = window[1] - window[0];
            // check for fail case #1: incline/decline too steep
            if diff.abs() > 3 || diff.abs() < 1 {
                score = 0;
            } else if let Some(last) = last_diff {
                // cehck for fail case #2: direction change
                if (last > 0 && diff < 0) || (last < 0 && diff > 0) {
                    score = 0;
                }
            }
            last_diff = Some(diff);
        }
        score
    }).sum();
    safety_score
}

pub fn get_answer_p2(filepath: &Path) -> i32 {
    println!("{}", filepath.display());
    // let input = fs::read_to_string(filepath).expect("Issue reading file to string");
    1
}