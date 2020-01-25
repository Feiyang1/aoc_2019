pub fn total_energy() {
    let mut moon_1 = Moon::new(4, 12, 13);
    let mut moon_2 = Moon::new(-9, 14, -3);
    let mut moon_3 = Moon::new(-7, -1, 2);
    let mut moon_4 = Moon::new(-11, 17, -1);

    let mut moons = vec![moon_1, moon_2, moon_3, moon_4];
    let delta = vec![3, 1, -1, -3];
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
}