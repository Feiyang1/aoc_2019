
pub fn max_thrust() {
   let (max_thrust, max_seq) = recurse((0..5).collect(), 0);
   println!("the max thrust possible is {}, seq: {}", max_thrust, max_seq);
}

fn recurse(phase_setting_left: Vec<i128>, input: i128) -> (i128, String) {
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
            let phase_setting_left_next: Vec<i128> = copy.into_iter().filter(|v| v != ps).collect();

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

pub fn max_thrust_repeat() {
    // crate::day5::run_intcode("./src/day7/input", vec![]);
     println!("max thrust is {}", recurse_and_execute(vec![], (5..10).collect()));
}

fn recurse_and_execute(permutation: Vec<i128>, phase_setting_left: Vec<i128>) -> i128 {
    if phase_setting_left.len() > 0 {
        let mut max = -10000;
        for ps in phase_setting_left.iter() {
            let mut permu = permutation.clone();
            permu.push(*ps);
            let copy = phase_setting_left.clone();
            let phase_setting_left_next: Vec<i128> = copy.into_iter().filter(|v| v != ps).collect();
            let result = recurse_and_execute(permu, phase_setting_left_next);

            if result > max {
                max = result;
            }
        }

        return max;
    } else { // execute amplifiers in loop
        let content = crate::utils::read_file("./src/day7/input");
        let a_codes: Vec<i128> = content
            .split(",")
            .map(|str_int| str_int.parse::<i128>().unwrap())
            .collect();

        let b_codes = a_codes.clone();
        let c_codes = a_codes.clone();
        let d_codes = a_codes.clone();
        let e_codes = a_codes.clone();
    
        let mut amplifiers = vec![a_codes, b_codes, c_codes, d_codes, e_codes];
        let mut amplifers_last_result: Vec<crate::day5::IntcodeResult> = vec![];

        for _ in (0..5) {
            amplifers_last_result.push(crate::day5::IntcodeResult{output: None, outputs_since_start_or_resume: vec![], resume_point: None});
        }

        let mut running_amp_idx = 0;
        let mut init_done = false;
        loop {
            let amplifier = &mut amplifiers[running_amp_idx];
         //   println!("got amplifier {}", running_amp_idx);
            let mut input = vec![];
            let mut resume_point = 0;
            if !init_done {
                input.push(permutation[running_amp_idx]);

                // input for the first amplifier
                if running_amp_idx == 0 {
                    input.push(0);
                } else {
                    let mut input_from: i128 = running_amp_idx as i128 - 1;
                    if input_from < 0 {
                        input_from = 4; // the last amplifier feeds to the first amplifier
                    }
                    input.push(amplifers_last_result[input_from as usize].output.unwrap())
                }
            } else {
           //     println!("looking for resume point {}", amplifers_last_result[running_amp_idx].resume_point.unwrap());
                resume_point = match amplifers_last_result[running_amp_idx].resume_point {
                   Some(p) => p,
                   None => {
            //        println!("is this the end?");
            //        println!("The max thrust is {}", amplifers_last_result[4].output.unwrap());
                    return amplifers_last_result[4].output.unwrap();
                   }
               };

               let mut input_from: i128 = running_amp_idx as i128 - 1;
               if input_from < 0 {
                    input_from = 4; // the last amplifier feeds to the first amplifier
                }
                input.push(amplifers_last_result[input_from as usize].output.unwrap())
            }

            println!("running amplifier {} with", running_amp_idx);
            let result = crate::day5::run_intcode_raw(amplifier, None, input, resume_point, true, 0);
        //    println!("amplifier {} outputs {}", running_amp_idx, result.output.unwrap());

            match result.resume_point {
                Some(i) => {println!("resume point found for {}", running_amp_idx)},
                None => {println!("no more resume point, program halts {}!", running_amp_idx)}
            }
            amplifers_last_result[running_amp_idx] = result;

            running_amp_idx = (running_amp_idx + 1) % 5;
            if running_amp_idx == 0 && !init_done {
                init_done = true;
            }

        //    println!("this ends here {}", running_amp_idx);
        }
        
    }
}