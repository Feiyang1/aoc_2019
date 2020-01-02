pub fn run_intcodes(arg1: u32, arg2: u32) -> u32 {
    let content = crate::utils::readFile("./src/day2/input");
    let mut arr: Vec<u32> = content.split(",").map(|str_int| str_int.parse::<u32>().unwrap()).collect();
    arr[1] = arg1;
    arr[2] = arg2;
    let mut index = 0;

    while arr[index] != 99 {
        let op = arr[index];
        let arg1 = arr[arr[index + 1] as usize];
        let arg2 = arr[arr[index + 2] as usize];
        let output_index = arr[index + 3] as usize;

        arr[output_index] = match op {
            1 => {
                arg1 + arg2
            },
            2 => {
                arg1*arg2
            },
            _ => {
                panic!(format!("I don't understand op {}", op))
            }
        };

        index += 4;
    }

    // let mut counter = 0;
    // arr.iter().map(|val| {
    //     println!("{}: {}", counter, val);
    //     counter += 1;
    // }).collect::<Vec<_>>();

    arr[0] 
}

pub fn find_pair() {
    for arg1 in 0..99 {
        for arg2 in 0..99 {
            if run_intcodes(arg1, arg2) == 19690720 {
                return println!("found arg1 {} and arg2 {}", arg1, arg2);
            }
        }
    }
}