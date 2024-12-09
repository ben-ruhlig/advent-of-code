use std::fs;

fn main() {
    // load input file to string
    let input = fs::read_to_string("input.txt").expect("Error reading file");

    // create integer vectors for data rows
    let (vec1, vec2): (Vec<i32>, Vec<i32>) = input.lines()
        .map(|line| {
            let parts: Vec<&str> = line.split_whitespace().collect();
            (parts[0].parse::<i32>(), parts[1].parse::<i32>())
        }).filter_map(|(v1, v2)| match (v1, v2) {
            (Ok(v1), Ok(v2)) => Some((v1, v2)),
            _ => None,
        }).unzip();
}
