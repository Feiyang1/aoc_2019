pub fn total_energy() {
    let mut moon_1 = Moon::new(4, 12, 13);
    let mut moon_2 = Moon::new(-9, 14, -3);
    let mut moon_3 = Moon::new(-7, -1, 2);
    let mut moon_4 = Moon::new(-11, 17, -1);

    let mut moons = vec![moon_1, moon_2, moon_3, moon_4];
    // 1000 steps
    for _ in 0..1000 {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                // v_x
                if moons[i].p_x > moons[j].p_x {
                    moons[i].v_x -= 1;
                    moons[j].v_x += 1;
                } else if moons[i].p_x < moons[j].p_x {
                    moons[i].v_x += 1;
                    moons[j].v_x -= 1;
                } else { // moons[i].p_x == moons[j].p_x
                    // no op
                }

                // v_y
                if moons[i].p_y > moons[j].p_y {
                    moons[i].v_y -= 1;
                    moons[j].v_y += 1;
                } else if moons[i].p_y < moons[j].p_y {
                    moons[i].v_y += 1;
                    moons[j].v_y -= 1;
                } else { // moons[i].p_y == moons[j].p_y
                    // no op
                }

                // v_z
                if moons[i].p_z > moons[j].p_z {
                    moons[i].v_z -= 1;
                    moons[j].v_z += 1;
                } else if moons[i].p_z < moons[j].p_z {
                    moons[i].v_z += 1;
                    moons[j].v_z -= 1;
                } else { // moons[i].p_z == moons[j].p_z
                    // no op
                }
            }
        }


        // update position
        for moon in moons.iter_mut() {
            moon.p_x += moon.v_x;
            moon.p_y += moon.v_y;
            moon.p_z += moon.v_z;
        }
    }

    let mut total = 0;
    for moon in moons.iter() {
        total += moon.energy();
    }

    println!("total energy is {}", total);
}

pub fn steps_before_repeating() {
    let init_moons = vec![Moon::new(4, 12, 13), Moon::new(-9, 14, -3), Moon::new(-7, -1, 2), Moon::new(-11, 17, -1)];

    let mut moon_1 = Moon::new(4, 12, 13);
    let mut moon_2 = Moon::new(-9, 14, -3);
    let mut moon_3 = Moon::new(-7, -1, 2);
    let mut moon_4 = Moon::new(-11, 17, -1);

    let mut moons = vec![moon_1, moon_2, moon_3, moon_4];

    let mut repeated_x = false;
    let mut repeated_y = false;
    let mut repeated_z = false;

    // x, y, z
    let mut repeat_factor = vec![0;3];
    let mut steps = 0;
    while !repeated_x || !repeated_y || !repeated_z {
        for i in 0..moons.len() {
            for j in i + 1..moons.len() {
                // v_x
                if moons[i].p_x > moons[j].p_x {
                    moons[i].v_x -= 1;
                    moons[j].v_x += 1;
                } else if moons[i].p_x < moons[j].p_x {
                    moons[i].v_x += 1;
                    moons[j].v_x -= 1;
                } else { // moons[i].p_x == moons[j].p_x
                    // no op
                }

                // v_y
                if moons[i].p_y > moons[j].p_y {
                    moons[i].v_y -= 1;
                    moons[j].v_y += 1;
                } else if moons[i].p_y < moons[j].p_y {
                    moons[i].v_y += 1;
                    moons[j].v_y -= 1;
                } else { // moons[i].p_y == moons[j].p_y
                    // no op
                }

                // v_z
                if moons[i].p_z > moons[j].p_z {
                    moons[i].v_z -= 1;
                    moons[j].v_z += 1;
                } else if moons[i].p_z < moons[j].p_z {
                    moons[i].v_z += 1;
                    moons[j].v_z -= 1;
                } else { // moons[i].p_z == moons[j].p_z
                    // no op
                }
            }
        }


        // update position
        for moon in moons.iter_mut() {
            moon.p_x += moon.v_x;
            moon.p_y += moon.v_y;
            moon.p_z += moon.v_z;
        }

        steps += 1;


        if at_start_x(&moons, &init_moons) && !repeated_x {
            repeat_factor[0] = steps;
            repeated_x = true;
        }

        if at_start_y(&moons, &init_moons) && !repeated_y {
            println!("y repeat at {}", steps);
            repeat_factor[1] = steps;
            repeated_y = true;
        }

        if at_start_z(&moons, &init_moons) && !repeated_z {
            repeat_factor[2] = steps;
            repeated_z = true;
        }
    }
    
    println!("{} {} {}", repeat_factor[0], repeat_factor[1], repeat_factor[2]);
    // find the least common multiple of x, y, z repeat factors
    println!("Need {} steps to repeat", lcm(repeat_factor[0], lcm(repeat_factor[1], repeat_factor[2])));
}

fn at_start_x(cur_vec: &Vec<Moon>, init_vec: &Vec<Moon>) -> bool {

    for i in 0..cur_vec.len() {
        if cur_vec[i].p_x != init_vec[i].p_x 
        || cur_vec[i].v_x != init_vec[i].v_x {
            return false;
        }
    }

    true
}

fn at_start_y(cur_vec: &Vec<Moon>, init_vec: &Vec<Moon>) -> bool {
    for i in 0..cur_vec.len() {
        if cur_vec[i].p_y != init_vec[i].p_y 
        || cur_vec[i].v_y != init_vec[i].v_y {
            return false;
        }
    }

    true
}

fn at_start_z(cur_vec: &Vec<Moon>, init_vec: &Vec<Moon>) -> bool {
    for i in 0..cur_vec.len() {
        if cur_vec[i].p_z != init_vec[i].p_z 
        || cur_vec[i].v_z != init_vec[i].v_z {
            return false;
        }
    }

    true
}

fn gcd(x: u128, y: u128) -> u128 {
    let mut x = x;
    let mut y = y;
    while y != 0 {
        let t = y;
        y = x % y;
        x = t;
    }
    x
}

fn lcm(a: u128, b: u128) -> u128 {
    a * b / gcd(a, b)
}

struct Moon {
    p_x: i32,
    p_y: i32,
    p_z: i32,
    v_x: i32,
    v_y: i32,
    v_z: i32
}

impl Moon {
    fn new(p_x: i32, p_y: i32, p_z: i32) -> Moon {
        Moon {
            p_x,
            p_y,
            p_z,
            v_x: 0,
            v_y: 0,
            v_z: 0
        }
    }

    pub fn energy(&self) -> i128 {
        let p_e = self.p_x.abs() + self.p_y.abs() + self.p_z.abs();
        let v_e = self.v_x.abs() + self.v_y.abs() + self.v_z.abs();

        p_e as i128 * v_e as i128
    }

    pub fn eq(&self, another: &Moon) -> bool {
        self.p_x == another.p_x && self.p_y == another.p_y && self.p_z == another.p_z
        && self.v_x == another.v_x && self.v_y == another.v_y && self.v_z == another.v_z 
    }
}