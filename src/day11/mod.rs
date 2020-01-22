use std::collections::HashMap;

pub fn estimate() {
    let content = crate::utils::read_file("./src/day11/input");
    let mut codes: Vec<i128> = content
        .split(",")
        .map(|str_int| str_int.parse::<i128>().unwrap())
        .collect();

    
    let mut grid: HashMap<String, u32> = HashMap::new();

    let mut current_pos = Position {
        direction: Direction::Up,
        x: 0,
        y: 0
    };

    grid.insert(current_pos.get_key(), 0);

    let mut res = crate::day5::run_intcode_raw(&mut codes, None, vec![0], 0, true, 0);

    while let Some(resume_point) = res.resume_point {

        let key = current_pos.get_key();

        let outputs = res.outputs_since_start_or_resume;

        let paint_color = outputs[0];
        let move_direction = outputs[1];

        grid.insert(key, paint_color as u32);

        match current_pos.direction {
            Direction::Up => {
                match move_direction {
                    0 => {
                        current_pos.x -= 1;
                        current_pos.direction = Direction::Left;
                    },
                    1 => {
                        current_pos.x += 1;
                        current_pos.direction = Direction::Right;
                    },
                    _ => panic!("invalid direction")
                }
            },
            Direction::Left => {
                match move_direction {
                    0 => {
                        current_pos.y -= 1;
                        current_pos.direction = Direction::Down;
                    },
                    1 => {
                        current_pos.y += 1;
                        current_pos.direction = Direction::Up;
                    },
                    _ => panic!("invalid direction")
                }
            },
            Direction::Down => {
                match move_direction {
                    0 => {
                        current_pos.x += 1;
                        current_pos.direction = Direction::Right;
                    },
                    1 => {
                        current_pos.x -= 1;
                        current_pos.direction = Direction::Left;
                    },
                    _ => panic!("invalid direction")
                }
            },
            Direction::Right => {
                match move_direction {
                    0 => {
                        current_pos.y += 1;
                        current_pos.direction = Direction::Up;
                    },
                    1 => {
                        current_pos.y -= 1;
                        current_pos.direction = Direction::Down;
                    },
                    _ => panic!("invalid direction")
                }
            }
        }

        let current_paint_color = if let Some(o) = grid.get(&current_pos.get_key()) {
            *o
        } else {
            0
        };

        res = crate::day5::run_intcode_raw(&mut codes, None, vec![current_paint_color as i128], resume_point, true, 0);
    }

    println!("painted {} tiles", grid.len());
}

pub fn paint() {
    let content = crate::utils::read_file("./src/day11/input");
    let mut codes: Vec<i128> = content
        .split(",")
        .map(|str_int| str_int.parse::<i128>().unwrap())
        .collect();

    
    let mut grid: HashMap<String, u32> = HashMap::new();

    let mut current_pos = Position {
        direction: Direction::Up,
        x: 0,
        y: 0
    };

    let mut min_x = 0;
    let mut max_x = 0;
    let mut min_y = 0;
    let mut max_y = 0;

    // starting on a white tile
    grid.insert(current_pos.get_key(), 1);

    let mut mem: HashMap<i128, i128> = HashMap::new();

    let mut res = crate::day5::run_intcode_raw(&mut codes, Some(&mut mem), vec![1], 0, true, 0);

    let mut count = 0;
    while let Some(resume_point) = res.resume_point {
        count += 1;
        let key = current_pos.get_key();

        let outputs = res.outputs_since_start_or_resume;

        let paint_color = outputs[0];
        let move_direction = outputs[1];

        println!("at {} {} dir {}, paint {}, move {}", current_pos.x, current_pos.y, current_pos.direction, paint_color, move_direction);

        grid.insert(key, paint_color as u32);

        if count == 4 {
            // break;
        }

        match current_pos.direction {
            Direction::Up => {
                match move_direction {
                    0 => {
                        current_pos.x -= 1;
                        current_pos.direction = Direction::Left;
                        if current_pos.x < min_x {
                            min_x = current_pos.x;
                        }
                    },
                    1 => {
                        current_pos.x += 1;
                        current_pos.direction = Direction::Right;
                        if current_pos.x > max_x {
                            max_x = current_pos.x;
                        }
                    },
                    _ => panic!("invalid direction")
                }
            },
            Direction::Left => {
                match move_direction {
                    0 => {
                        current_pos.y -= 1;
                        current_pos.direction = Direction::Down;
                        if current_pos.y < min_y {
                            min_y = current_pos.y;
                        }
                    },
                    1 => {
                        current_pos.y += 1;
                        current_pos.direction = Direction::Up;
                        if current_pos.y > max_y {
                            max_y = current_pos.y;
                        }
                    },
                    _ => panic!("invalid direction")
                }
            },
            Direction::Down => {
                match move_direction {
                    0 => {
                        current_pos.x += 1;
                        current_pos.direction = Direction::Right;
                        if current_pos.x > max_x {
                            max_x = current_pos.x;
                        }
                    },
                    1 => {
                        current_pos.x -= 1;
                        current_pos.direction = Direction::Left;
                        if current_pos.x < min_x {
                            min_x = current_pos.x;
                        }
                    },
                    _ => panic!("invalid direction")
                }
            },
            Direction::Right => {
                match move_direction {
                    0 => {
                        current_pos.y += 1;
                        current_pos.direction = Direction::Up;
                        if current_pos.y > max_y {
                            max_y = current_pos.y;
                        }
                    },
                    1 => {
                        current_pos.y -= 1;
                        current_pos.direction = Direction::Down;
                        if current_pos.y < min_y {
                            min_y = current_pos.y;
                        }
                    },
                    _ => panic!("invalid direction")
                }
            }
        }

        let current_paint_color = if let Some(o) = grid.get(&current_pos.get_key()) {
            *o
        } else {
            0
        };

        res = crate::day5::run_intcode_raw(&mut codes, Some(&mut mem), vec![current_paint_color as i128], resume_point, true, res.relative_base);
    }

    for j in (min_y..max_y + 1).rev() {
        
        for i in min_x..max_x + 1 {

            if i == 0 && j == 0 {
                print!("@");
                continue;
            }
          //  println!("y {} x {}", j, i);

            if let Some(color) = grid.get(&format!("{}-{}", i, j)) {
                if *color == 1 {
                    print!("#");
                } else {
                    print!(".");
                }
            } else {
                print!(".");
            }
        }
        println!();
    }
}


struct Position {
    pub direction: Direction,
    pub x: i32,
    pub y: i32
}

impl Position {
    pub fn get_key(&self) -> String {
        format!("{}-{}", self.x, self.y)
    }
}

enum Direction {
    Up,
    Left,
    Down,
    Right
}

impl std::fmt::Display for Direction {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
       write!(f, "{}", match self {
            Direction::Down => "DOWN",
            Direction::Left => "LEFT",
            Direction::Up => "UP",
            Direction::Right => "RIGHT"
        })
    }
}