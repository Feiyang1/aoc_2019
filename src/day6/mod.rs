use std::cell::RefCell;
use std::collections::HashMap;

pub fn count_orbit() {
    let content = crate::utils::read_file("./src/day6/input");

    let mut map: HashMap<&str, Wrapper> = HashMap::new();
    for pair in content.split("\r\n") {
        let objects: Vec<&str> = pair.split(")").collect();
        let center = objects[0];
        let orbiting = objects[1];

        if map.contains_key(center) {
            map.get_mut(center).unwrap().orbiters.push(orbiting);
        } else {
            map.insert(
                center,
                Wrapper {
                    count: 0,
                    orbiters: vec![orbiting],
                },
            );
        }
    }

    // root nodes. Can be calculated in the previous step
    let mut set = vec!["COM"];
    let mut next_set: Vec<&str> = Vec::new();
    let mut count = 0;
    while set.len() > 0 {
        for center in set.iter() {
            if let Some(centerWrapper) = map.get(center) {
                let center_count = centerWrapper.count;
                let orbiters: Vec<&str> = centerWrapper.orbiters.iter().cloned().collect();
                next_set.extend(orbiters.iter());
                for orbiter in orbiters.iter() {
                    let orbiterWrapperOption = map.get_mut(orbiter);
                    match orbiterWrapperOption {
                        Some(orbiterWrapper) => {
                            orbiterWrapper.count = center_count + 1;
                        }
                        None => {}
                    }
                    count += center_count + 1;
                }
            }
        }

        set = next_set;
        next_set = Vec::new();
    }

    println!("Total number of orbits is {}", count);
}

struct Wrapper<'a> {
    count: u32,
    orbiters: Vec<&'a str>,
}

// Part 2
pub fn steps_to_santa() {
    let content = crate::utils::read_file("./src/day6/input");

    // HashMap is terrible to use!!
    let mut map: HashMap<&str, u32> = HashMap::new();
    let mut items: Vec<Node> = Vec::new();
    for pair in content.split("\r\n") {
        let objects: Vec<&str> = pair.split(")").collect();
        let center = objects[0];
        let orbiting = objects[1];

        if !map.contains_key(center) {
            let node = Node {
                name: center,
                neighbors: Vec::new(),
                visited: false,
                visited_2: false
            };
            items.push(node);
            map.insert(center, (items.len() - 1) as u32);
        };


        if !map.contains_key(orbiting) {
            let node = Node {
                name: orbiting,
                neighbors: Vec::new(),
                visited: false,
                visited_2: false
            };
            items.push(node);
            map.insert(orbiting, (items.len() - 1) as u32);
        };

        let orbiting_node = &mut items[*map.get(orbiting).unwrap() as usize];
        orbiting_node.neighbors.push(*map.get(center).unwrap() as usize);

        let center_node = &mut items[*map.get(center).unwrap() as usize];
        center_node.neighbors.push(*map.get(orbiting).unwrap() as usize);
    }

    let mut santa = vec![*map.get("SAN").unwrap() as usize];
    let mut santa_next: Vec<usize> = Vec::new();
    let mut me = vec![*map.get("YOU").unwrap() as usize];
    let mut me_next: Vec<usize> = Vec::new();
    let mut visitedByMe: HashMap<&str, u32> = HashMap::new();

    println!("santa is at {}, me at {}", santa[0], me[0]);
    let mut round = 0;
    loop {
        println!("round {}", round);

        for me_node_idx in me.iter() {

            let node = &mut items[*me_node_idx];
            if node.visited  {
                continue;
            }
            node.visited = true;

            let node_immu = &items[*me_node_idx];
            visitedByMe.insert(node_immu.name, round);
            me_next.extend(node_immu.neighbors.iter().filter(|x| !items[**x].visited));
        }

        println!("I have visited {} nodes, visiting {} nodes next", visitedByMe.keys().len(), me_next.len());
        me = me_next;
        me_next = Vec::new();

        for santa_node_idx in santa.iter() {
           let santa_node = &mut items[*santa_node_idx];
           if santa_node.visited_2  {
            continue;
            }
           santa_node.visited_2 = true;

           if let Some(steps) = visitedByMe.get(santa_node.name) {
                println!("It takes {} steps to get to santa", steps + round - 2);
                return;
           }

           let santa_node_immu = & items[*santa_node_idx];
           santa_next.extend(santa_node_immu.neighbors.iter().filter(|x| !items[**x].visited_2));
        }
        println!("Santa visiting {} nodes next", santa_next.len());
        santa = santa_next;
        santa_next = Vec::new();

        round += 1;

    }
}

struct Node<'a> {
    name: &'a str,
    visited: bool,
    visited_2: bool,
    neighbors: Vec<usize>,
}
