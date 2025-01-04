use std::fs;

pub fn solution(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Issue reading file to string");

    let reports: Vec<Vec<i32>> = input
        .lines()
        .map(|report| {
            report
                .split_whitespace()
                .map(|score| score.parse::<i32>().expect("Invalid number"))
                .collect()
        })
        .collect();

    // populate purge index
    let safety_score: i32 = reports
        .iter()
        .map(|report| {
            let mut last_diff: Option<i32> = None;
            let mut failed_index: Option<usize> = None;
            let mut score: i32 = 1;

            // First loop to identify first failure (if exists)
            for (index, window) in report.windows(2).enumerate() {
                let diff = window[1] - window[0];

                // Fail case #1: Incline/decline too steep
                if diff.abs() > 3 || diff.abs() < 1 {
                    failed_index = Some(index + 1);
                    break;

                // Fail case #2: Direction change
                } else if let Some(last) = last_diff {
                    if (last > 0 && diff < 0) || (last < 0 && diff > 0) {
                        failed_index = Some(index + 1);
                        break;
                    }
                }
                // update for next iteration
                last_diff = Some(diff);
            }

            // If no failure, the report must be safe.
            if failed_index.is_none() {
                return score;
            }

            // Try purging each element to see if the report becomes safe
            for purge_idx in 0..report.len() {
                let mut last_diff: Option<i32> = None;
                let mut is_safe = true;

                for window in report
                    .iter()
                    .enumerate()
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
        })
        .sum();
    safety_score
}
