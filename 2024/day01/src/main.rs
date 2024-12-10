use std::{fs, path::Path};

fn main() {

    fn part_one_get_answer(filepath: &Path) -> i32 {
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

        // sort the vectors, least to greatest
        vec1.sort();
        vec2.sort();

        // determine the answer
        let answer: i32 = vec1
            .iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum();

        answer
    }

     // filepath with cargo.toml (i.e. day01)
    let project_root = env!("CARGO_MANIFEST_DIR");
    // input data from 
    let p1_input_path = Path::new(project_root).join("day01.txt"); 
    let p1_answer = part_one_get_answer(&p1_input_path);
    println!("Part 1 Answer: {}", p1_answer);

}
