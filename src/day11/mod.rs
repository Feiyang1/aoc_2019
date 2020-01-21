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