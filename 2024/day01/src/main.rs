use std::fs;

fn main() {
    let loc_id_lists = fs::read("./input.txt")
        .expect("Couldn't read file ./input.txt");

    for id in loc_id_lists {
        println!("{id}");
    }
}
