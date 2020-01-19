pub fn boost() {
    let content = crate::utils::read_file("./src/day9/input");
    let mut codes: Vec<i128> = content
        .split(",")
        .map(|str_int| str_int.parse::<i128>().unwrap())
        .collect();

    println!("testtest");
    crate::day5::run_intcode_raw(&mut codes, None, vec![], 0, false, 0);
}