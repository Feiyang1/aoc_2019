
pub fn calc() {
    // PART 1
    let contents = crate::utils::read_file("src/day1/input");
    let total_fuel: i32 = contents
        .split("\r\n")
        .map(|line| line.parse::<i32>().unwrap())
        .map(|mass| mass / 3 - 2)
        .fold(0, |sum, fuel| sum + fuel);

    println!("part1 - total fuel required is {}", total_fuel);

    // PART 2
    let total_fuel_1: i32 = contents
        .split("\r\n")
        .map(|line| line.parse::<i32>().unwrap())
        .map(|mass| calc_recursive(mass))
        .fold(0, |sum, fuel| sum + fuel);
    println!("part2 - total fuel required is {}", total_fuel_1);
}

fn calc_recursive(mass: i32) -> i32 {
    let fuel_mass = mass / 3 - 2;
    if fuel_mass <= 0 {
        0
    } else {
        fuel_mass + calc_recursive(fuel_mass)
    }
}
