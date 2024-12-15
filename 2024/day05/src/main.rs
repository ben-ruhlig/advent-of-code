mod day05;

use std::time::Instant;
use day05::{get_answer_p1, get_answer_p2};

fn main() {
    println!("\n=====================Day05========================");
    
    // part 1
    {
        let start = Instant::now();
        let solution = get_answer_p1("day05.txt");
        let duration = start.elapsed();
        println!("    Part 1 Answer: {solution}\n        Duration: {duration:?}");
    }
    
    // part 2
    {
        let start = Instant::now();
        let solution = get_answer_p2("day05.txt");
        let duration = start.elapsed();
        println!("    Part 2 Answer: {solution}\n        Duration: {duration:?}");
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_one() {
        assert_eq!(143, get_answer_p1("day05-test.txt"));
    }
    #[test]
    fn part_two() {
        assert_eq!(1, get_answer_p2("day05-test.txt"));
    }
}