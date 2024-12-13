mod day04;

use std::path::Path;
use day04::{get_answer_p1, get_answer_p2};

fn main() {
    // grab input filepath
    let project_root = env!("CARGO_MANIFEST_DIR");

    println!("\n=====================Day04========================");
    let input_path = Path::new(project_root).join("day04.txt");
    println!("    Part 1 Answer: {}", get_answer_p1(&input_path));
    println!("    Part 2 Answer: {}", get_answer_p2(&input_path));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(project_root).join("day04-test.txt");
        assert_eq!(18, get_answer_p1(&input_path));
    }
    #[test]
    fn part_two() {
        let project_root = env!("CARGO_MANIFEST_DIR");
        let input_path = Path::new(project_root).join("day04-test.txt");
        assert_eq!(2, get_answer_p2(&input_path));
    }
}