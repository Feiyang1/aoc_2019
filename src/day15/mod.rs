use std::collections::{HashMap, HashSet};

pub fn shortest_path() {
    explore(&mut HashMap::new(), true);
}

pub fn fill_oxygen() {
    let mut map: HashMap<String, u32> = HashMap::new();
    let (x, y) = explore(&mut map, false);

    let mut to_spread = vec![(x, y)];
    let mut minutes = 0;
    let mut spreaded: HashSet<String> = HashSet::new();

    let dirs = vec![(0, 1), (1, 0), (0, -1), (-1, 0)];
    while !to_spread.is_empty() {

        let mut to_spread_next: Vec<(i128, i128)> = Vec::new();
        for source in to_spread.iter() {
            for dir in dirs.iter() {
                let x = source.0 + dir.0;
                let y = source.1 + dir.1;

                if spreaded.contains(&key2((x, y))) {
                    continue;
                }

                if let Some(t) = map.get(&key2((x, y))) {
                    if *t == 1 { // space
                        to_spread_next.push((x, y));
                        spreaded.insert(key2((x, y)));
                    }
                }
            }
        }

        minutes += 1;
        to_spread = to_spread_next;
    }

    println!("spead to entire area in {} mins", minutes - 1);
}

fn explore(map: &mut HashMap<String, u32>, stop_on_oxygen_system: bool) -> (i128, i128) { // return x, y of the oxygen system
    let codes = crate::utils::read_intcodes("./src/day15/input");

    let mut steps = 0;
    let mut oxygen_system_coordinate = (0,0);

    let mut save_points: Vec<SavePoint> = Vec::new();
    save_points.push(SavePoint {
        codes,
        mem: HashMap::new(),
        resume_point: 0,
        relative_base: 0,
        x: 0,
        y: 0
    });

    map.insert(key(&save_points[0]), 1);

    let moves = vec!["N", "E", "S", "W"];
    while !save_points.is_empty() {
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

                if map.contains_key(&key(&new_sp)) { // skip if already visited
                    continue;
                }

                let result = run_intcode_save_point(&mut new_sp, vec![input], true);

                if let Some(output) = result.output {
                    match output {
                        0 => { // no op
                            map.insert(key(&new_sp), 0);
                        },
                        1 => {
                            map.insert(key(&new_sp), 1);
                            new_sp.resume_point = result.resume_point.unwrap();
                            new_sp.relative_base = result.relative_base;
                            next_save_points.push(new_sp);
                        },
                        2 => {
                            map.insert(key(&new_sp), 2);
                            oxygen_system_coordinate = (new_sp.x, new_sp.y);
                            println!("found the oxygen system in {} steps", steps + 1);

                            if stop_on_oxygen_system {
                                return (new_sp.x, new_sp.y);
                            }
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

    println!("area mapped!");

    return oxygen_system_coordinate;
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
    key2((save_point.x, save_point.y))
}

fn key2(point: (i128, i128)) -> String {
    format!("{}-{}", point.0, point.1)
}