use std::{fs, path};

#[allow(unused)]
pub fn get_answer_p1(filepath: &path::Path) -> i32 {
    let input = fs::read_to_string(filepath).expect("Failed to read file to string.");

    let (rules, orders) = input.split_once("\n\n").expect("Unexpected delimiter.");
    println!("Rules:\n{}\n\nOrders:\n{}", rules, orders);

    1
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
