use std::{fs::File, io::Read};

fn main() {
    let mut input = String::new();
    File::open("input.txt").unwrap().read_to_string(&mut input).unwrap();

    println!("{:?}", input);
}
