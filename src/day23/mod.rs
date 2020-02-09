use crate::day5::{IntcodeState, run_intcode_state};
use std::collections::HashMap;

pub fn communicate() {
    let codes = crate::utils::read_intcodes("./src/day23/input");

    let mut states: Vec<IntcodeState> = Vec::new();
    let mut queues: Vec<Vec<(i128, i128)>> = Vec::new();

    for addr in 0..50 {

        let mut state = IntcodeState {
            codes: codes.clone(),
            resume_point: Some(0),
            relative_base: 0,
            mem: HashMap::new()
        };

        let res = run_intcode_state(&mut state, vec![addr], true);

        state.resume_point = res.resume_point;
        state.relative_base = res.relative_base;
        states.push(state);

        queues.push(Vec::new());
    }


    loop {
        for (idx, state) in states.iter_mut().enumerate() {

            let messages = &mut queues[idx];

            let mut inputs: Vec<i128> = Vec::new();

            if messages.len() > 0 {
                for m in messages.iter() {
                    inputs.push(m.0 as i128);
                    inputs.push(m.1 as i128);
                }
            } else {
                inputs.push(-1);
            }

            messages.clear();
            println!("{} running", idx);
            let res = run_intcode_state(state, inputs, true);

            let outputs = res.outputs_since_start_or_resume;
            let mut ii = 0;

            while ii < outputs.len() {
                let des = outputs[ii];
                let x = outputs[ii + 1];
                let y = outputs[ii + 2];
                
                for o in outputs.iter() {
                    println!("...{}...", o);
                }
                println!("{}: sending msg to {}", idx, des);
                if des == 255 {
                    println!("send X {}  Y {} to 255", x, y);
                    return;
                }

                queues[des as usize].push((x, y));


                ii += 3;
            }

            state.resume_point = res.resume_point;
            state.relative_base = res.relative_base;
        }
    }
}