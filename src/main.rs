#[derive(Debug)]
enum OpCode {
    Add,
    Multiply,
    Input,
    Output,
    JumpIfNz,
    JumpIfZ,
    JumpLt,
    JumpEq,
    AdjRelBase,
    Exit,
}

impl OpCode {
    fn from(chars: &mut Vec<char>) -> Self {
        let opcode = chars.pop().unwrap().to_digit(10).unwrap()
            + (chars.pop().unwrap_or('0').to_digit(10).unwrap()) * 10;

        match opcode {
            1 => OpCode::Add,
            2 => OpCode::Multiply,
            3 => OpCode::Input,
            4 => OpCode::Output,
            5 => OpCode::JumpIfNz,
            6 => OpCode::JumpIfZ,
            7 => OpCode::JumpLt,
            8 => OpCode::JumpEq,
            9 => OpCode::AdjRelBase,
            99 => OpCode::Exit,
            x => panic!("Unrecognized opcode {}", x),
        }
    }
}

#[derive(Debug)]
enum ReferenceType {
    Direct,
    Indirect,
    Relative,
}

impl ReferenceType {
    fn from(c: char) -> Self {
        match c {
            '0' => ReferenceType::Indirect,
            '1' => ReferenceType::Direct,
            '2' => ReferenceType::Relative,
            x => panic!("Unrecognized reference type {}", x),
        }
    }
}

#[derive(Default, Clone)]
struct Computer {
    memory: Vec<i64>,
    index: usize,
    instruction_chars: Vec<char>,
    inputs: Vec<i64>,
    relative_base: i64,
}

impl Computer {
    fn new_from_file(filename: &str) -> Self {
        Computer {
            memory: std::fs::read_to_string(filename)
                .unwrap()
                .split(',')
                .map(|input| input.parse::<i64>().unwrap())
                .collect(),
            ..Default::default()
        }
    }

    fn get_next_value(&mut self) -> i64 {
        let source = match ReferenceType::from(self.instruction_chars.pop().unwrap_or('0')) {
            ReferenceType::Indirect => self.memory[self.index],
            ReferenceType::Direct => self.index as i64,
            ReferenceType::Relative => self.memory[self.index] + self.relative_base,
        };
        println!("Source {}", source); 
        let source_index = source as usize;
        let val = if source_index > self.memory.len() {
            0
        } else {
            self.memory[source as usize]
        };
        self.index += 1;
        val
    }

    fn write(&mut self, val: i64) {
        let dest = match ReferenceType::from(self.instruction_chars.pop().unwrap_or('0')) {
            ReferenceType::Indirect => self.memory[self.index],
            ReferenceType::Direct => panic!("Direct write not permitted!"),
            ReferenceType::Relative => self.memory[self.index] + self.relative_base,
        };
        println!("Dest {}", dest); 
        let dest_index = dest as usize;
        while dest_index >= self.memory.len() {
            self.memory.push(0);
        };
        self.memory[dest_index] = val;
        self.index += 1;
    }

    fn get_instruction(&mut self) -> Vec<char> {
        let instruction = self.memory[self.index].to_string().chars().collect();
        self.index += 1;
        instruction
    }

    fn run(&mut self) -> Result<(), i64> {

        println!("Inputs {:?}", self.inputs);

        loop {
            self.instruction_chars = self.get_instruction();
            let opcode = OpCode::from(&mut self.instruction_chars);
            println!("{:?}", opcode);

            match opcode {
                OpCode::Add => {
                    let val = self.get_next_value() + self.get_next_value();
                    self.write(val);
                }
                OpCode::Multiply => {
                    let val = self.get_next_value() * self.get_next_value();
                    self.write(val);
                }
                OpCode::Input => {
                    let input = self.inputs.remove(0);
                    self.write(input);
                }
                OpCode::Output => {
                    let v1 = self.get_next_value();
                    return Err(v1); 
                }
                OpCode::JumpIfZ => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    if v1 == 0 {
                        println!("Jump!");
                        self.index = v2 as usize;
                    }
                }
                OpCode::JumpIfNz => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    if v1 != 0 {
                        println!("Jump!");
                        self.index = v2 as usize;
                    }
                }
                OpCode::JumpLt => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    self.write(if v1 < v2 { 1 } else { 0 });
                }
                OpCode::JumpEq => {
                    let (v1, v2) = (self.get_next_value(), self.get_next_value());
                    self.write(if v1 == v2 { 1 } else { 0 });
                }
                OpCode::AdjRelBase => {
                    println!("Moving base from {}", self.relative_base);
                    let v1 = self.get_next_value();
                    self.relative_base += v1;
                    println!("Moving base to {}", self.relative_base);
                }
                OpCode::Exit => {
                    return Ok(());
                }
            };
        }
    }
}

fn main() {
    let mut computer = Computer::new_from_file("inputs.txt");

    let mut args = std::env::args();
    let _ = args.next();
    let a1 = args.next().unwrap();

    computer.inputs.push(a1.parse().unwrap());

    loop {
        match computer.run() {
            Ok(_) => {
                println!("... done");
                break;
            }
            Err(x) => {
                println!("BOOST output {}", x)
            }
        }
    }   
}
