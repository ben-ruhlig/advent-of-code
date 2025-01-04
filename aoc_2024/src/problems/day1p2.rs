use std::fs;

fn get_sorted_vectors(input: String) -> (Vec<i32>, Vec<i32>) {
    // create integer vectors for data rows
    let (mut vec1, mut vec2): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].parse::<i32>(), parts[1].parse::<i32>())
        })
        .filter_map(|(v1, v2)| match (v1, v2) {
            (Ok(v1), Ok(v2)) => Some((v1, v2)),
            _ => None,
        })
        .unzip();

    // return sorted vectors
    vec1.sort();
    vec2.sort();
    (vec1, vec2)
}

pub fn solution(file_path: &str) -> i32 {
    let input = fs::read_to_string(file_path).expect("Error reading file");
    let (vec1, vec2) = get_sorted_vectors(input);

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
