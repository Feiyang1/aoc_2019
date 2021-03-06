pub fn checksum() {
    let content = crate::utils::read_file("./src/day8/input");
    let width = 25;
    let height = 6;
    let digits_per_layer = width * height;

    let mut digits_processed_in_this_layer = 0;

    let mut layer_least_0: [u32; 10] = [0; 10];
    let mut this_layer: [u32; 10] = [0; 10];

    let mut isFirstLayer = true;
    for c in content.chars() {

        let num = c.to_digit(10).unwrap();
        this_layer[num as usize] += 1;

        digits_processed_in_this_layer += 1;

        // parse next layer
        if digits_processed_in_this_layer == digits_per_layer {
            digits_processed_in_this_layer = 0;

            if isFirstLayer {
                layer_least_0 = this_layer;
                isFirstLayer = false;
            } else if this_layer[0] < layer_least_0[0] {
                layer_least_0 = this_layer;
            }

            this_layer = [0; 10];
        }
    }

    println!("num of 1 * num of 2 is {}", layer_least_0[1] * layer_least_0[2]);
}

pub fn decode() {
    let content = crate::utils::read_file("./src/day8/input");
    let width = 25;
    let height = 6;
    let digits_per_layer = width * height;

    let mut message = vec![2; digits_per_layer];
    let mut digits_processed = 0;
    for c in content.chars() {
        let idx = digits_processed % digits_per_layer;
        
        if message[idx] == 2 {
            message[idx] = c.to_digit(10).unwrap();
        }

        digits_processed += 1;
    }

    let mut count = 0;
    for d in message.iter() {
        print!("{}", d);
        count += 1;

        if count == width {
            println!("");
            count = 0;
        }
    }

}