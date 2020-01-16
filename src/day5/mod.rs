use std::io;
use std::fmt;

struct Op {
instruction: Instruction,
    params: [Option<Parameter>; 3],
}

struct Parameter {
    value: i32,
    mode: ParameterMode,
}

enum ParameterMode {
    Position,
    Immediate,
}

enum Instruction {
    Add,
    Multiply,
    Input,
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equal
}

enum Result {
    Jump(i32),
    Output(i32)
}

impl Op {
    fn run(&self, memory: &mut Vec<i32>) -> Option<Result> {
        match self.instruction {
            Instruction::Add => {
                let p1 = self.params[0].as_ref().unwrap().get_value(memory);
                let p2 = self.params[1].as_ref().unwrap().get_value(memory);
                let p3 = self.params[2].as_ref().unwrap().value;

                memory[p3 as usize] = p1 + p2;
            },
            Instruction::Multiply => {
                let p1 = self.params[0].as_ref().unwrap().get_value(memory);
                let p2 = self.params[1].as_ref().unwrap().get_value(memory);
                let p3 = self.params[2].as_ref().unwrap().value;

                memory[p3 as usize] = p1*p2;
            },
            Instruction::Input => {
                let mut input = String::new();
                println!("please enter a code");
                io::stdin().read_line(&mut input).expect("Falied to read line");
        
                let input: i32 = match input.trim().parse() {
                    Ok(num) => num,
                    Err(_) => panic!(format!("{} is not a number", input))
                };

                let p1 = self.params[0].as_ref().unwrap().value;

                memory[p1 as usize] = input;
            },
            Instruction::Output => {
                let p1 = self.params[0].as_ref().unwrap().get_value(memory);
                println!("The output is {}", p1);
                return Some(Result::Output(p1));
            },
            Instruction::JumpTrue => {
                let p1 = self.params[0].as_ref().unwrap().get_value(memory);

                if p1 > 0 {
                    let p2 = self.params[1].as_ref().unwrap().get_value(memory);
                    return Some(Result::Jump(p2));
                }
            },
            Instruction::JumpFalse => {
                let p1 = self.params[0].as_ref().unwrap().get_value(memory);

                if p1 == 0 {
                    let p2 = self.params[1].as_ref().unwrap().get_value(memory);
                    return Some(Result::Jump(p2));
                }
            },
            Instruction::LessThan => {
                let p1 = self.params[0].as_ref().unwrap().get_value(memory);
                let p2 = self.params[1].as_ref().unwrap().get_value(memory);
                let p3 = self.params[2].as_ref().unwrap().value;

                memory[p3 as usize] = if p1 < p2 { 1 } else { 0 };
            },
            Instruction::Equal => {
                let p1 = self.params[0].as_ref().unwrap().get_value(memory);
                let p2 = self.params[1].as_ref().unwrap().get_value(memory);
                let p3 = self.params[2].as_ref().unwrap().value;

                memory[p3 as usize] = if p1 == p2 { 1 } else { 0 };
            }
        };

        return None;
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self.instruction {
            Instruction::Add => "Add",
            Instruction::Multiply => "Multiply",
            Instruction::Input => "Input",
            Instruction::Output => "Output",
            Instruction::JumpFalse => "JumpFalse",
            Instruction::JumpTrue => "JumpTrue",
            Instruction::LessThan => "LessThan",
            Instruction::Equal => "Equal"
        };

        let p1 = match &self.params[0] {
            Some(i) => "Some",
            None => "None"
        };

        write!(f, "({}, {})", op, p1)
    }
}

impl Parameter {
    fn get_value(&self, memory: &Vec<i32>) -> i32 {
        match self.mode {
            ParameterMode::Immediate => {
               // println!("getting Immediate value {}", self.value);
                self.value
            },
            ParameterMode::Position => {
              //  println!("getting Position value {}", memory[self.value as usize]);
                memory[self.value as usize]
            }
        }
    }
}

pub fn run_intcode(code_path: &str) -> i32 {
    let content = crate::utils::read_file(code_path);
    let mut codes: Vec<i32> = content
        .split(",")
        .map(|str_int| str_int.parse::<i32>().unwrap())
        .collect();
    let mut cur = 0;
    let mut last_output = -1;
    while codes[cur] != 99 {
        let code = format!("{}", codes[cur]);

        let mut op = Op {
            instruction: Instruction::Add,
            params: [None, None, None],
        };

        let code_len = code.len();
        let my_code = format!("{}{}", "0", code);
        let op_code = if code_len >= 2 {
            &code[code_len - 2..]
        } else {
            &my_code[..]
        };


        let mut op_len = 0;

        op.instruction = match op_code {
            "01" => {
                op_len = 4;
                Instruction::Add
            }
            "02" => {
                op_len = 4;
                Instruction::Multiply
            }
            "03" => {
                op_len = 2;
                Instruction::Input
            }
            "04" => {
                op_len = 2;
                Instruction::Output
            }
            "05" => {
                op_len = 3;
                Instruction::JumpTrue
            },
            "06" => {
                op_len = 3;
                Instruction::JumpFalse
            },
            "07" => {
                op_len = 4;
                Instruction::LessThan
            },
            "08" => {
                op_len = 4;
                Instruction::Equal
            },
            _ => panic!(format!("unknown op code {}", op_code)),
        };

        // parameters
        let mut i: usize = 0;
        while i < op_len - 1 {
            let pos: i32 = code_len as i32 - 3 - i as i32;
            op.params[i] = if pos >= 0 {
                let mode = &code[pos as usize..pos as usize + 1];
               // println!("parameter {} {}", mode, codes[cur + 1 + i]);

                if mode == "0" {
                    Some(Parameter {
                        mode: ParameterMode::Position,
                        value: codes[cur + 1 + i],
                    })
                } else {
                   Some(Parameter {
                        mode: ParameterMode::Immediate,
                        value: codes[cur + 1 + i],
                    })
                }
            } else {
              //  println!("parameter {} {}", "00", codes[cur + 1 + i]);
                Some(Parameter {
                    mode: ParameterMode::Position,
                    value: codes[cur + 1 + i],
                })
            };

            

            i += 1;
        }

       // println!("running op {} {} at {}", op, code, cur);

        let result = op.run(&mut codes);

        match result {
            Some(r) => {
                match r {
                    Result::Jump(j) => cur = j as usize,
                    Result::Output(o) => {
                        last_output = o;
                        cur += op_len;
                    }
                }
            },
            None => cur += op_len
        };

       // println!("next code at {}", cur);
    }

    return last_output;
}
