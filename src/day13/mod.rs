use std::collections::HashMap;
use std::io;
use std::io::Read;

pub fn count_blocks() {
    let content = crate::utils::read_file("./src/day13/input");
    let mut codes: Vec<i128> = content
    .split(",")
    .map(|str_int| str_int.parse::<i128>().unwrap())
    .collect();


    let crate::day5::IntcodeResult { output, resume_point, relative_base, outputs_since_start_or_resume } = crate::day5::run_intcode_raw(&mut codes, None, vec![], 0, false, 0);

    let mut block_map: HashMap<String, bool> = HashMap::new();
    let mut block_count = 0;
    for i in (0..outputs_since_start_or_resume.len()).step_by(3) {
        let x = outputs_since_start_or_resume[i];
        let y = outputs_since_start_or_resume[i + 1];
        let tile_id = outputs_since_start_or_resume[i + 2];

        let key = format!("{}-{}", x, y);

        if tile_id == 2 { // block
            block_map.insert(key, true);
            block_count += 1;
        } else if tile_id == 0 { // empty
            if let Some(exist) = block_map.get(&key) {
                if *exist {
                    block_count -= 1;
                    block_map.remove(&key);
                }
            }
        }
    }

    println!("total {} block tiles", block_count);
}

pub fn highscore() {
    let content = crate::utils::read_file("./src/day13/input");
    let mut codes: Vec<i128> = content
    .split(",")
    .map(|str_int| str_int.parse::<i128>().unwrap())
    .collect();

    // play for free
    codes[0] = 2;
    let mut game_state: HashMap<String, i32> = HashMap::new();
    let mut memory: HashMap<i128, i128> = HashMap::new();
    let mut paddle_x = 0;
    let mut ball_x = 0;

    let crate::day5::IntcodeResult { mut output, mut resume_point, mut relative_base, mut outputs_since_start_or_resume } = crate::day5::run_intcode_raw(&mut codes, Some(&mut memory), vec![], 0, true, 0);

    let mut max_x = 0;
    let mut max_y = 0;
    // init game
    for i in (0..outputs_since_start_or_resume.len()).step_by(3) {
        let x = outputs_since_start_or_resume[i];
        let y = outputs_since_start_or_resume[i + 1];
        let tile_id = outputs_since_start_or_resume[i + 2];

        if x > max_x {
            max_x = x;
        }

        if y > max_y {
            max_y = y;
        }

        let key = format!("{}-{}", x, y);

        game_state.insert(key, tile_id as i32);

        if tile_id == 3 {
            paddle_x = x;
        } else if tile_id == 4 {
            ball_x = x;
        }
    }

    while let Some(rp) = resume_point {

        display(max_x as i32, max_y as i32, &game_state);
        // println!("please input next move");
        // let mut dir = String::new();
        // io::stdin().read_line(&mut dir).expect("Falied to read line");
    
        // let next_input = match &dir[..1] {
        //     "a" => -1,
        //     "s" => 0,
        //     "d" => 1,
        //     _ => panic!("invalid input")
        // };


        let next_input = if ball_x > paddle_x {
            1
        } else if ball_x < paddle_x {
            -1
        } else {
            0
        };
        println!("next move is {}", next_input);

        let result = crate::day5::run_intcode_raw(&mut codes, Some(&mut memory), vec![next_input], rp, true, relative_base);

        outputs_since_start_or_resume = result.outputs_since_start_or_resume;
        resume_point = result.resume_point;
        relative_base = result.relative_base;

     //   println!("next resume point is {}", resume_point.unwrap());

        // update game state 
        for i in (0..outputs_since_start_or_resume.len()).step_by(3) {
            let x = outputs_since_start_or_resume[i];
            let y = outputs_since_start_or_resume[i + 1];
            let tile_id = outputs_since_start_or_resume[i + 2];
    
            let key = format!("{}-{}", x, y);
    
            game_state.insert(key, tile_id as i32);

            if tile_id == 3 {
                paddle_x = x;
            } else if tile_id == 4 {
                ball_x = x;
            }
        }
    }

    println!("the score is {}", outputs_since_start_or_resume[outputs_since_start_or_resume.len() - 1]);
}

fn display(x_d: i32, y_d: i32, object_map: &HashMap<String, i32>) {
    for y in 0..y_d {
        for x in 0..x_d {
            let key = format!("{}-{}", x, y);

            if let Some(obj) = object_map.get(&key) {
                match obj {
                    0 => print!(" "),
                    1 => print!("w"),
                    2 => print!("b"),
                    3 => print!("_"),
                    4 => print!("o"),
                    _ => print!(" ")
                }
            }
        }
        println!();
    }
}