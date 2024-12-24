use std::{
    fs,
    path,
    collections::HashSet
};

#[allow(unused)]
pub fn get_answer_p1(filepath: &path::Path) -> usize{
    let input = fs::read_to_string(filepath).unwrap();

    let (orderings, updates) = input.split_once("\n\n").unwrap();

    let orderings: HashSet<(usize, usize)> = orderings
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &usize, y: &usize| !orderings.contains(&(*y, *x));

    let mut count = 0;
    for update in updates.lines() {
        let update: Vec<usize> = update.split(",").map(|x| x.parse().unwrap()).collect();

        if update.is_sorted_by(compare) {
            count += update[update.len() / 2];
        }
    }

    count
}

#[allow(unused)]
pub fn get_answer_p2(filepath: &path::Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");
    1
}

fn main() {
    let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day05.txt");
    println!("Day05 P1: {}", get_answer_p1(&filepath));
    // println!("Day05 P2: {}", get_answer_p2(&filepath));
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn day05_p1() {
        let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day05_test.txt");
        assert_eq!(143, get_answer_p1(&filepath));
    }
    // #[test]
    // fn day05_p2() {
    //     let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day05_test.txt");
    //     assert_eq!(, get_answer_p2(&filepath));
    // }
}
