use std::{fs, path::Path};

#[allow(unused)]
pub fn get_answer_p1(file: &str) -> i32 {
    let input = fs::read_to_string(Path::new(env!("CARGO_MANIFEST_DIR")).join(file))
        .expect("Failed to read file to string.");

    let (rules, orders) = input.split_once("\n\n").expect("Unexpected delimiter.");
    println!("Rules:\n{}\n\nOrders:\n{}", rules, orders);

    1
}

#[allow(unused)]
pub fn get_answer_p2(file: &str) -> i32 {
    1
}
