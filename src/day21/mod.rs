pub fn pt1() {
    let mut codes = crate::utils::read_intcodes("./src/day21/input");

    let probe_programes = [
        "OR C T",
        "AND B T",
        "AND A T",
        "NOT T J",
        "AND D J",
        "WALK"
    ];

    let mut inputs = vec![];

    for instruction in probe_programes.iter() {
        for c in instruction.chars() {
            inputs.push(c as i128);
        }
        inputs.push(10);
    }

    let res = crate::day5::run_intcode_raw(&mut codes, None, inputs, 0, false, 0);

    if let Some(v) = res.output {
        println!("out is {}", v);
    }
}

pub fn pt2() {
    let mut codes = crate::utils::read_intcodes("./src/day21/input");

    let probe_programes = [
        "OR C T",
        "AND B T",
        "AND A T",
        "NOT T J",
        "OR E T",
        "OR H T",
        "AND T J",
        "AND D J",
        "RUN"
    ];

    let mut inputs = vec![];

    for instruction in probe_programes.iter() {
        for c in instruction.chars() {
            inputs.push(c as i128);
        }
        inputs.push(10);
    }

    let res = crate::day5::run_intcode_raw(&mut codes, None, inputs, 0, false, 0);

    if let Some(v) = res.output {
        println!("out is {}", v);
    }
}