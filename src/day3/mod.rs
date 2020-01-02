use std::fmt;

pub fn find_nearest() -> i32 {
    let content = crate::utils::readFile("./src/day3/input");
    let path_arrays: Vec<&str> = content.split("\r\n").collect();

    let path1: Vec<&str> = path_arrays[0].split(",").collect();
    let path2: Vec<&str> = path_arrays[1].split(",").collect();

    let mut horizontals: Vec<Line> = Vec::new();
    let mut verticals: Vec<Line> = Vec::new();
    let mut current: (i32, i32, i32) = (0, 0, 0);
    for item in path1.iter() {
        let direction = parse_direction(item);
        match direction {
            Direction::L(i) => {
                let go_to = current.0 - i;
                current.2 += i;
                horizontals.push(Line {
                    anchor: current.1, 
                    start: go_to, 
                    end: current.0, 
                    steps: current.2,
                    direction
                });
                current.0 = go_to;
            },
            Direction::R(i) => {
                let go_to = current.0 + i;
                current.2 += i;
                horizontals.push(Line {direction, anchor: current.1, start: current.0, end: go_to, steps: current.2 });
                current.0 = go_to;
            },
            Direction::U(i) => {
                let go_to = current.1 + i;
                current.2 += i;
                verticals.push(Line {direction, anchor: current.0, start: current.1, end: go_to, steps: current.2});
                current.1 = go_to;
            },
            Direction::D(i) => {
                let go_to = current.1 - i;
                current.2 += i;
                verticals.push(Line {direction, anchor: current.0, start: go_to, end: current.1, steps: current.2});
                current.1 = go_to;
            }
        };
    }

    current = (0, 0, 0);
    let mut min = Option::None;
    for item in path2.iter() {
        let direction = parse_direction(item);
        let mut result;
        match direction {
            Direction::L(i) => {
                let go_to = current.0 - i;
                current.2 += i;
                let line = Line {direction, anchor: current.1, start: go_to, end: current.0, steps: current.2};
               // result = find_intersection_min(line, &verticals);
                result = find_intersection_min_step(line, &verticals);
                current.0 = go_to;
            },
            Direction::R(i) => {
                let go_to = current.0 + i;
                current.2 += i;
                let line = Line {direction, anchor: current.1, start: current.0, end: go_to, steps: current.2};
               // result = find_intersection_min(line, &verticals);
                result = find_intersection_min_step(line, &verticals);
                current.0 = go_to;
            },
            Direction::U(i) => {
                let go_to = current.1 + i;
                current.2 += i;
                let line = Line {direction, anchor: current.0, start: current.1, end: go_to, steps: current.2};
               // result = find_intersection_min(line, &horizontals);
                result = find_intersection_min_step(line, &horizontals);
                current.1 = go_to;
            },
            Direction::D(i) => {
                let go_to = current.1 - i;
                current.2 += i;
                let line = Line {direction, anchor: current.0, start: go_to, end: current.1, steps: current.2};
               // result = find_intersection_min(line, &horizontals);
                result = find_intersection_min_step(line, &horizontals);
                current.1 = go_to;
            }
        };

        min = match result {
            Some(i) => match min {
                            Some(cur_min) => {
                                if i < cur_min {
                                    Some(i)
                                } else {
                                    min
                                }
                            },
                            None => Some(i)
                }
            ,
            None => min
        }
    }

    println!("the min distance is {}", min.unwrap());
    return min.unwrap();
}

fn find_intersection_min(me: Line, orthogonals: &Vec<Line>) -> Option<i32> {
    let mut min = Option::None;
    for line in orthogonals.iter() {
        if me.start <= line.anchor && me.end >= line.anchor
        && line.start <= me.anchor && line.end >= me.anchor {
            let distance = i32::abs(me.anchor) + i32::abs(line.anchor);
            min = match min {
                Some(i) => {
                    if distance < i {
                        Some(distance)
                    } else {
                        Some(i)
                    }
                },
                None => Some(distance)
            };
        }
    }

    return min;
}

fn find_intersection_min_step(me: Line, orthogonals: &Vec<Line>) -> Option<i32> {
    let mut min = Option::None;
    for line in orthogonals.iter() {
        // TODO: the wires leaving the central port at (0,0) should not be considered
        if me.start <= line.anchor && me.end >= line.anchor
        && line.start <= me.anchor && line.end >= me.anchor {
            let me_steps = me.steps - match me.direction {
                Direction::L(i) => line.anchor - me.start,
                Direction::R(i) => me.end - line.anchor,
                Direction::U(i) => me.end - line.anchor,
                Direction::D(i) => line.anchor - me.start
            };

            let line_steps = line.steps - match line.direction {
                Direction::L(i) => me.anchor - line.start,
                Direction::R(i) => line.end - me.anchor,
                Direction::U(i) => line.end - me.anchor,
                Direction::D(i) => me.anchor - line.start
            };

            let total_steps = me_steps + line_steps;

            // println!("found intersection {} + {} = {}", me_steps, line_steps, total_steps);
            // println!("directions: {} and {}", me.direction, line.direction);

            min = match min {
                Some(i) => {
                    if total_steps < i {
                        Some(total_steps)
                    } else {
                        min
                    }
                },
                None => Some(total_steps)
            };
        }
    }

    return min;
}

fn parse_direction(raw_dir: &str) -> Direction {
    let strstr = String::from(raw_dir);
    let dir = &strstr[0..1];
    let length = strstr[1..].parse::<i32>().unwrap();

    match dir {
        "L" => Direction::L(length),
        "R" => Direction::R(length),
        "U" => Direction::U(length),
        "D" => Direction::D(length),
        _ => panic!(format!("unknown string {}", dir))
    }
}

enum Direction {
    L(i32),
    R(i32),
    U(i32),
    D(i32)
}

impl fmt::Display for Direction {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let (dir_type, length) = match self {
            Direction::L(i) => ("L", i),
            Direction::R(i) => ("R", i),
            Direction::U(i) => ("U", i),
            Direction::D(i) => ("D", i)
        };

        write!(f, "({}, {})", dir_type, length)
    }
}

struct Line {
    anchor: i32,
    start: i32,
    end: i32,
    // cumulative steps from the origin
    steps: i32,
    direction: Direction
}