use std::fmt;

pub fn pt1() {
    let list = shuffle(10007, 1);
    
    for (idx, i) in list.iter().enumerate() {
        if *i == 2019 {
            println!("card 2019 is at {}", idx);
            return;
        }
    }
}

pub fn pt2() {
    let list = shuffle(119315717514047, 101741582076661);
    for (idx, i) in list.iter().enumerate() {
        if *i == 2020 {
            println!("card 2019 is at {}", idx);
            return;
        }
    }
}

pub fn shuffle(cards: u128, rounds: u128) -> Vec<u128> {
    let commands_raw = crate::utils::read_file("./src/day22/input");
    let commands: Vec<Command> = commands_raw.split("\r\n").map(|st| {
        let words: Vec<&str> = st.split(" ").collect();
        if st == "deal into new stack" {
            Command::Stack
        } else if words[0] == "cut" {
            Command::Cut(words[1].parse::<i32>().unwrap())
        } else {
            Command::Increment(words[3].parse::<u32>().unwrap())
        }
    }).collect();

    let mut list = Vec::new();
    println!("initializing deck");
    for i in 0..cards {
        list.push(i);
    }

    for r in 0..rounds {
        println!("round {}", r);
        for c in commands.iter() {
            match c {
                Command::Increment(i) => {
                    list = increment(&list, *i);
                },
                Command::Cut(i) => {
                    list = cut(&list, *i);
                },
                Command::Stack => stack(&mut list)
            }
        }
    }

    return list;
}

fn stack(list: &mut Vec<u128>) {
    list.reverse();
}

fn cut(list:& Vec<u128>, size: i32) -> Vec<u128> {
    let size = if size > 0 {
        size
    } else {
        list.len() as i32 + size
    };

    let mut l: Vec<u128> = list[size as usize..].iter().map(|x| *x).clone().collect();
    l.extend(list[..size as usize].iter().clone());

    return l;
}

fn increment(list: &Vec<u128>, inc: u32) -> Vec<u128> {
    let len = list.len();
    let mut new_list: Vec<u128> = list.iter().map(|x| *x).clone().collect();

    let mut cur_idx = 0;
    for i in list.iter() {
        new_list[cur_idx] = *i;
        cur_idx = (cur_idx + inc as usize) % len;
    }

    return new_list;
}


enum Command {
    Increment(u32),
    Cut(i32),
    Stack
}


impl fmt::Display for Command {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let thing = match self {
            Command::Increment(i) => format!("Increment {}", i),
            Command::Cut(i) => format!("Cut {}", i),
            Command::Stack => format!("Stack")
        };

        write!(f, "{}", thing)
    }
}
