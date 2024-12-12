use std::{fs, path::Path};

pub fn get_answer_p1(filepath: &Path) -> i32 {
    let reports = fs::read_to_string(filepath).expect("Issue reading file to string");

    // store return value of safety check
    let mut safety_score: i32 = 0;

    // flags
    let mut first_pass = true; // indicates whether first level in iter
    let mut second_pass = true; // indicates whether first level in iter
    let mut prev_dir = -1; // 0 negative, 1 positive, -1 not set
    let mut prev_val = -1; // prev level value encountered, -1 not set
    let mut cur_val: i32; // current level value encountered
    let mut dir: bool; // indicates level direction relative to previous level
    let mut safe: bool = true; // indicates if level safe (true default, set to false)

    for report in reports.lines() {

        println!("\n===============================================================\n");

        // iterate through levels in each report (chars in line string)
        for level in report.split_whitespace().peekable() {
            cur_val = level.parse::<i32>().expect("Data not able to parse to i32");
            if first_pass {
                println!("first pass");
                prev_val = cur_val;
                first_pass = false;
                continue;
            } else if second_pass {
                // if absolute value of change > 3, unsafe
                println!("second pass: prev_val {}", prev_val);
                if (cur_val - prev_val).abs() > 3 {
                    println!("cur_val {} is UNSAFE", cur_val);
                    break;
                }
                prev_dir = if cur_val > prev_dir { 1 } else { 0 };
                second_pass = false;
                prev_val = cur_val;
                continue;
            } else {
                // set direction
                dir = cur_val > prev_val;
                
                // If direction changed, or absolute value of change > 3, unsafe. Rest flags, break loop
                if (dir && (prev_dir == 1)) || (!dir && (prev_dir == 0)) || ((cur_val - prev_val).abs() > 3) {
                    safe = false;
                    println!("cur_val {} is UNSAFE", cur_val);
                    break;
                }
                println!("cur_val {} is OK", cur_val);
                prev_dir = if dir { 1 } else { 0 };
                prev_val = cur_val;
            }
        }

        // update score, reset safe flag
        if safe {
            safety_score += 1;
            first_pass = true;
            second_pass = true;
            prev_val = -1;
        } else {
            first_pass = true;
            second_pass = true;
            prev_val = -1;
            safe = true;
        }
    }
    // return safety score
    safety_score
}

pub fn get_answer_p2(filepath: &Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Issue reading file to string");
    println!("{}", input);
    1
}