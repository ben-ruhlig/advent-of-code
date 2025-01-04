use std::{cmp::Ordering, collections::HashSet, fs};

pub fn solution(file_path: &str) -> usize {
    let input = fs::read_to_string(file_path).expect("Failed to read file to string.");
    let (orderings, updates) = input.split_once("\n\n").unwrap();
    let orderings: HashSet<(usize, usize)> = orderings
        .lines()
        .map(|line| (line[0..2].parse().unwrap(), line[3..].parse().unwrap()))
        .collect();

    let compare = |x: &usize, y: &usize| !orderings.contains(&(*y, *x));

    let sort_compare = |x: &usize, y: &usize| {
        let (x, y) = (*x, *y);
        if orderings.contains(&(x, y)) {
            Ordering::Less
        } else if orderings.contains(&(y, x)) {
            Ordering::Greater
        } else {
            Ordering::Equal
        }
    };

    let mut count = 0;
    for update in updates.lines() {
        let mut update: Vec<usize> = update.split(",").map(|x| x.parse().unwrap()).collect();
        if !update.is_sorted_by(compare) {
            update.sort_by(sort_compare);
            count += update[update.len() / 2]
        }
    }
    count
}
