use std::fs;

pub fn readFile(path: &str) -> String {
   return fs::read_to_string(path).expect("reading file failed");
}