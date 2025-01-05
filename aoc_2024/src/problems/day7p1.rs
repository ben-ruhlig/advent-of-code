use std::fs;


pub fn solution(file_path: &str) -> u32 {
    let input = fs::read_to_string(file_path).unwrap();
    input.lines().for_each(|line| {
        println!("{}", line);
    });
    1
}
