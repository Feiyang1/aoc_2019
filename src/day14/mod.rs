use std::collections::HashMap;

pub fn cost_for_1_fuel() {
    let mut recipes = read_recipe();
    cost_for_fuel(1, &recipes, HashMap::new());

}

pub fn max_fuel() {
    let content = crate::utils::read_file("./src/day14/input");

    let mut recipes = read_recipe();

    let mut ore_quantity: i64 = 1000000000000;
    let mut fuel_produced = 0;
    let mut batch_size = 1000;
    let mut last_surplus: HashMap<String, u64> = HashMap::new();

    while ore_quantity > 0 && batch_size != 0 {
        let mut last_surplus_copy: HashMap<String, u64> = HashMap::new();

        for (key, val) in &last_surplus {
            last_surplus_copy.insert(String::from(key), *val);
        }

        let cost = cost_for_fuel(batch_size, &recipes, last_surplus_copy);
        

        if ore_quantity - cost.ore_cost as i64 > 0 {            
            ore_quantity -= (cost.ore_cost as i64);
            fuel_produced += batch_size;
            last_surplus = cost.surplus;
            batch_size *= 2;
        } else {
            batch_size /= 2;
        }
    }

    println!("Total fuel {}", fuel_produced);
}

fn cost_for_fuel(quantity: u64, recipes: & HashMap<String, Formula>, mut surplus: HashMap<String, u64>) -> Cost {

    let mut raw_things:Vec<Material> = vec![Material {
        name: String::from("FUEL"),
        quantity
    }];

    let mut ore_needed = 0;

    while raw_things.len() > 0 {

        let mut next_raw_things: HashMap<&str, Material> = HashMap::new();
        for thing in raw_things.iter() {
      //      println!("thing is {}", thing.name);

            if let Some(formula_for_thing) = recipes.get(&thing.name) {
                let mut batches_to_make = thing.quantity / formula_for_thing.quantity;

                let rest = thing.quantity % formula_for_thing.quantity;
                if rest != 0 {
                    if let Some(thing_surplus) = surplus.get(&thing.name[..]) {
                        if (*thing_surplus as i32) - (rest as i32) >= 0 { // fulfill requirement using surplus completely
                            surplus.insert(String::from(&thing.name), thing_surplus - rest);
                        } else { // use up surplus and produce more
                            batches_to_make += 1;
                            let still_need = rest - thing_surplus;
                            let new_surplus = formula_for_thing.quantity - still_need;
                            surplus.insert(String::from(&thing.name), new_surplus);
                        }
                    } else {
                        batches_to_make += 1;
                        surplus.insert(String::from(&thing.name), formula_for_thing.quantity - rest);
                    }
                }
    
                for raw_for_thing in formula_for_thing.materials.iter() {
                    let to_make = batches_to_make * raw_for_thing.quantity;
                    if let Some(t) = next_raw_things.get_mut(&raw_for_thing.name[..]) {
                        t.quantity += to_make;
                    } else {
                        next_raw_things.insert(&raw_for_thing.name[..], Material {
                            name: String::from(&raw_for_thing.name),
                            quantity: to_make
                        });
                    }
                }
            } else { // ORE
           //     println!("Adding {} to ORE count {}", thing.name, thing.quantity);
                ore_needed += thing.quantity;
            }

        }

        raw_things = Vec::new();
        for (_, value) in next_raw_things {
            raw_things.push(value);
        }
    }

   // println!("Need {} OREs ", ore_needed);
    return Cost {
        ore_cost: ore_needed,
        surplus
    };
}

fn read_recipe() -> HashMap<String, Formula>{
    let content = crate::utils::read_file("./src/day14/input");

    let mut recipes: HashMap<String, Formula> = HashMap::new();
    for line in content.split("\r\n") {
        let left_right: Vec<&str> = line.split("=>").collect();
        let output = parse_material(left_right[1]);
        
        let inputs: Vec<Material> = left_right[0].split(",").map(|input| parse_material(input)).collect();

        recipes.insert(output.name, Formula {
            quantity: output.quantity,
            materials: inputs
        });
    }

    return recipes;
}

struct Cost {
    ore_cost: u64,
    surplus: HashMap<String, u64>
}

struct Formula {
    quantity: u64,
    materials: Vec<Material>
}

struct Material {
    name: String,
    quantity: u64,
}

impl std::clone::Clone for Material {
    fn clone(&self) -> Self {
        Material {
            name: self.name.clone(),
            quantity: self.quantity
        }
    }
}

fn parse_material(input: &str) -> Material {
    let mut parsed: Vec<String> = input.split(" ").filter(|thing| *thing != " " && *thing != "").map(|st| String::from(st)).collect();
    Material {
        name: parsed.remove(1),
        quantity: parsed[0].parse::<u64>().unwrap()
    }
}