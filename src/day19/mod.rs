pub fn count_areas() {
    let mut codes = crate::utils::read_intcodes("./src/day19/input");

    let mut count = 0;
    for r in 0..50 {
        for c in 0..50 {
            let res = crate::day5::run_intcode_raw(&mut codes.clone(), None, vec![r, c], 0, false, 0);

            if let Some(v) = res.output {
                if v == 1 {
                    count += 1;
                }
            }
        }
    }

    println!("tractor beam affects {} areas", count);
}