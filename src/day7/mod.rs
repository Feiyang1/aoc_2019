
pub fn max_thrust() {
   let (max_thrust, max_seq) = recurse((0..5).collect(), 0);
   println!("the max thrust possible is {}, seq: {}", max_thrust, max_seq);
}

fn recurse(phase_setting_left: Vec<i32>, input: i32) -> (i32, String) {
    // println!("level {}, input {}", phase_setting_left.len(), input);
    let mut max = -1000000;
    let mut max_seq = format!("");
    for ps in phase_setting_left.iter() {
        let out = crate::day5::run_intcode("./src/day7/input", vec![*ps, input]);

        if phase_setting_left.len() == 1 {
            if out > max {
                max = out;
                max_seq = format!("{}", ps);
            }
        } else {
            let copy = phase_setting_left.clone();
            let phase_setting_left_next: Vec<i32> = copy.into_iter().filter(|v| v != ps).collect();

            if phase_setting_left.len() == 5 {
                println!("using ps {}, output {}, left {}", ps, out, format!("{}, {}, {}, {}", phase_setting_left_next[0], phase_setting_left_next[1], phase_setting_left_next[2], phase_setting_left_next[3]));
            }

            let (thrust, seq) = recurse(phase_setting_left_next, out);
            println!("thrust is {}, input is {}", thrust, out);
            if thrust > max {
                max = thrust;
                max_seq = format!("{} -> {}", ps, seq);
            }
        }

    }
    
    return (max, max_seq);
}