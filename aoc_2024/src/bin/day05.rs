use std::{fs, path};

#[allow(unused)]
pub fn get_answer_p1() -> i32 {
    let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day04.txt");
    let input = fs::read_to_string(&filepath).expect("Failed to read file to string.");

    let (rules, orders) = input.split_once("\n\n").expect("Unexpected delimiter.");
    println!("Rules:\n{}\n\nOrders:\n{}", rules, orders);

    1
}

#[allow(unused)]
pub fn get_answer_p2() -> i32 {
    let filepath = path::Path::new(env!("CARGO_MANIFEST_DIR")).join("input/day04.txt");
    let input = fs::read_to_string(&filepath).expect("Failed to read file to string.");
    1
}

fn main() {
    get_answer_p1();
    get_answer_p2();
}
