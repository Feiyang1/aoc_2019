use std::collections::HashMap;

pub fn count_blocks() {
    let content = crate::utils::read_file("./src/day13/input");
    let mut codes: Vec<i128> = content
    .split(",")
    .map(|str_int| str_int.parse::<i128>().unwrap())
    .collect();


    let crate::day5::IntcodeResult { output, resume_point, relative_base, outputs_since_start_or_resume } = crate::day5::run_intcode_raw(&mut codes, None, vec![], 0, false, 0);

    let mut block_map: HashMap<String, bool> = HashMap::new();
    let mut block_count = 0;
    for i in (0..outputs_since_start_or_resume.len()).step_by(3) {
        let x = outputs_since_start_or_resume[i];
        let y = outputs_since_start_or_resume[i + 1];
        let tile_id = outputs_since_start_or_resume[i + 2];

        let key = format!("{}-{}", x, y);

        if tile_id == 2 { // block
            block_map.insert(key, true);
            block_count += 1;
        } else if tile_id == 0 { // empty
            if let Some(exist) = block_map.get(&key) {
                if *exist {
                    block_count -= 1;
                    block_map.remove(&key);
                }
            }
        }
    }

    println!("total {} block tiles", block_count);
}