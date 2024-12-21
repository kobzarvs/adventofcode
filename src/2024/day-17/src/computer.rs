#[derive(Debug, Clone)]
pub enum Command {
    ADV = 0,
    BXL = 1,
    BST = 2,
    JNZ = 3,
    BXC = 4,
    OUT = 5,
    BDV = 6,
    CDV = 7,
}

impl TryFrom<i64> for Command {
    type Error = ();

    fn try_from(value: i64) -> Result<Self, Self::Error> {
        match value {
            0 => Ok(Command::ADV),
            1 => Ok(Command::BXL),
            2 => Ok(Command::BST),
            3 => Ok(Command::JNZ),
            4 => Ok(Command::BXC),
            5 => Ok(Command::OUT),
            6 => Ok(Command::BDV),
            7 => Ok(Command::CDV),
            _ => Err(()),
        }
    }
}

#[derive(Debug, Clone)]
pub enum Instr {
    ADV(i64),
    BXL(i64),
    BST(i64),
    JNZ(i64),
    BXC(i64),
    OUT(i64),
    BDV(i64),
    CDV(i64),
}

#[derive(Debug, Clone)]
pub struct Computer {
    pub a: i64,
    pub b: i64,
    pub c: i64,
    pub ip: i64,
    pub program: Vec<Instr>,
    pub raw_program: Vec<i64>,
    pub last_output: Option<i64>,
}

impl Computer {
    fn get_combo_value(&self, operand: i64) -> i64 {
        match operand {
            0..=3 => operand,
            4 => self.a,
            5 => self.b,
            6 => self.c,
            _ => 0, // для операнда 7 (зарезервирован)
        }
    }

    pub fn handle_next_instruction(&mut self) -> Option<i64> {
        match &self.program[self.ip as usize] {
            Instr::ADV(op) => {
                self.a = self.a >> self.get_combo_value(*op);
            }
            Instr::BDV(op) => {
                self.b = self.a >> self.get_combo_value(*op);
            }
            Instr::CDV(op) => {
                self.c = self.a >> self.get_combo_value(*op);
            }
            Instr::BXL(op) => {
                self.b ^= op;
            }
            Instr::BXC(_) => {
                self.b ^= self.c;
            }
            Instr::BST(op) => {
                self.b = self.get_combo_value(*op) & 0b111;
            }
            Instr::OUT(op) => {
                self.ip += 1;
                self.last_output = Some(self.get_combo_value(*op) % 8);
                return self.last_output;
            }
            Instr::JNZ(op) => {
                if self.a != 0 {
                    self.ip = *op;
                    self.ip -= 1;
                }
            }
        }
        self.ip += 1;
        None
    }

    pub fn run(&mut self, n: i64) -> Vec<i64> {
        let mut outputs: Vec<i64> = Vec::new();
        let len = self.program.len();
        let mut counter = 0;
        
        while (self.ip as usize) < len {
            if n != -1 && counter >= n {
                break;
            }
            if let Some(out) = self.handle_next_instruction() {
                outputs.push(out);
            }
            counter += 1;
        }

        outputs
    }

    // pub fn run_debug(&mut self) -> Vec<String> {
    //     let mut outputs: Vec<String> = Vec::new();
    //     println!("\nInitial state: {:?}\n", self);
    // 
    //     while (self.ip as usize) < self.program.len() {
    //         let current_ip = self.ip;
    //         match &self.program[self.ip as usize] {
    //             Instr::ADV(op) => {
    //                 let combo_value = self.get_combo_value(*op);
    //                 let divisor = 2_i64.pow(combo_value as u32);
    //                 println!(
    //                     "IP:{:2} ADV: A = A / 2^{} ({}) = {} / {} ",
    //                     current_ip, combo_value, op, self.a, divisor
    //                 );
    //                 self.a = self.a / divisor;
    //                 println!("     Result: {:?}", self);
    //                 self.ip += 1;
    //             }
    //             Instr::BXL(op) => {
    //                 println!(
    //                     "IP:{:2} BXL: B = B XOR {} = {} XOR {}",
    //                     current_ip, op, self.b, op
    //                 );
    //                 self.b ^= op;
    //                 println!("     Result: {:?}", self);
    //                 self.ip += 1;
    //             }
    //             Instr::BST(op) => {
    //                 let combo_value = self.get_combo_value(*op);
    //                 println!(
    //                     "IP:{:2} BST: B = {} % 8 = {}",
    //                     current_ip,
    //                     combo_value,
    //                     combo_value % 8
    //                 );
    //                 self.b = combo_value % 8;
    //                 println!("     Result: {:?}", self);
    //                 self.ip += 1;
    //             }
    //             Instr::JNZ(op) => {
    //                 println!("IP:{:2} JNZ: if A({}) != 0 goto {}", current_ip, self.a, op);
    //                 if self.a != 0 {
    //                     self.ip = *op;
    //                 } else {
    //                     self.ip += 1;
    //                 }
    //                 println!("     Result: {:?}", self);
    //             }
    //             Instr::BXC(_op) => {
    //                 println!(
    //                     "IP:{:2} BXC: B = B XOR C = {} XOR {}",
    //                     current_ip, self.b, self.c
    //                 );
    //                 self.b ^= self.c;
    //                 println!("     Result: {:?}", self);
    //                 self.ip += 1;
    //             }
    //             Instr::OUT(op) => {
    //                 let combo_value = self.get_combo_value(*op);
    //                 let out_value = combo_value % 8;
    //                 println!(
    //                     "IP:{:2} OUT: output {} % 8 = {}",
    //                     current_ip, combo_value, out_value
    //                 );
    //                 outputs.push(out_value.to_string());
    //                 println!("     Result: {:?}", self);
    //                 self.ip += 1;
    //             }
    //             Instr::BDV(op) => {
    //                 let combo_value = self.get_combo_value(*op);
    //                 let divisor = 2_i64.pow(combo_value as u32);
    //                 println!(
    //                     "IP:{:2} BDV: B = A / 2^{} ({}) = {} / {}",
    //                     current_ip, combo_value, op, self.a, divisor
    //                 );
    //                 self.b = self.a / divisor;
    //                 println!("     Result: {:?}", self);
    //                 self.ip += 1;
    //             }
    //             Instr::CDV(op) => {
    //                 let combo_value = self.get_combo_value(*op);
    //                 let divisor = 2_i64.pow(combo_value as u32);
    //                 println!(
    //                     "IP:{:2} CDV: C = A / 2^{} ({}) = {} / {}",
    //                     current_ip, combo_value, op, self.a, divisor
    //                 );
    //                 self.c = self.a / divisor;
    //                 println!("     Result: {:?}", self);
    //                 self.ip += 1;
    //             }
    //         }
    //         println!("");
    //     }
    // 
    //     println!("Final outputs: {}", outputs.join(","));
    //     outputs
    // }
}
