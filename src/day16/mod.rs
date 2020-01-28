pub fn fft() {
    let content = crate::utils::read_file("./src/day16/input");
    let mut input: Vec<i32> = content.split("").filter(|thing| *thing != "").map(|digit| digit.parse::<i32>().unwrap()).collect();

    let pattern = vec![0, 1, 0, -1];

    for _ in (0..100) {
        let mut next_input: Vec<i32> = Vec::new();
        for step in 1..(input.len() + 1) {
            let mut pattern_pos = 0;
            let mut count = 1; // shift to left by 1
            let mut result = 0;
            for digit in input.iter() {
                if count == step {
                    pattern_pos = (pattern_pos + 1) % pattern.len();
                    count = 0;
                }
                let pattern_m = pattern[pattern_pos];
    
                result += digit * pattern_m;
    
                count += 1;
            }
    
            next_input.push(result.abs() % 10);
        }

        input = next_input;
    }



    for d in input.iter() {
        print!("{}", d);
    }
}