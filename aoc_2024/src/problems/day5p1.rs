use std::{collections::HashSet, fs};

pub fn solution(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).unwrap();

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
