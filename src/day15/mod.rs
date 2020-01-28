use std::collections::{HashMap, HashSet};

pub fn shortest_path() {
    let codes = crate::utils::read_intcodes("./src/day15/input");

    let mut steps = 0;
    let mut found = false;

    let mut save_points: Vec<SavePoint> = Vec::new();
    save_points.push(SavePoint {
        codes,
        mem: HashMap::new(),
        resume_point: 0,
        relative_base: 0,
        x: 0,
        y: 0
    });

    let mut visited: HashSet<String> = HashSet::new();
    visited.insert(key(&save_points[0]));

    let moves = vec!["N", "E", "S", "W"];
    while !found {
        let mut next_save_points: Vec<SavePoint> = Vec::new();

        for sp in save_points.iter() {
            for m in moves.iter() {
                let mut new_sp = sp.clone();

                let input = match m {
                    &"N" => {
                        new_sp.y -= 1;
                        1
                    }, //1
                    &"E" => {
                        new_sp.x += 1;
                        4
                    }, //4
                    &"S" => {
                        new_sp.y += 1;
                        2
                    }, //2
                    &"W" => {
                        new_sp.x -= 1;
                        3
                    }, //3
                    _ => panic!("invalid direction")
                };

                if visited.contains(&key(&new_sp)) { // skip if already visited
                    continue;
                } else {
                    visited.insert(key(&new_sp));
                }

                let result = run_intcode_save_point(&mut new_sp, vec![input], true);

                if let Some(output) = result.output {
                    match output {
                        0 => { // no op
                        },
                        1 => { 
                            new_sp.resume_point = result.resume_point.unwrap();
                            new_sp.relative_base = result.relative_base;
                            next_save_points.push(new_sp);
                        },
                        2 => {
                            found = true;
                        },
                        _ => panic!("invalid output")
                    }
                } else {
                    panic!("invalid output");
                }
            }
        }

        steps += 1;
        save_points = next_save_points;
    }

    println!("found the oxygen system in {} steps", steps);
}

#[derive(Clone)]
struct SavePoint {
    codes: Vec<i128>,
    mem: HashMap<i128, i128>,
    resume_point: usize,
    relative_base: i128,
    x: i128,
    y: i128
}

fn run_intcode_save_point(save_point: &mut SavePoint, inputs: Vec<i128>, stop_on_pending_input: bool) -> crate::day5::IntcodeResult {
    crate::day5::run_intcode_raw(&mut save_point.codes, Some(&mut save_point.mem), inputs, save_point.resume_point, stop_on_pending_input, save_point.relative_base)
}

fn key(save_point: &SavePoint) -> String {
    format!("{}-{}", save_point.x, save_point.y)
}