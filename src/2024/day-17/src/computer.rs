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
            _ => Err(())
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
    pub ip: usize,
    pub program: Vec<Instr>,
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

    pub fn run(&mut self, id: i64) -> Vec<String> {
        let mut outputs: Vec<String> = Vec::new();
        
        while self.ip < self.program.len() {
            match &self.program[self.ip] {
                Instr::ADV(op) => {
                    let combo_value = self.get_combo_value(*op);
                    let divisor = 2_i64.pow(combo_value as u32);
                    self.a = self.a / divisor;
                    self.ip += 1;
                    // println!("id: {},  adv {} => {:?}", id, op, self);
                },
                Instr::BXL(op) => {
                    self.b ^= op; // литеральный операнд
                    self.ip += 1;
                    // println!("id: {}, bxl {} => {:?}", id, op, self);
                },
                Instr::BST(op) => {
                    let combo_value = self.get_combo_value(*op);
                    self.b = combo_value % 8;
                    self.ip += 1;
                    // println!("id: {},  bst {} => {:?}", id, op, self);
                },
                Instr::JNZ(op) => {
                    if self.a != 0 {
                        self.ip = *op as usize; // литеральный операнд
                    } else {
                        self.ip += 1;
                    }
                    // println!("id: {}, jnz {} => {:?}", id, op, self);
                },
                Instr::BXC(_) => {
                    self.b ^= self.c;
                    self.ip += 1;
                    // println!("id: {}, bxc => {:?}", id, self);
                },
                Instr::OUT(op) => {
                    let combo_value = self.get_combo_value(*op);
                    outputs.push((combo_value % 8).to_string());
                    self.ip += 1;
                    // println!("id: {}, out {} => {:?}", id, op, self);
                    // print!("id: {:9}\r", id);
                },
                Instr::BDV(op) => {
                    let combo_value = self.get_combo_value(*op);
                    let divisor = 2_i64.pow(combo_value as u32);
                    self.b = self.a / divisor;
                    self.ip += 1;
                    // println!("id: {}, bdv {} => {:?}", id, op, self);
                },
                Instr::CDV(op) => {
                    let combo_value = self.get_combo_value(*op);
                    let divisor = 2_i64.pow(combo_value as u32);
                    self.c = self.a / divisor;
                    self.ip += 1;
                    // println!("id: {}, cdv {} => {:?}", id, op, self);
                },
            }
        }
        
        outputs
    }
}
