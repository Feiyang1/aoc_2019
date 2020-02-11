use std::collections::{HashMap, HashSet};

pub fn bug_score() {
    let bugs_raw = crate::utils::read_file("./src/day24/input");
    
    let mut bugs: Vec<bool> = Vec::new();
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    for line in bugs_raw.split("\r\n") {
        height += 1;
        width = line.len() as u32;
        for c in line.chars() {
            if c == '#' {
                bugs.push(true)
            } else {
                bugs.push(false)
            }
        }
    }

    let mut map = BugMap {
        map: bugs,
        width,
        height
    };

    let mut cache: HashSet<u64> = HashSet::new();
    while true {
        let score = map.score();

        if cache.contains(&score) {
            println!("the biodiversity score is {}", score);
            return;
        } else {
            cache.insert(score);
        }

        map.next();
    }
}

struct BugMap {
    map: Vec<bool>,
    width: u32,
    height: u32
}

impl BugMap {
    fn score(&self) -> u64 {
        let base: u64 = 2;
        let mut score: u64 = 0;
        for (idx, b) in self.map.iter().enumerate() {
            if *b {
                score += base.pow(idx as u32);
            }
        }

        score
    }

    fn next(&mut self) {
        let mut next_map = self.map.clone();

        for (idx, b) in self.map.iter().enumerate() {
            next_map[idx] = self.next_state(idx as u32);
        }

        self.map = next_map;
    }

    fn next_state(&self, idx: u32) -> bool {
        let neighbors: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
        let r = idx / self.width;
        let c = idx % self.width;

        let mut neighbor_bug_count = 0;
        for n in neighbors.iter() {
            let n_r = n.0 + r as i32;
            let n_c = n.1 + c as i32;

            if self.get(n_r, n_c) {
                neighbor_bug_count += 1;
            }
        }

        if self.map[idx as usize] {
            if neighbor_bug_count == 1 {
                true
            } else {
                false
            }
        } else {
            if neighbor_bug_count == 1 || neighbor_bug_count == 2{
                true
            } else {
                false
            }
        }

    }

    fn get(&self, r: i32, c: i32) -> bool {
        if r >= self.height as i32 || c >= self.width as i32 || r < 0 || c < 0 {
            return false;
        } 

        self.map[(r*self.width as i32 + c) as usize]
    }
}

pub fn recursive_bug_score() {
    let bugs_raw = crate::utils::read_file("./src/day24/input");
    
    let mut bugs: Vec<u32> = Vec::new();
    let mut width: u32 = 0;
    let mut height: u32 = 0;
    for line in bugs_raw.split("\r\n") {
        height += 1;
        width = line.len() as u32;
        for c in line.chars() {
            if c == '#' {
                bugs.push(1)
            } else {
                bugs.push(0)
            }
        }
    }

    let mut hashmap = HashMap::new();
    hashmap.insert(0, bugs);
    let mut map = RBugMap {
        maps: hashmap,
        width,
        height
    };

    for m in 0..200 {
        println!("min {}", m);
        map.next();
    }

    println!("{} bugs", map.bug_count());
}

struct RBugMap {
    maps: HashMap<i32, Vec<u32>>,
    width: u32,
    height: u32
}

impl RBugMap {

    fn bug_count(&self) -> u32 {
        let mut count = 0;
        for v in self.maps.values() {
            for b in v.iter() {
                count += b;
            }
        }

        count
    }

    fn next(&mut self) {

        let mut new_maps: HashMap<i32, Vec<u32>> = HashMap::new();
        let mut queue: Vec<i32> = self.maps.keys().map(|x| *x).collect();

        while queue.len() > 0 {
            let lvl = queue.pop().unwrap();
            let mut map: Vec<u32> = vec![0; 25];
           
            for idx in 0..25 {

                if idx == 12 {
                    continue;
                }

                let cur_state = if let Some(m) = self.maps.get(&lvl) {
                    m[idx]
                } else {
                    0
                };

              //  println!("calc {}", idx);
                map[idx] = self.next_state(lvl, idx as u32, cur_state);
            }

            if map.iter().filter(|x| **x == 1).collect::<Vec<&u32>>().len() > 0 {
                if !self.maps.contains_key(&(lvl + 1)) {
                    queue.push(lvl + 1);
                }
    
                if !self.maps.contains_key(&(lvl - 1)) {
                    queue.push(lvl - 1);
                }
            }

            new_maps.insert(lvl, map);
        }

        self.maps = new_maps;
    }

    fn next_state(&self, level: i32, idx: u32, cur_state: u32) -> u32 {
        let neighbors: [(i32, i32); 4] = [(1, 0), (-1, 0), (0, -1), (0, 1)];
        let r = idx / self.width;
        let c = idx % self.width;

        let mut neighbor_bug_count = 0;
        for n in neighbors.iter() {
            let n_r = n.0 + r as i32;
            let n_c = n.1 + c as i32;
            neighbor_bug_count += self.get_bug_count(level, n_r, n_c, r, c);
        }

        if cur_state == 1 {
            if neighbor_bug_count == 1 {
                1
            } else {
                0
            }
        } else {
            if neighbor_bug_count == 1 || neighbor_bug_count == 2 {
                1
            } else {
                0
            }
        }

    }

    fn get_bug_count(&self, lvl: i32, r: i32, c: i32, from_r: u32, from_c: u32) -> u32 {
     //   println!("counting bugs at {}", lvl);
        let idx = r*self.width as i32 + c;
        let from_idx = from_r * self.width + from_c;

        if r >= self.height as i32 {
            let level = lvl - 1;
            if let Some(m) = self.maps.get(&level) {
                m[17]
            } else {
                0
            }
        } else if  c >= self.width as i32 {
            let level = lvl - 1;
            if let Some(m) = self.maps.get(&level) {
                m[13]
            } else {
                0
            }
        } else if  r < 0 {
            let level = lvl - 1;
            if let Some(m) = self.maps.get(&level) {
                m[7]
            } else {
                0
            }
        } else if c < 0 {
            let level = lvl - 1;
            if let Some(m) = self.maps.get(&level) {
                m[11]
            } else {
                0
            }
        } else if idx == 12 {
           let level = lvl + 1;

           if let Some(m) = self.maps.get(&level) {
                if from_idx == 7 {
                    m[0] + m[1] + m[2] + m[3] + m[4]
                } else if from_idx == 11 {
                    m[0] + m[5] + m[10] + m[15] + m[20]
                } else if from_idx == 17 {
                    m[20] + m[21] + m[22] + m[23] + m[24]
                } else { // 13
                    m[4] + m[9] + m[14] + m[19] + m[24]
                }
           } else {
               0
           }
        } else {
            if let Some(m) = self.maps.get(&lvl) {
                m[idx as usize]
            } else {
                0
            }
        }
    }
}