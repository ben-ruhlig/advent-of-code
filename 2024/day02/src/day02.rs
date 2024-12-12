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
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");

    let reports: Vec<Vec<i32>> = input.lines()
        .map(| report | {
            report.split_whitespace()
            .map( |score| {
                score.parse::<i32>().expect("Invalid number")
            }).collect()
        }).collect();

    
    // populate purge index
    let safety_score: i32 = reports.iter().map(| report | {

        let mut last_diff: Option<i32> = None;
        let mut idx = 0;
        let mut failed_idx: Option<usize> = None;
        let mut score: i32 = 1;

        // First loop to identify first failure (if exists)
        for window in report.windows(2) {
            let diff = window[1] - window[0];
            
            // Fail case #1: Incline/decline too steep
            if diff.abs() > 3 || diff.abs() < 1 {
                failed_idx = Some(idx + 1);
                break;
            
            // Fail case #2: Direction change
            } else if let Some(last) = last_diff {
                if (last > 0 && diff < 0) || (last < 0 && diff > 0) {
                    failed_idx = Some(idx + 1);
                    break;
                }
            }
            // update for next iteration
            last_diff = Some(diff);
            idx += 1;
        }

        // If no failure, the report must be safe.
        if failed_idx.is_none() {
            return score;
        }

        // Try purging each element to see if the report becomes safe
        for purge_idx in 0..report.len() {
            let mut last_diff: Option<i32> = None;
            let mut is_safe = true;

            for window in report.iter().enumerate()
                .filter(|&(i, _)| i != purge_idx) // Skip the "purged" index
                .map(|(_, &val)| val)
                .collect::<Vec<_>>()
                .windows(2)
                {
                    let diff = window[1] - window[0];
                    // Fail case #1: Incline/decline too steep
                    if diff.abs() > 3 || diff.abs() < 1 {
                        is_safe = false;
                        break;
                    }
                    // Fail case #2: Direction change
                    if let Some(last) = last_diff {
                        if (last > 0 && diff < 0) || (last < 0 && diff > 0) {
                            is_safe = false;
                            break;
                        }
                    }

                    last_diff = Some(diff);
                }
            if is_safe {
                score = 1;
                break;
            } else {
                score = 0;
            }
        }
        score
    }).sum();
    safety_score
}