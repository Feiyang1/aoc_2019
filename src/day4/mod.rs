pub fn count_possible_numbers() {
    let mut count = 0;

    for i in 1..6 {     
        count_recursive(&i.to_string()[..], i, 1, &mut count, false);
    }

    println!("there are {} possible numbers", count);
}

fn count_recursive(prefix: &str, last_digit: u32, streak: u32, count: &mut u32, dup: bool) {

    if prefix.len() == 6 {
        let number = prefix.parse::<u32>().unwrap();
        if number >= 109165 && number <=  576723 && (dup || streak == 2) {
            *count += 1;
            println!("found match {}", number);
        }
        return;
    }

    // remaining digits after this digit
    let remaining_digits = 6 - prefix.len() - 1;
    let mut scale = 1;
    for _ in 0..remaining_digits {
        scale *= 10;
    }

    let mut i = last_digit;
    while i <= 9 {
        let next_prefix = format!("{}{}", prefix, i);

        let this_num = next_prefix.parse::<u32>().unwrap() * scale;

        if this_num > 576723 {
            return;
        }

        let next_streak = if i == last_digit {
            streak + 1
        } else {
            1
        };

        //count_recursive(&next_prefix[..], i, next_streak, count, dup || i == last_digit);
        
        // PART 2
        count_recursive(&next_prefix[..], i, next_streak, count, dup || (streak == 2 && i != last_digit));

        i += 1;
    }
}