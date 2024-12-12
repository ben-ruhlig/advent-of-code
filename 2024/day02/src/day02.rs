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
            
            // FAIL #1: incline/decline too steep
            if diff.abs() > 3 || diff.abs() < 1 {
                failed_idx = Some(idx + 1);
                break;
            
            // FAIL #2: direction change
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

        // Second loop only only if initial failure was identified
        if let Some(purge_idx) = failed_idx {
            last_diff = None;
            // create vector excluding failed level
            let purged_vector: Vec<i32> = report.iter()
                .enumerate()
                .filter(| &(i, _)| i != purge_idx)
                .map(|(_, &value)| value)
                .collect();

            // loop through purged vector to find another failure
            for window in purged_vector.windows(2) {
                let diff = window[1] - window[0];
                // FAIL #1: incline/decline too steep
                if diff.abs() > 3 || diff.abs() < 1 {
                    score = 0;
                    break;
                // FAIL #2: direction change
                } else if let Some(last) = last_diff {
                    if (last > 0 && diff < 0) || (last < 0 && diff > 0) {
                        score = 0;
                        break;
                    }
                }
                // update for next iteration
                last_diff = Some(diff);
            }
        }
        score
    }).sum();
    safety_score
}