use std::fs;

pub fn read_from_file(file_path: &str) -> Vec<String> {
    let contents = fs::read_to_string(file_path).expect("Should have been able to read the file");
    contents.split('\n')
        .map(str::to_string).collect()
}