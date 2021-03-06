
use std::collections::{HashMap, HashSet};

pub fn shortest_path_recursive_maze() {
    let map = construct_map();
    let graph = construct_graph(&map);

    let mut candidates = Vec::<RecursiveCost>::new();
    candidates.push(RecursiveCost {
        portal: String::from("AA"),
        level: 0,
        cost: 0
    });
 
    let mut included = HashSet::<String>::new();
    while true {
        candidates.sort_by(|a, b| a.cost.cmp(&b.cost));
        let take = candidates.remove(0);
        included.insert(format!("{}-{}", take.portal, take.level));
         println!("taking {} at level {} cost {}", take.portal, take.level, take.cost);
        if take.portal == "ZZ" && take.level == 0 {
            println!("shortest path is {}", take.cost);
            return;
        }

        let sister_portal_name = get_sister_portal_name(&take.portal);

        if !graph.contains_key(&take.portal) {
            continue;
        }
        
        let neighbors = graph.get(&take.portal).unwrap();
        for (k, v) in neighbors.iter() {   
           // println!("attempting {}", k);
            if take.level == 0 && k.len() == 2 
            && *k != sister_portal_name
            && *k != "ZZ" { // skip outer labels at level 0
                continue;
            }

            if k == "AA-i" {
                continue;
            }

            if take.level != 0 && (k == "AA" || k == "ZZ") { // skip AA and ZZ at deeper level
                continue;
            }

            let k_level = if *k == sister_portal_name {
            if k.len() == 2 {
                take.level + 1
            } else {
                take.level - 1
            }
            } else {
                take.level
            };

            let k_key = format!("{}-{}", k, k_level);
            if included.contains(&k_key) {
                continue;
            }

            let mut already_candidate = false;
            for c in candidates.iter_mut() {
                if c.portal == *k && c.level == k_level {
                    already_candidate = true;
                    if c.cost > take.cost + v {
                        c.cost = take.cost + v;
                    }
                }
            }

            if !already_candidate {
                candidates.push(RecursiveCost {
                    portal: String::from(k),
                    level: k_level,
                    cost: take.cost + v
                });
            }
        }
    }
}

struct RecursiveCost {
    portal: String,
    level: u32,
    cost: u32
}

pub fn shortest_path() {
   let map = construct_map();
   let graph = construct_graph(&map);

//    for (k,v) in graph.iter() {
//        println!("portal {}", k);
       
//        for (kk, vv) in v.iter() {
//             println!(" to {}, {}", kk, vv);
//        }
//    }

   let mut candidates = Vec::<Cost>::new();
   candidates.push(Cost {
       portal: String::from("AA"),
       cost: 0
   });

   let mut included = HashSet::<String>::new();
   while true {
       candidates.sort_by(|a, b| a.cost.cmp(&b.cost));
       let take = candidates.remove(0);
       included.insert(String::from(&take.portal));
        println!("taking {} cost {}", take.portal, take.cost);
       if take.portal == "ZZ" {
           println!("shortest path is {}", take.cost);
           return;
       }

       for neighbors in graph.get(&take.portal) {
            for (k, v) in neighbors.iter() {
                if included.contains(k) {
                    continue;
                }

                let mut already_candidate = false;
                for c in candidates.iter_mut() {
                    if c.portal == *k {
                        already_candidate = true;
                        if c.cost > take.cost + v {
                            c.cost = take.cost + v;
                        }
                    }
                }

                if !already_candidate {
                    candidates.push(Cost {
                        portal: String::from(k),
                        cost: take.cost + v
                    });
                }
            }
       }
   }
}

struct Cost {
    portal: String,
    cost: u32
}

fn construct_graph(map: & Vec<Vec<String>>) -> HashMap<String, HashMap<String, u32>> {
    let mut graph = HashMap::new();
    for (r_idx, row) in map.iter().enumerate() {
        for (c_idx, item) in row.iter().enumerate() {
            if item.len() == 2 { // is portal
             //   println!("portal {} {}", r_idx, c_idx);
                add_neighbors(map, r_idx, c_idx, &mut graph);
            }
        }
    }

    return graph;
}

fn add_neighbors(
    map: & Vec<Vec<String>>, 
    r_idx: usize,
    c_idx: usize,
    graph: &mut HashMap<String, HashMap<String, u32>>
) {
    let map_w = map[0].len();
    let map_h = map.len();
    let my_key = format!("{}-{}", r_idx, c_idx);
    let my_name = get_portal_name(map, r_idx, c_idx);
    let sister_portal_name = get_sister_portal_name(&my_name);

    let mut temp = HashMap::new();
    temp.insert(sister_portal_name, 1);
    graph.insert(String::from(&my_name), temp);
    
    let mut visited = HashSet::<String>::new();
    visited.insert(my_key);
    let dirs = [(-1, 0), (1, 0), (0, 1), (0, -1)];
    let mut to_explore = Vec::<(usize, usize)>::new();

    for d in dirs.iter() {
        let next_idx = (r_idx as i32 + d.0, c_idx as i32 + d.1);

        if next_idx.0 >= 0 && next_idx.0 < map_h as i32 
        && next_idx.1 >=0 && next_idx.1 < map_w as i32 
        && !visited.contains(&format!("{}-{}", next_idx.0, next_idx.1)){
            to_explore.push((next_idx.0 as usize, next_idx.1 as usize));
        }
    }

    let mut steps = 0;
    while to_explore.len() > 0 {
        steps += 1;
        let mut to_explore_next = Vec::new();

        for (r, c) in to_explore.iter() {
         //   println!("exploring {} {}", r, c);
            let thing = &map[*r][*c];
            visited.insert(format!("{}-{}", *r, *c));

            if thing != "." && thing.len() == 1 {
                continue;
            } else if thing ==  "." {
                for d in dirs.iter() {
                    let next_idx = (*r as i32 + d.0, *c as i32 + d.1);
            
                    if next_idx.0 >= 0 && next_idx.0 < map_h as i32 
                    && next_idx.1 >=0 && next_idx.1 < map_w as i32 
                    && !visited.contains(&format!("{}-{}", next_idx.0, next_idx.1)){
                        to_explore_next.push((next_idx.0 as usize, next_idx.1 as usize));
                    }
                }
            } else { // portal
                let to_portal_name = get_portal_name(map, *r, *c);
                if let Some(v) = graph.get_mut(&my_name) {
                    if !v.contains_key(&to_portal_name) {
                        v.insert(to_portal_name, steps);
                    }
                } else {
                    let mut temp = HashMap::new();
                    temp.insert(to_portal_name, steps);

                    graph.insert(String::from(&my_name), temp);
                }
            }
        }

        to_explore = to_explore_next;
    }
}

fn get_portal_name(
    map: & Vec<Vec<String>>, 
    r_idx: usize,
    c_idx: usize
) -> String {
    let raw_name = &map[r_idx][c_idx];
    if is_outer_portal(map, r_idx, c_idx) {
        return String::from(raw_name);
    } else {
        return format!("{}-i", raw_name);
    }
}

// hard coded
fn is_outer_portal(
    map: & Vec<Vec<String>>,
    r_idx: usize,
    c_idx: usize
) -> bool {
    let h = map.len();
    let w = map[0].len();

    r_idx == 2 || r_idx == h - 3 || c_idx == 2 || c_idx == w - 3  
}

fn get_sister_portal_name(portal_name: &String) -> String {
    if portal_name.len() == 2 {
        return format!("{}-i", portal_name);
    } else {
        return String::from(&portal_name[0..2]);
    }
}

fn construct_map() -> Vec<Vec<String>> {
    let raw_map = crate::utils::read_file("./src/day20/input");

    let mut map = Vec::<Vec<char>>::new();
    for row in raw_map.split("\r\n") {
       let mut row_vec = Vec::new();
        for c in row.chars() {
            row_vec.push(c);
        }

        map.push(row_vec);
    }

    let mut map_str = Vec::<Vec<String>>::new();
    for (r_idx, row) in map.iter().enumerate() {
        let mut row_vec = Vec::new();
        for (c_idx, col) in row.iter().enumerate() {
            if let Some(v) = is_portal(&map, r_idx, c_idx) {
                row_vec.push(v);
            } else {
                row_vec.push(format!("{}", col));
            }
        }
        map_str.push(row_vec);
    }

    return map_str;
}

fn is_portal(map: &Vec<Vec<char>>, r_idx: usize, c_idx: usize) -> Option<String> {
    
    if map[r_idx][c_idx] != '.' {
        return None;
    }
    // left
    if let Some(v) = two_letters(map, r_idx, c_idx, (0, -1)) {
        return Some(v);
    }

    // down
    if let Some(v) = two_letters(map, r_idx, c_idx, (1, 0)) {
        return Some(v);
    }

    // right
    if let Some(v) = two_letters(map, r_idx, c_idx, (0, 1)) {
        return Some(v);
    }

    // up
    if let Some(v) = two_letters(map, r_idx, c_idx, (-1, 0)) {
        return Some(v);
    }

    None
}

fn two_letters(map: &Vec<Vec<char>>, r_idx: usize, c_idx: usize, delta: (i32, i32)) -> Option<String> {
    let h = map.len();
    let w = map[0].len();
    
    let (first, second) = if delta.0 < 0 {
        ((r_idx as i32 + 2*delta.0, c_idx as i32), (r_idx as i32 + delta.0, c_idx as i32))
    } else if delta.0 > 0 {
        ((r_idx as i32 + delta.0, c_idx as i32), (r_idx as i32 + 2*delta.0, c_idx as i32))
    } else if delta.1 < 0 {
        ((r_idx as i32, c_idx as i32 + 2*delta.1), (r_idx as i32, c_idx as i32 + delta.1))
    } else if delta.1 > 0 {
        ((r_idx as i32, c_idx as i32 + delta.1), (r_idx as i32, c_idx as i32 + delta.1*2))
    } else {
        panic!("impossible!");
    };

    if first.0 < 0 || first.0 >= h as i32 || first.1 < 0 || first.1 >= w as i32
    || second.0 < 0 || second.0 >= h as i32 || second.1 < 0 || second.1 >= w as i32 {
        return None;
    }

    let first_char = map[first.0 as usize][first.1 as usize];
    let second_char = map[second.0 as usize][second.1 as usize];

    if first_char as u32 >= 65 && second_char as u32 >= 65 {
        return Some(format!("{}{}", first_char, second_char));
    } else {
        return None;
    }

}