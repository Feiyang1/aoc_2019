use std::collections::{HashMap, HashSet};

pub fn shortest_path() {

    let map = construct_map();

    let mut graph = construct_graph(&map);

    for (k, v) in graph.iter() {
        println!("char {}", k);

        for (k, v) in v.iter() {
            println!("go to {}, distance {}", k, v);
        }
    }

    let entrance_neighbors = graph.remove(&'@').unwrap();
    
    let mut least_steps = 999999999;

    let mut cache = HashMap::<String, u32>::new();

    for (to, distance) in entrance_neighbors.iter() {
        if *to as u32 >= 97 { // key
            let mut graph_copy = graph.clone();
            let steps = shortest_path_recurse(graph_copy, *to, &mut cache);
            println!("this path takes {} steps", steps + distance);
            if (steps + distance) < least_steps {
                least_steps = steps + distance;
            }
        }
    }

    println!("it takes least {} steps", least_steps);
}

fn shortest_path_recurse(mut graph: HashMap<char, HashMap<char, u32>>, at_key: char, cache: &mut HashMap<String, u32>) -> u32 {
  //  println!("{} things left", graph.len());

    let mut cache_key =  format!("{}", at_key);
    let mut kk = graph.keys().map(|k| *k).collect::<Vec<char>>();
    kk.sort();
    for k in kk {
        cache_key.push('-');
        cache_key.push(k);
    }

    if let Some(res) = cache.get(&cache_key) {
       // println!("use cache {}", cache_key);
        return *res;
    }

    let gate_key = (at_key as u8 - 32) as char;
    let gate_paths = remove_from_graph(&mut graph, gate_key);
    let key_paths = remove_from_graph(&mut graph, at_key);

    if graph.len() == 0 {
        println!("complete!");
        return 0;
    }

    let mut least_steps = 999999999;
    for (to, distance) in key_paths.iter() {
        if *to as u32 >= 97 { // key
            let mut graph_copy = graph.clone();
            let steps = shortest_path_recurse(graph_copy, *to, cache);
            if (steps + distance) < least_steps {
                least_steps = steps + distance;
            }
        }
    }

    cache.insert(cache_key, least_steps);
    return least_steps;
}

fn remove_from_graph(graph: &mut HashMap<char, HashMap<char, u32>>, key: char) -> HashMap<char, u32> {

    if !graph.contains_key(&key) {
        println!("removing non existing key {}", key);
        return HashMap::new();
    }

   // println!("removing {}", key);
    let key_paths = graph.remove(&key).unwrap();

    for (k, _) in key_paths.iter() {
     //   println!("....... {}", key);
        let neighor = graph.get_mut(&k).unwrap();
        neighor.remove(&key);
    }

    let adjacent_keys = key_paths.keys().collect::<Vec<&char>>();
    for i in 0..adjacent_keys.len() {
        for j in (i+1)..adjacent_keys.len() {
            let key_i = adjacent_keys[i];
            let key_j = adjacent_keys[j];

            let distance_i_j = key_paths.get(&key_i).unwrap() + key_paths.get(&key_j).unwrap();
            let point1 = graph.get_mut(&key_i).unwrap();

            if let Some(d) = point1.get(key_j) {
                if distance_i_j < *d {
                    point1.insert(*key_j, distance_i_j);
                }
            } else {
                point1.insert(*key_j, distance_i_j);
            }

            let point2 = graph.get_mut(&key_j).unwrap();

            if let Some(d) = point2.get(key_i) {
                if distance_i_j < *d {
                    point2.insert(*key_i, distance_i_j);
                }
            } else {
                point2.insert(*key_i, distance_i_j);
            }
        }
    }

    return key_paths;
}

fn construct_map() -> Vec<Vec<char>> {
    let content = crate::utils::read_file("./src/day18/input");

    let height = content.split("\r\n").count();
    let width = content.split("\r\n").collect::<Vec<&str>>()[0].len();


    let mut map = vec![vec!['#'; width]; height];

    for (row, line) in content.split("\r\n").enumerate() {
        for (col, c) in line.chars().enumerate() {
            map[row][col] = c;
        }
    }

    // for r in map.iter() {
    //     for c in r.iter() {
    //         print!("{}", c);
    //     }

    //     println!();
    // }


    return map;
}

fn construct_graph(map: &Vec<Vec<char>>) -> HashMap<char, HashMap<char, u32>> {
    let mut graph = HashMap::new();

    for (r_i, row) in map.iter().enumerate() {
        for (c_i, c) in row.iter().enumerate() {
            if *c != '#' && *c != '.' {
                add_neighbors(map, r_i, c_i, &mut graph);
            }
        }
    }

    return graph;
}

fn add_neighbors(
    map: &Vec<Vec<char>>, 
    row: usize, 
    col: usize, 
    graph: &mut HashMap<char, HashMap<char, u32>>
) {
    let mut visited = HashSet::new();
    let source_cr = map[row][col];
    visited.insert(format!("{}-{}", row, col));

    let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];
    let mut last = vec![(row as i32, col as i32)];
    let mut distance = 0;
  //  println!("mapping {}", source_cr);
    while last.len() > 0 {
        distance += 1;
      //  println!("distance {}",  distance);
        let mut next = Vec::<(i32, i32)>::new();
        
        for n in last.iter() {
            for d in dirs.iter() {
                let r = n.0 + d.0;
                let c = n.1 + d.1;
                
                if r < 0 || c < 0 || r >= map.len() as i32 || c >= map[0].len() as i32 {
                    continue;
                }
              
                let cr = map[r as usize][c as usize];
              //  println!("next is {} {} {}", r, c, cr);
                if cr == '#' {
                    continue;
                }

                let key = format!("{}-{}", r, c);
                if !visited.contains(&key) {

                    visited.insert(key);
                    

                    if cr != '.' && cr != '@' { // either a gate or a key
                        if let Some(thing) = graph.get_mut(&source_cr) {
                
                            if let Some(d) = thing.get(&cr) {
                                if distance < *d {
                                    thing.insert(cr, distance);
                                }
                            } else {
                                thing.insert(cr, distance);
                            }
                        } else {
                            let mut hm = HashMap::new();
                            hm.insert(cr, distance);
                            graph.insert(source_cr, hm);
                        }
                    } else {
                        next.push((r, c));
                    }
                }
            }
        }
     //   println!("next {}", next.len());
        last = next;
    }
}

#[derive(Clone)]
struct Path {
    to: char,
    distance: u32
}

