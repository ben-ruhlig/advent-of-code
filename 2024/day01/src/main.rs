use std::{fs, path::Path};

fn main() {

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

    fn get_answer_p1(vec1: &[i32], vec2: &[i32]) -> i32 {

        vec1
            .iter()
            .zip(vec2.iter())
            .map(|(a, b)| (a - b).abs())
            .sum()
    }

    fn get_answer_p2(vec1: &[i32], vec2: &[i32]) -> i32 {

        vec1[0];
        vec2[0];

        // let mut vec3: Vec<i32>;
        for i in vec1 {
            println!("{}", i);
        }

        12345
    }

    
    // grab input filepath
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join("day01.txt"); 

    // process and sort input vectors
    let (vec1, vec2) = get_sorted_vectors(&input_path);

    println!("Part 1 Answer: {}", get_answer_p1(&vec1, &vec2));
    println!("Part 2 Answer: {}", get_answer_p2(&vec1, &vec2));
        
}
