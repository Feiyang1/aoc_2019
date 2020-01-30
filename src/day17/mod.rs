use std::collections::HashMap;

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