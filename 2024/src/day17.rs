use std::{fs::read_to_string, thread};

struct Computer {
    pub a_register: u64,
    pub b_register: u64,
    pub c_register: u64,
    pub instructions: Vec<u64>,
    instruction_pointer: usize,
}

impl Computer {
    pub fn new(a_register: u64, b_register: u64, c_register: u64, instructions: Vec<u64>) -> Self {
        Self {
            a_register,
            b_register,
            c_register,
            instructions,
            instruction_pointer: 0,
        }
    }

    pub fn cycle(&mut self) -> Option<u64> {
        let instruction = self.instructions[self.instruction_pointer];
        let operand = self.instructions[self.instruction_pointer + 1];

        match instruction {
            0 => {
                self.adv(operand);
                None
            }
            1 => {
                self.bxl(operand);
                None
            }
            2 => {
                self.bst(operand);
                None
            }
            3 => {
                self.jnz(operand);
                None
            }
            4 => {
                self.bxc();
                None
            }
            5 => Some(self.out(operand)),
            6 => {
                self.bdv(operand);
                None
            }
            7 => {
                self.cdv(operand);
                None
            }
            _ => None,
        }
    }

    pub fn can_cycle(&self) -> bool {
        self.instruction_pointer < self.instructions.len()
    }

    fn get_combo_operand(&self, operand: u64) -> u64 {
        if (0..=3).contains(&operand) {
            return operand;
        }
        match operand {
            4 => self.a_register,
            5 => self.b_register,
            6 => self.c_register,
            _ => 8,
        }
    }

    fn adv(&mut self, combo_op: u64) {
        self.a_register >>= self.get_combo_operand(combo_op);
        self.instruction_pointer += 2;
    }

    fn bxl(&mut self, literal_op: u64) {
        self.b_register ^= literal_op;
        self.instruction_pointer += 2;
    }

    fn bst(&mut self, combo_op: u64) {
        self.b_register = self.get_combo_operand(combo_op) % 8;
        self.instruction_pointer += 2;
    }

    fn jnz(&mut self, literal_op: u64) {
        if self.a_register == 0 {
            self.instruction_pointer += 2;
        } else {
            self.instruction_pointer = literal_op as usize;
        }
    }

    fn bxc(&mut self) {
        self.b_register ^= self.c_register;
        self.instruction_pointer += 2;
    }

    fn out(&mut self, combo_op: u64) -> u64 {
        self.instruction_pointer += 2;
        self.get_combo_operand(combo_op) % 8
    }

    fn bdv(&mut self, combo_op: u64) {
        self.b_register = self.a_register >> self.get_combo_operand(combo_op);
        self.instruction_pointer += 2;
    }

    fn cdv(&mut self, combo_op: u64) {
        self.c_register = self.a_register >> self.get_combo_operand(combo_op);
        self.instruction_pointer += 2;
    }
}

pub fn part_one() {
    let input = read_to_string("./src/input").unwrap();

    let mut lines = input.lines();
    let a_register = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let b_register = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let c_register = lines
        .next()
        .unwrap()
        .split(": ")
        .nth(1)
        .unwrap()
        .parse::<u64>()
        .unwrap();
    let program = lines
        .next_back()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    println!(
        "{}\t{}\t{}\n{}",
        a_register,
        b_register,
        c_register,
        0,
        // program
        //     .iter()
        //     .map(|x| x.to_string())
        //     .collect::<Vec<_>>()
        //     .join(",")
    );

    println!(
        "{}",
        program
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<_>>()
            .join(",")
    );

    let get_op = |pointer: usize, instructions: Vec<u64>| -> String {
        match instructions[pointer] {
            0 => "adv",
            1 => "bxl",
            2 => "bst",
            3 => "jnz",
            4 => "bxc",
            5 => "out",
            6 => "bdv",
            7 => "cdv",
            _ => "",
        }
        .to_string()
    };

    let mut computer = Computer::new(a_register, b_register, c_register, program);
    let mut out = vec![];
    while computer.can_cycle() {
        print!(
            "Op: {},{}  ",
            get_op(computer.instruction_pointer, computer.instructions.clone()),
            computer.instructions[computer.instruction_pointer + 1]
        );

        let output = computer.cycle();
        if output.is_some() {
            out.push(output.unwrap().to_string());
        }
        // if let Some(output) = computer.cycle() {
        //     out.push(output.to_string());
        // }

        println!(
            "A: {:#032b}  B: {:#032b}  C: {:#032b}  P: {}  {}",
            computer.a_register,
            computer.b_register,
            computer.c_register,
            computer.instruction_pointer,
            if output.is_some() {
                output.unwrap().to_string()
            } else {
                "".to_string()
            }
        );
    }

    println!("{}", out.join(","));
}

pub fn part_two() {
    let input = read_to_string("./src/input").unwrap();

    let mut lines = input.lines();
    let program = lines
        .next_back()
        .unwrap()
        .split(' ')
        .nth(1)
        .unwrap()
        .split(',')
        .map(|x| x.parse::<u64>().unwrap())
        .collect::<Vec<_>>();

    // println!(
    //     "{}\t{}\t{}\n{}",
    //     a_register,
    //     b_register,
    //     c_register,
    //     0,
    // program
    //     .iter()
    //     .map(|x| x.to_string())
    //     .collect::<Vec<_>>()
    //     .join(",")
    // );

    // let get_op = |pointer: usize, instructions: Vec<u32>| -> String {
    //     match instructions[pointer] {
    //         0 => "adv",
    //         1 => "bxl",
    //         2 => "bst",
    //         3 => "jnz",
    //         4 => "bxc",
    //         5 => "out",
    //         6 => "bdv",
    //         7 => "cdv",
    //         _ => "",
    //     }
    //     .to_string()
    // };

    let num_threads = 8;
    let mut handles = vec![];
    for j in 0..num_threads {
        let cloned = program.clone();
        let min = 2u64.pow(3 * (cloned.len() as u32 - 1));
        let max = 2u64.pow(3 * cloned.len() as u32);
        let diff = max - min;
        let part = diff / num_threads;

        let handle = thread::spawn(move || {
            for i in (min + part * j)..(min + part * (j + 1)) {
                let mut computer = Computer::new(i, 0, 0, cloned.clone());
                let mut out = vec![];
                while computer.can_cycle() {
                    // print!(
                    //     "Op: {}  ",
                    //     get_op(computer.instruction_pointer, computer.instructions.clone())
                    // );

                    // let output = computer.cycle();
                    // if output.is_some() {
                    //     out.push(output.unwrap().to_string());
                    // }
                    if let Some(output) = computer.cycle() {
                        out.push(output);
                    }

                    // println!(
                    //     "A: {:#032b}  B: {:#032b}  C: {:#032b}  P: {}  {}",
                    //     computer.a_register,
                    //     computer.b_register,
                    //     computer.c_register,
                    //     computer.instruction_pointer,
                    //     if output.is_some() {
                    //         output.unwrap().to_string()
                    //     } else {
                    //         "".to_string()
                    //     }
                    // );
                }
                if out == cloned {
                    println!("{i}");
                    // println!(
                    //     "{}",
                    //     out.iter()
                    //         .map(|x| x.to_string())
                    //         .collect::<Vec<_>>()
                    //         .join(",")
                    // );
                    break;
                }
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join();
    }
    // for i in (2u64.pow(3 * (program.len() as u32 - 1)))..(2u64.pow(3 * program.len() as u32)) {
    //     let mut computer = Computer::new(i, 0, 0, program.clone());
    //     let mut out = vec![];
    //     while computer.can_cycle() {
    //         // print!(
    //         //     "Op: {}  ",
    //         //     get_op(computer.instruction_pointer, computer.instructions.clone())
    //         // );

    //         // let output = computer.cycle();
    //         // if output.is_some() {
    //         //     out.push(output.unwrap().to_string());
    //         // }
    //         if let Some(output) = computer.cycle() {
    //             out.push(output);
    //         }

    //         // println!(
    //         //     "A: {:#032b}  B: {:#032b}  C: {:#032b}  P: {}  {}",
    //         //     computer.a_register,
    //         //     computer.b_register,
    //         //     computer.c_register,
    //         //     computer.instruction_pointer,
    //         //     if output.is_some() {
    //         //         output.unwrap().to_string()
    //         //     } else {
    //         //         "".to_string()
    //         //     }
    //         // );
    //     }
    //     if out == program {
    //         println!("{i}");
    //         // println!(
    //         //     "{}",
    //         //     out.iter()
    //         //         .map(|x| x.to_string())
    //         //         .collect::<Vec<_>>()
    //         //         .join(",")
    //         // );
    //         break;
    //     }
    // }

    // println!("{}", out.join(","));
}
