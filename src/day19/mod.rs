use std::collections::HashMap;

pub fn count_areas() {
    let mut codes = crate::utils::read_intcodes("./src/day19/input");

    let mut count = 0;
    for r in 0..100 {
        for c in 0..100 {
            let res = crate::day5::run_intcode_raw(&mut codes.clone(), None, vec![r, c], 0, false, 0);

            if let Some(v) = res.output {
                if v == 1 {
                    count += 1;
                    print!("#");
                } else {
                    print!(".");
                }
            }
        }
        println!();
    }

    println!("tractor beam affects {} areas", count);
}

// TODO - perf improvement, binary search on r
pub fn fit_santa() {
    let codes = crate::utils::read_intcodes("./src/day19/input");

    let mut r = 200;
    let mut c = 0;
    let mut cache_r = HashMap::<String, u32>::new();
    let mut cache_c = HashMap::<String, u32>::new();

    let mut affected_found = false;
    while true {
        println!("{} - {}", r, c);
        let res = crate::day5::run_intcode_raw(&mut codes.clone(), None, vec![r, c], 0, false, 0);

        if let Some(v) = res.output {
            if v == 1 {
                affected_found = true;
                let w = right(& codes, r, c, &mut cache_r);
                let h = down(& codes, r, c, &mut cache_c);
                println!("w {} h {}", w, h);
                if w >= 100 && h >= 100 {
                    println!("found pos {} {} - {}", r, c, 10000*r + c);
                    return;
                } else {
                    c += 1; // go to right
                    continue;
                }
            } else {
                if affected_found { // move to next line
                    r += 1;
                    c = 0;
                    affected_found = false;
                } else { // move to right on the same line because we haven't found the first affected area.
                    c += 1; 
                }
            }
        }
    }
}

fn right(codes: &Vec<i128>, r: i128, c: i128, cache: &mut HashMap<String, u32>,) -> u32 {
    recurse_one_dir(codes, r, c, cache, (0, 1))
}

fn down(codes: &Vec<i128>, r: i128, c: i128, cache: &mut HashMap<String, u32>) -> u32 {
    recurse_one_dir(codes, r, c, cache, (1, 0))
}

fn recurse_one_dir(codes: &Vec<i128>, r: i128, c: i128, cache: &mut HashMap<String, u32>, mov: (u32, u32)) -> u32 {
    let key = format!("{}-{}", r, c);
    if let Some(v) = cache.get(&key) {
        return *v;
    }

    let res = crate::day5::run_intcode_raw(&mut codes.clone(), None, vec![r, c], 0, false, 0);

    if let Some(v) = res.output {
        if v == 0 {
            return 0;
        }

        // next
        let count = recurse_one_dir(codes, r + mov.0 as i128, c + mov.1 as i128, cache, mov) + 1;

        cache.insert(key, count);
        return count;
    } else {
        panic!("impossible output!");
    }

}