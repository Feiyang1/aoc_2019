use std::io;
use std::fmt;
use std::collections::HashMap;

struct Op {
    instruction: Instruction,
    params: [Option<Parameter>; 3],
}

struct Parameter {
    value: i128,
    mode: ParameterMode,
}

enum ParameterMode {
    Position,
    Immediate,
    Relative(i128) // relative base value
}

enum Instruction {
    Add,
    Multiply,
    Input(Option<i128>),
    Output,
    JumpTrue,
    JumpFalse,
    LessThan,
    Equal,
    UpdateRelativeBase
}

enum Result {
    Jump(i128),
    Output(i128)
}

impl Op {
    fn run(&self, codes: &mut Vec<i128>, mem: &mut HashMap<i128, i128>, relative_base: &mut i128) -> Option<Result> {

        match self.instruction {
            Instruction::Add => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);
                let p2 = self.params[1].as_ref().unwrap().get_value(codes, mem);
                let p3 = self.params[2].as_ref().unwrap().get_addr(codes);

                let val = p1 + p2;
                // println!("{} + {} = {} to pos {}", p1, p2, val, p3);

                insert_val(codes, mem, p3, val);
            },
            Instruction::Multiply => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);
                let p2 = self.params[1].as_ref().unwrap().get_value(codes, mem);
                let p3 = self.params[2].as_ref().unwrap().get_addr(codes);

                let val = p1*p2;
                // println!("{} * {} = {} to pos {}", p1, p2, val, p3);
                insert_val(codes, mem, p3, val);
            },
            Instruction::Input(i) => {


                let input = match i {
                    Some(val) => val,
                    None => {
                        let mut input = String::new();
                        // println!("please enter a code");
                        io::stdin().read_line(&mut input).expect("Falied to read line");
                
                        match input.trim().parse() {
                            Ok(num) => num,
                            Err(_) => panic!(format!("{} is not a number", input))
                        }
                    }
                };

                let p1 = self.params[0].as_ref().unwrap().get_addr(codes);
                // println!("saving {} to pos {}", input, p1);

                insert_val(codes, mem, p1, input);
            },
            Instruction::Output => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);
                println!("The output is {}", p1);
                return Some(Result::Output(p1));
            },
            Instruction::JumpTrue => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);

                if p1 > 0 {
                    let p2 = self.params[1].as_ref().unwrap().get_value(codes, mem);
                    return Some(Result::Jump(p2));
                }
            },
            Instruction::JumpFalse => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);

                if p1 == 0 {
                    let p2 = self.params[1].as_ref().unwrap().get_value(codes, mem);
                    return Some(Result::Jump(p2));
                }
            },
            Instruction::LessThan => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);
                let p2 = self.params[1].as_ref().unwrap().get_value(codes, mem);
                let p3 = self.params[2].as_ref().unwrap().get_addr(codes);

                let val = if p1 < p2 { 1 } else { 0 };

                insert_val(codes, mem, p3, val);
            },
            Instruction::Equal => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);
                let p2 = self.params[1].as_ref().unwrap().get_value(codes, mem);
                let p3 = self.params[2].as_ref().unwrap().get_addr(codes);

                let val = if p1 == p2 { 1 } else { 0 };
                insert_val(codes, mem, p3, val);
            }
            Instruction::UpdateRelativeBase => {
                let p1 = self.params[0].as_ref().unwrap().get_value(codes, mem);
                *relative_base += p1;
            }
        };

        return None;
    }
}

fn  insert_val(codes: &mut Vec<i128>, mem: &mut HashMap<i128, i128>, pos: i128, val: i128) {

    let codes_len = codes.len() as i128;
    if codes_len > pos {                    
        codes[pos as usize] = val;
    } else {
        mem.insert(pos, val);
    }
}

impl fmt::Display for Op {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let op = match self.instruction {
            Instruction::Add => "Add",
            Instruction::Multiply => "Multiply",
            Instruction::Input(_) => "Input",
            Instruction::Output => "Output",
            Instruction::JumpFalse => "JumpFalse",
            Instruction::JumpTrue => "JumpTrue",
            Instruction::LessThan => "LessThan",
            Instruction::Equal => "Equal",
            Instruction::UpdateRelativeBase => "UpdateRelativeBase"
        };

        let p1 = match &self.params[0] {
            Some(i) => "Some",
            None => "None"
        };

        write!(f, "({}, {})", op, p1)
    }
}

impl Parameter {
    fn get_value(&self, codes: &Vec<i128>, mem: &HashMap<i128, i128>) -> i128 {

        let codes_len = codes.len();
        match self.mode {
            ParameterMode::Immediate => {
              //   println!("getting Immediate value {}", self.value);
                self.value
            },
            ParameterMode::Position => {
              if codes_len as i128 > self.value  {                
             //   println!("getting Position value {}", codes[self.value as usize]);
                codes[self.value as usize]
              } else {
                match mem.get(&self.value) {
                    Some(val) => {
                //         println!("getting Position value {} in mem at addr {}", val, self.value);
                        *val
                    },
                    None => {
                //         println!("getting Position value {} in mem at addr {}", 0, self.value);
                        0
                    }
                }
              }
            },
            ParameterMode::Relative(base) => {
                let addr = self.value + base;
                if codes_len as i128 > addr {
                //    println!("getting Position value {}", codes[addr as usize]);
                    codes[addr as usize]
                } else {
                //    println!("getting position value from mem");
                    match mem.get(&addr) {
                        Some(val) => *val,
                        None => 0
                    }
                }
            }
        }
    }

    fn get_addr(&self, codes: &Vec<i128>) -> i128 {
        match self.mode {
            ParameterMode::Immediate => panic!("addr can't be Immediate"),
            ParameterMode::Position => self.value,
            ParameterMode::Relative(base) => self.value + base 
        }
    }
}

pub struct IntcodeResult {
    pub output: Option<i128>,
    pub outputs_since_start_or_resume: Vec<i128>,
    pub resume_point: Option<usize>,
    pub relative_base: i128
}

pub fn run_intcode_raw(codes: &mut Vec<i128>, memory: Option<&mut HashMap<i128, i128>>, inputs: Vec<i128>, resume_point: usize, stop_on_pending_input: bool, relative_base: i128) -> IntcodeResult {
    let mut cur = resume_point;
    let mut outputs = vec![];
    let mut input_count = 0;
    let mut relative_base = relative_base;

    let mut mm = HashMap::new();
    let mut memory = match memory {
        Some(m) => m,
        None => &mut mm
    };

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
                if input_count < inputs.len() {
                    let input = inputs[input_count];
                    input_count += 1;
                    // println!("the input is {}", input);
                    Instruction::Input(Some(input))
                } else {
                    // stop the program on pending input and return the resume pointer
                    if stop_on_pending_input {
                        return IntcodeResult {
                            output: if outputs.len() > 0 { Some(outputs[outputs.len() - 1]) } else { None },
                            outputs_since_start_or_resume: outputs,
                            resume_point: Some(cur),
                            relative_base
                        };
                    } else {
                        Instruction::Input(None)
                    }
                }
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
            "09" => {
                op_len = 2;
                Instruction::UpdateRelativeBase
            }
            _ => panic!(format!("unknown op code {}", op_code)),
        };

        // parameters
        let mut i: usize = 0;
        while i < op_len - 1 {
            let pos: i128 = code_len as i128 - 3 - i as i128;
            op.params[i] = if pos >= 0 {
                let mode = &code[pos as usize..pos as usize + 1];
               // println!("parameter {} {}", mode, codes[cur + 1 + i]);

                if mode == "0" {
                    Some(Parameter {
                        mode: ParameterMode::Position,
                        value: codes[cur + 1 + i],
                    })
                } else if mode == "1" {
                   Some(Parameter {
                        mode: ParameterMode::Immediate,
                        value: codes[cur + 1 + i],
                    })
                } else { // mode == "2"
                    Some(Parameter {
                        mode: ParameterMode::Relative(relative_base),
                        value: codes[cur + 1 + i]
                    })
                }
            } else {
              // println!("parameter {} {}", "00", codes[cur + 1 + i]);
                Some(Parameter {
                    mode: ParameterMode::Position,
                    value: codes[cur + 1 + i],
                })
            };

            

            i += 1;
        }

      // println!("running op {} {} at {}", op, code, cur);
        let result = op.run(codes, &mut memory, &mut relative_base);

        match result {
            Some(r) => {
                match r {
                    Result::Jump(j) => cur = j as usize,
                    Result::Output(o) => {
                        outputs.push(o);
                        cur += op_len;
                    }
                }
            },
            None => cur += op_len
        };

        // println!("next code at {}", cur);
    }

    return IntcodeResult {
        output: if outputs.len() > 0 { Some(outputs[outputs.len() - 1]) } else { None },
        outputs_since_start_or_resume: outputs,
        resume_point: None,
        relative_base
    };
}

pub fn run_intcode(code_path: &str, inputs: Vec<i128>) -> i128 {
    let content = crate::utils::read_file(code_path);
    let mut codes: Vec<i128> = content
        .split(",")
        .map(|str_int| str_int.parse::<i128>().unwrap())
        .collect();
    
    let result = run_intcode_raw(&mut codes, None, inputs, 0, false, 0);
    return result.output.unwrap();
}

pub fn run_intcode_state(state: &mut IntcodeState, inputs: Vec<i128>, stop_on_pending_input: bool) -> IntcodeResult {
    run_intcode_raw(&mut state.codes, Some(&mut state.mem), inputs, state.resume_point.unwrap(), stop_on_pending_input, state.relative_base)
}

pub struct IntcodeState {
    pub codes: Vec<i128>,
    pub resume_point: Option<usize>,
    pub relative_base: i128,
    pub mem: HashMap<i128, i128>
}