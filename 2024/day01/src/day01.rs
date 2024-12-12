use std::{fs, path::Path};

fn get_sorted_vectors(filepath: &Path) -> (Vec<i32>, Vec<i32>) {
    // load input file to string
    let input = fs::read_to_string(filepath).expect("Error reading file");

    // create integer vectors for data rows
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = input.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].parse::<i32>(), parts[1].parse::<i32>())
        }).filter_map(|(v1, v2)| match (v1, v2) {
            (Ok(v1), Ok(v2)) => Some((v1, v2)),
            _ => None,
        }).unzip();

    // return sorted vectors
    vec1.sort();
    vec2.sort();
    (vec1, vec2)
}

pub fn get_answer_p1(filepath: &Path) -> i32 {

    let (vec1, vec2) = get_sorted_vectors(&filepath);

    vec1
        .iter()
        .zip(vec2.iter())
        .map(|(a, b)| (a - b).abs())
        .sum()
}

pub fn get_answer_p2(filepath: &Path) -> i32 {

    let (vec1, vec2) = get_sorted_vectors(&filepath);

    let mut vec3: Vec<i32> = Vec::new();
    let mut cnt = 0;

    for i in &vec1 {
        for j in &vec2 {
            if i == j {
                cnt += 1;
            }
        vec3.push(cnt * i);
        cnt = 0;
        }
    }
    vec3.iter().sum()
}