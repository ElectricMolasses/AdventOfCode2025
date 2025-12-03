use std::fs;
use std::path::Path;

pub fn read_by_line(file_path: &str) -> String {
    let path = Path::new(file_path);
    let contents = fs::read_to_string(path)
            .expect("TO READ DAH DAYUM FILE BOY");


    return contents;
}
