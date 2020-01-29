use std::collections::HashMap;

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

pub fn repeat_10000_times() {
    let content = crate::utils::read_file("./src/day16/input");
    let mut input: Vec<i32> = content.split("").filter(|thing| *thing != "").map(|digit| digit.parse::<i32>().unwrap()).collect();

    let mut repeated_input: Vec<i32> = Vec::new();
    for _ in 0..10000 {
        for i in input.iter() {
            repeated_input.push(*i);
        }
    }

    let out_addr = repeated_input[0..7].iter().map(|d| format!("{}", d)).collect::<Vec<String>>().join("").parse::<u32>().unwrap();

    let pattern = vec![0, 1, 0, -1];

    for p in (0..100) {
        println!("phase {}", p);
        let mut next_input: Vec<i32> = Vec::new();

        let mut sum = vec![0; repeated_input.len() + 1]; // it's 1 element larger to hold the base sum 0 at index 0
        let mut s = 0;
        for (idx, d) in repeated_input.iter().enumerate() {
            s += d;
            sum[idx + 1] = s;
        }


        for step in 1..(repeated_input.len() + 1) {
            let mut pattern_pos = 1; 
            let mut result = 0;
          //  println!("digit {}", step);
            let mut i = step - 1; // starting where pattern value is 1
            while i < repeated_input.len() {
                
                let pattern_m = pattern[pattern_pos];
               // println!("i {} {} {}", i, step, pattern_m);

                if pattern_m == 0 {

                    i += step;
                
                    pattern_pos = (pattern_pos + 1) % pattern.len();
                    continue;
                }
                
                let l = if i + step >= sum.len() {
                    sum[sum.len() - 1]
                } else {
                    sum[i + step]
                };
                result += (l - sum[i]) * pattern_m;
    
                i += step;
            }
    
            next_input.push(result.abs() % 10);
        }

        repeated_input = next_input;
    }


    for i in (0..8) {
        print!("{}", repeated_input[(out_addr + i) as usize]);
    }

}