use std::collections::HashSet;

pub fn calc_alignment() {
    let mut codes = crate::utils::read_intcodes("./src/day17/input");

    let outputs = crate::day5::run_intcode_raw(&mut codes, None, vec![], 0, false, 0);

    let mut width: i128 = 0;
    let mut width_found = false;

    let mut alignment = 0;
    let mut x: i128 = 0;
    let mut y: i128 = 0;
    for (idx, o) in outputs.outputs_since_start_or_resume.iter().enumerate() {
        if !width_found {
            width += 1;
        }

        if *o == 10 {
            width_found = true;
        }

        if width_found {
            x = idx as i128 % width;
            y = idx as i128 / width;

            if *o == 35 && is_intersection(&outputs.outputs_since_start_or_resume, width, (x, y)) {
                alignment += x*y;
            }
        }

        x += 1;
        y += 1;
        print!("{}", std::char::from_u32(*o as u32).unwrap());
    }

    println!("alignment parameter is {}", alignment);
}

pub fn traverse() {
    let mut codes = crate::utils::read_intcodes("./src/day17/input");
    let mut codes_2 = codes.clone();

    // get map
    let outputs = crate::day5::run_intcode_raw(&mut codes, None, vec![], 0, false, 0);
    
    let mut width = 0;
    for o in outputs.outputs_since_start_or_resume.iter() {
        width += 1;

        if *o == 10 {
            break;
        }
    }

    let map = Map {
        data: outputs.outputs_since_start_or_resume,
        width
    };

    let scaffold_count = map.data.iter().filter(|x| **x == 35).count();
    // find vaccum robot
    let robot_coord = map.index_to_coordinate(map.data.iter().position(|x| *x == 94).unwrap() as u32);
    let mut r = RobotState {
        x: robot_coord.0,
        y: robot_coord.1,
        direction: "n"
    };


    let mut visited = HashSet::<u32>::new();
    let mut commands = Vec::<String>::new();
    

    // find path
    let mut moved_distance = 0;
    while visited.len() < scaffold_count {
        let m = next_move(&r, &map);
        match m {
            Move::Forward(d) => {
                let moved_to = move_in_direction(&r, r.direction);
                moved_distance += d;
                visited.insert(map.coordinate_to_index(moved_to.0 as u32, moved_to.1 as u32).unwrap());
                r.move_to(moved_to.0 as u32, moved_to.1 as u32);
                println!("move to {} {}", moved_to.0, moved_to.1);
            },
            Move::Turn(d) => {
                if moved_distance > 0 {
                    commands.push(format!("{}", moved_distance));
                    moved_distance = 0;
                }
                commands.push(String::from(d));
                match d {
                    "L" => r.turn_left(),
                    "R" => r.turn_right(),
                    _ => panic!("invalid d")
                }
                println!("turn {}", d);
            }
        }
    }

    if moved_distance > 0 {
        commands.push(format!("{}", moved_distance));
    }

    for c in commands.iter() {
        print!("{},", c);
    }
    
    println!("len {}", commands.len());

    let mut patterns: Vec<Pattern> = vec![];
    let names = ["A", "B", "C"];

    let mut avail_ranges = vec![(0 as u32, (commands.len() - 1) as u32)];

    while avail_ranges.len() > 0 {

        for ar in avail_ranges.iter() {
            println!("available range {}-{}", ar.0, ar.1);
        }    

        println!("############################################");

        let candidate = avail_ranges[0];
        let mut len = candidate.1 - candidate.0 + 1;

        if len > 20 {
            len = 20;
        }

        let mut avail_next: Vec<(u32, u32)> = vec![];
        while len > 0 {
            let matches = find_matches(&commands[candidate.0 as usize..(candidate.0 + len) as usize], &commands, &avail_ranges);

            if matches.len() > 1 {

                // update available ranges
                for (lower, higher) in avail_ranges.iter() {

                    let mut c_l = *lower;
                    let mut c_h = *higher;

                    let mut last_slot = None;
                    let mut no_divide = true;

                    for m in matches.iter() {
                        let l = *m;
                        let h = l + len - 1;

                        if l > c_l && h <= c_h{
                            avail_next.push((c_l, l - 1));
                            no_divide = false;
                        } 
                        
                        if h < c_h && l >= c_l {
                            last_slot = Some((h + 1, c_h));
                            c_l = h + 1;
                            no_divide = false;
                        } 
                        
                        if h == c_h && l == c_l {
                            no_divide = false;
                        }
                    }

                    if let Some(thing) = last_slot {
                        avail_next.push(thing);
                    }

                    if no_divide {
                        avail_next.push((*lower, *higher));
                    }
                }

                println!("found a pattern {} {}", len, matches.len());
                patterns.push(Pattern {
                    name: String::from(names[patterns.len()]),
                    len,
                    starting_points: matches
                });

                break;
            }

            len -= 1;
        }

        avail_ranges = avail_next;
    }

    println!("");
    for p in patterns.iter() {
        println!("{}, {}", p.name, p.len);
    }

    codes_2[0] = 2;

    // construct intcode inputs
    
    let mut helper_arr = vec![""; commands.len()];
    for p in patterns.iter() {
        for idx in p.starting_points.iter() {
            helper_arr[*idx as usize] = &p.name[..];
        }
    }

    for ha in helper_arr.iter() {
        print!("{}", ha);
    }

    let mut call_seq: Vec<String> = helper_arr.into_iter().filter(|thing| *thing != "").map(|t| String::from(t)).collect();
    let mut inputs: Vec<i128> = Vec::new();
    construct_input(&call_seq[..], &mut inputs);

   // A
    let a_start = patterns[0].starting_points[0];
    let a = &commands[a_start as usize..(a_start + patterns[0].len) as usize];

    construct_input(&a, &mut inputs);

   // B
   let b_start = patterns[1].starting_points[0];
   let b = &commands[b_start as usize..(b_start + patterns[1].len) as usize];
   construct_input(&b, &mut inputs);
   
   // C
   let c_start = patterns[2].starting_points[0];
   let c = &commands[c_start as usize..(c_start + patterns[2].len) as usize];
   construct_input(&c, &mut inputs);


   inputs.push('n' as i128);
   inputs.push(10);

   for ip in inputs.iter() {
       println!("iiiii {}", ip);
   }

   let res = crate::day5::run_intcode_raw(&mut codes_2, None, inputs, 0, false, 0);

   println!("has {}", res.output.unwrap());

   if let Some(rp) = res.resume_point {
       println!("has resume point");
   }
}

fn construct_input(commands: &[String], inputs: &mut Vec<i128>) {
    for (idx, cmd) in commands.iter().enumerate() {

        for c in cmd.chars() {
            println!("cmd is {}", c);
            inputs.push(c as i128);
        }

        if idx < commands.len() - 1 {
            inputs.push(44); // comma
        }
    }

    inputs.push(10); // newline
}

fn find_matches(p: &[String], strg: &Vec<String>, ranges: &Vec<(u32, u32)>) -> Vec<u32> {
    let mut matches: Vec<u32> = vec![];
    for (lower, higher) in ranges.iter() {

        let mut i = *lower;
        
        while i < *higher + 1  {
            
            let mut out_of_bound = false;
            let mut found_match = true;
            
            for (count, c) in p.iter().enumerate() {

                if i + count as u32 > *higher {
                    out_of_bound = true;
                    break;
                }

                if *c != strg[i as usize + count] {
                    found_match = false;
                    break;
                }

            }

            if !out_of_bound && found_match {
                matches.push(i);
                i += p.len() as u32;  
            } else {
                i += 1
            }
        }
    }

    return matches;
}

fn next_move(robot: &RobotState, map: &Map) -> Move {

    // move forward
    let move_to = move_in_direction(robot, robot.direction);
    if let Some(thing) = map.get(move_to.0, move_to.1) {
        if  thing == 35 {
            return Move::Forward(1);
        }
    }

    let dirs = ["n", "w", "s", "e"];
    let r_d = dirs.iter().position(|x| *x == robot.direction).unwrap();
    
    // turn left
    let next_dir = dirs[(r_d + 1) % 4];
    let move_to_after_turn = move_in_direction(robot, next_dir);

    if let Some(thing) = map.get(move_to_after_turn.0, move_to_after_turn.1) {
        if thing == 35 {
            return Move::Turn("L");
        }
    }
    
    // turn right
    let next_dir = dirs[(r_d + 3) % 4];
    let move_to_after_turn = move_in_direction(robot, next_dir);

    if let Some(thing) = map.get(move_to_after_turn.0, move_to_after_turn.1) {
        if thing == 35 {
            return Move::Turn("R");
        }
    }

    return Move::Turn("L");
}

fn move_in_direction(robot: &RobotState, d: &str) -> (i32, i32) {
    match d {
        "n" => {
            (robot.x as i32, robot.y as i32 - 1)
        },
        "w" => {
            (robot.x as i32 - 1, robot.y as i32)
        },
        "s" => {
            (robot.x as i32, robot.y as i32 + 1)
        },
        "e" => {
            (robot.x as i32 + 1, robot.y as i32)
        },
        _ => panic!("invalid direction")
    }
}

fn is_intersection(map: &Vec<i128>, width: i128, coordinate: (i128, i128)) -> bool {
    let (x, y) = coordinate;
    let idx = y * width + x;

    if x == 0 || x == width - 1 { // edge node can't be an intersection
        return false;
    }
    let w_idx = (idx - 1) as usize;
    let e_idx = (idx + 1) as usize;
    let n_idx = (idx - width) as usize;
    let s_idx = (idx + width) as usize;

    if n_idx < 0 || s_idx as usize >= map.len() {
        return false;
    }

    map[w_idx] == 35 && map[e_idx] == 35 && map[n_idx] == 35 && map[s_idx] == 35 
}

struct RobotState {
    direction: &'static str,
    x: u32,
    y: u32
}

impl RobotState {
    fn turn_left(&mut self) {
        let dirs = ["n", "w", "s", "e"];
        let r_d = dirs.iter().position(|x| *x == self.direction).unwrap();
        self.direction = dirs[(r_d + 1) % 4];
    }

    fn turn_right(&mut self) {
        let dirs = ["n", "w", "s", "e"];
        let r_d = dirs.iter().position(|x| *x == self.direction).unwrap();
        self.direction = dirs[(r_d + 3) % 4];
    }

    fn move_to(&mut self, x: u32, y: u32) {
        self.x = x;
        self.y = y;
    }
}

struct Map {
    data: Vec<i128>,
    width: u32
}

impl Map {
    fn get(&self, x: i32, y: i32) -> Option<i128> {
        if x >= self.width as i32 || x < 0 || y < 0 {
            return None;
        }

        let index = y * self.width as i32 + x;

        if index >= self.data.len() as i32{
            return None;
        }

        Some(self.data[index as usize])
    }

    fn index_to_coordinate(&self, index: u32) -> (u32, u32) {
        (index % self.width, index / self.width)
    }

    fn coordinate_to_index(&self, x: u32, y: u32) -> Option<u32> {
        if x >= self.width {
            return None;
        }

        let index = y * self.width + x;

        if index >= self.data.len() as u32{
            return None;
        }

        return Some(index);
    }
}

enum Move {
    Turn(&'static str),
    Forward(u32)
}

struct Pattern {
    name: String,
    len: u32,
    starting_points: Vec<u32>
}