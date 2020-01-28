use std::fs;

pub fn read_file(path: &str) -> String {
   return fs::read_to_string(path).expect("reading file failed");
}

pub fn read_intcodes(path: &str) -> Vec<i128> {
   let content = read_file(path);
   content
       .split(",")
       .map(|str_int| str_int.parse::<i128>().unwrap())
       .collect()
}