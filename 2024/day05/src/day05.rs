use std::path::Path;

#[allow(unused)]
pub fn get_answer_p1(file: &str) -> i32 {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join(file);
    
    // read the file content to a data structure (okay to remain as string since number is irrelevant?)
    
    
    1
}

#[allow(unused)]
pub fn get_answer_p2(file: &str) -> i32 {
    let project_root = env!("CARGO_MANIFEST_DIR");
    let input_path = Path::new(project_root).join(file);
    2
}