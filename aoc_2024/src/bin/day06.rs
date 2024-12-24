use std::{fs, path,};
use std::collections::HashSet;

#[derive(Debug, Clone, Copy)]
enum Direction {
    Right,
    Down,
    Left,
    Up,
}

impl Direction {
    fn turn_right(&mut self) -> Self {
        match self {
            Direction::Right => Direction::Down,
            Direction::Down => Direction::Left,
            Direction::Left => Direction::Up,
            Direction::Up => Direction::Right,
        }
    }
}

#[derive(Debug, Clone, Copy)]
struct Position {
    x: usize,
    y: usize,
}






#[allow(unused_variables)]
pub fn get_answer_p1(filepath: &path::Path) -> usize{
    let input = fs::read_to_string(filepath).unwrap();
    1
}

#[allow(unused_variables)]
pub fn get_answer_p2(filepath: &path::Path) -> usize {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");
    1
}

fn main() {
    let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06.txt");
    println!("Day06 P1: {}", get_answer_p1(&filepath));
    println!("Day06 P2: {}", get_answer_p2(&filepath));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day06_p1() {
        let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06_test.txt");
        assert_eq!(41, get_answer_p1(&filepath));
    }
    #[test]
    fn day06_p2() {
        let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day06_test.txt");
        assert_eq!(2, get_answer_p2(&filepath));
    }
}
