use std::collections::HashSet;

pub fn max_visibility() {
    let content = crate::utils::read_file("./src/day10/input");

    let matrix: Vec<Vec<&str>> = content.split("\r\n").map(|row| row.split("").filter(|item| *item != "").collect()).collect();

    for row in matrix.iter() {
        for item in row.iter() {
           print!("{}", item);
        }

        println!();
    }
    
    let mut visible_count = vec![vec![0; matrix[0].len()]; matrix.len()];

    let mut max_visible = 0;

    for i in 0..matrix.len() {
        for j in 0..matrix[i].len() {

            if matrix[i][j] != "#" {
                continue;
            }

            println!("at {} {}", i, j);
            // calculate visible asteroids from position i,j
            let mut blocked_angles: HashSet<String> = HashSet::new();


            for j_same_line in j+1..matrix[i].len() {
                if matrix[i][j_same_line] == "#" {
                    visible_count[i][j] += 1;
                    if visible_count[i][j] > max_visible { max_visible = visible_count[i][j]; }
                    
                    visible_count[i][j_same_line] += 1;
                    if  visible_count[i][j_same_line] > max_visible { max_visible =  visible_count[i][j_same_line]; }

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
                        let gcd_ = gcd(w.abs(), h.abs());
                        
                        let key = format!("{}-{}", w/gcd_, h/gcd_);

                        if !blocked_angles.contains(&key) {
                            visible_count[i][j] += 1;
                            if visible_count[i][j] > max_visible { max_visible = visible_count[i][j]; }

                            visible_count[i_inner][j_inner] += 1;
                            if visible_count[i_inner][j_inner] > max_visible { max_visible = visible_count[i_inner][j_inner]; }

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
}


fn gcd(a: i32, b: i32) -> i32
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