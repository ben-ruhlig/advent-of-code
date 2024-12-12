mod day01;

use std::path::Path;
use day01::{get_answer_p1, get_answer_p2};

fn main() {

    // grab input filepath
    let project_root = env!("CARGO_MANIFEST_DIR");

    println!("\n=====================Day01========================");
    let day01_input = Path::new(project_root).join("day01.txt");
    println!("    Part 1 Answer: {}", get_answer_p1(&day01_input));
    println!("    Part 2 Answer: {}", get_answer_p2(&day01_input));
        
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let test_input = Path::new(project_root).join("day01-test.txt");
        assert_eq!(11, get_answer_p1(&test_input));
    }
    #[test]
    fn part_two() {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let test_input = Path::new(project_root).join("day01-test.txt");
        assert_eq!(31, get_answer_p2(&test_input));
    }
}
