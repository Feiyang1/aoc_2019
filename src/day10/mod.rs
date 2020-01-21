use std::collections::{HashSet, HashMap};
use std::cmp::Ordering;

pub fn max_visibility() -> (usize, usize) {
    let content = crate::utils::read_file("./src/day10/input");

    let matrix: Vec<Vec<&str>> = content.split("\r\n").map(|row| row.split("").filter(|item| *item != "").collect()).collect();
    
    let mut visible_count = vec![vec![0; matrix[0].len()]; matrix.len()];

    let mut max_visible = 0;
    let mut max_i = 0;
    let mut max_j = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {

            if matrix[i][j] != "#" {
                continue;
            }

            // println!("at {} {}", i, j);
            // calculate visible asteroids from position i,j
            let mut blocked_angles: HashSet<String> = HashSet::new();


            for j_same_line in j+1..matrix[i].len() {
                if matrix[i][j_same_line] == "#" {
                    visible_count[i][j] += 1;
                    if visible_count[i][j] > max_visible { max_visible = visible_count[i][j]; max_i = i; max_j = j; }
                    
                    visible_count[i][j_same_line] += 1;
                    if  visible_count[i][j_same_line] > max_visible { max_visible =  visible_count[i][j_same_line]; max_i = i; max_j = j_same_line;}

                    break;
                }
            }

            for i_inner in i + 1..matrix.len() {
                for j_inner in 0..matrix[i_inner].len() {
                  //  println!("at inner {} {}", i_inner, j_inner);
                    if matrix[i_inner][j_inner] == "#" {
                       // println!("before subtract {} {}", );

                        let w = i_inner as i32 - i as i32;
                        let h = j_inner as i32 - j as i32;
                        let gcd_ = gcd(w.abs() as usize, h.abs() as usize) as i32;
                        
                        let key = format!("{}-{}", w/gcd_, h/gcd_);

                        if !blocked_angles.contains(&key) {
                            visible_count[i][j] += 1;
                            if visible_count[i][j] > max_visible { max_visible = visible_count[i][j]; max_i = i; max_j = j; }

                            visible_count[i_inner][j_inner] += 1;
                            if visible_count[i_inner][j_inner] > max_visible { max_visible = visible_count[i_inner][j_inner]; max_i = i_inner; max_j = j_inner;}

                            blocked_angles.insert(key);
                        }
                    }
                }
            }
        }
    }

    println!("max visible asteroids {}", max_visible);

    for row in visible_count.iter() {
        for item in row.iter() {
           print!("{},", item);
        }

        println!();
    }

    return (max_i, max_j);
}


fn gcd(a: usize, b: usize) -> usize
{ 
    // Everything divides 0  
    if (a == 0) {return b; }
       
    if (b == 0) {return a; }
   
    // base case 
    if (a == b) {
        return a; 
    }
   
    // a is greater 
    if (a > b) {
        return gcd(a-b, b); 
    }
    return gcd(a, b-a); 
} 

pub fn destroy() {
    let content = crate::utils::read_file("./src/day10/input");

    let matrix: Vec<Vec<&str>> = content.split("\r\n").map(|row| row.split("").filter(|item| *item != "").collect()).collect();

    let (s_i, s_j) = max_visibility();

    let mut target_in_line_map: HashMap<String, TargetInLine> = HashMap::new();

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {
            // skip asteroid where the station is
            if i == s_i && j == s_j || matrix[i][j] != "#" {
                continue;
            }

            let i_delta = (i as i32) - (s_i as i32);
            let j_delta = (j as i32) - (s_j as i32);

            let gcd_ = gcd(i_delta.abs() as usize, j_delta.abs() as usize) as i32;

            let i_delta_simple = i_delta/gcd_;
            let j_delta_simple = j_delta/gcd_;

            let key = format!("{}-{}", i_delta_simple, j_delta_simple);

            if i == 1 && j == 8 {
                println!("come on {} {} {}, key {}, i {} j {}", i_delta, j_delta, gcd_, key, i, j);
            }

            match target_in_line_map.get_mut(&key) {
                Some(result) => {
                    result.targets.push((i, j));
                },
                None => {
                    target_in_line_map.insert(key, TargetInLine {
                        i_delta: i_delta_simple,
                        j_delta: j_delta_simple,
                        targets: vec![(i, j)]
                    });
                }
            }
        }
    }

    let mut target_in_line_vec: Vec<&mut TargetInLine> = target_in_line_map.values_mut().collect();

    target_in_line_vec.sort_by(|a, b| {
        if in_top_right(a) && (in_bottom_right(b) || in_bottom_left(b) || in_top_left(b))
            || in_bottom_right(a) && (in_bottom_left(b) || in_top_left(b)) 
            || in_bottom_left(a) && in_top_left(b)
        {
            return Ordering::Less;
        } else if in_bottom_right(a) && in_top_right(b)
            || in_bottom_left(a) && (in_bottom_right(b) || in_top_right(b))
            || in_top_left(a) && (in_bottom_left(b) || in_bottom_right(b) || in_top_right(b))
        {
            return Ordering::Greater;
        } else { // a, b are in the same region

            if in_top_right(a) || in_bottom_left(a){
                if a.j_delta == 0 {
                    return Ordering::Less;
                }

                if b.j_delta == 0 {
                    return Ordering::Greater;
                }

                let tan_a = a.i_delta.abs() as f64 / a.j_delta.abs() as f64;
                let tan_b = b.i_delta.abs() as f64 / b.j_delta.abs() as f64;

                if  tan_a > tan_b {
                    return Ordering::Less;
                } else {
                    return Ordering::Greater;
                }
            }  else { // top left or bottom right
                let tan_a = a.i_delta.abs() as f64 / a.j_delta.abs() as f64;
                let tan_b = b.i_delta.abs() as f64 / b.j_delta.abs() as f64;

                if  tan_a > tan_b {
                    return Ordering::Greater;
                } else {
                    return Ordering::Less;
                }
            }
        }
    });

    let mut count = 0;
    let target_count = 200;


    for item in target_in_line_vec.iter_mut() {

        println!("x_d {} y_d {} len {}", item.j_delta, item.i_delta, item.targets.len());
        if item.targets.len() == 0 {
            continue;
        }

        count += 1;
        let destroyed = if in_top_right(item) || in_top_left(item) {
            item.targets.remove(item.targets.len() - 1)
        } else { // in bottom
            item.targets.remove(0)
        };
        println!("x_d: {} y_d: {}, destroyed {} {}", item.j_delta, item.i_delta, destroyed.0, destroyed.1);


        if count == target_count {
            println!("{} destroyed at x: {} y: {}. result: {}", target_count, destroyed.0, destroyed.1, 100 * destroyed.1 + destroyed.0);
            return;
        }
    }
}

fn in_top_right(a: &TargetInLine) -> bool {
    a.j_delta >= 0 && a.i_delta < 0
}

fn in_bottom_right(a: &TargetInLine) -> bool {
    a.j_delta > 0 && a.i_delta >= 0
}

fn in_bottom_left(a: &TargetInLine) -> bool {
    a.j_delta <= 0 && a.i_delta > 0
}

fn in_top_left(a: &TargetInLine) -> bool {
    a.j_delta < 0 && a.i_delta <= 0
}

struct TargetInLine {
    i_delta: i32,
    j_delta: i32,
    targets: Vec<(usize, usize)>
}