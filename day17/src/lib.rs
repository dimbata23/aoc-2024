use num::FromPrimitive;
use num_derive::FromPrimitive;
use std::fs::File;
use std::io::{self, BufRead};
use std::path::Path;

type Dt = i128;

pub fn run() -> io::Result<()> {
    let input = parse_file("sample_2_input")?;
    let res_part1 = calculate_part1(&input);
    let res_part2 = calculate_part2(&input);

    println!("Part one result: {res_part1}");
    println!("Part two result: {res_part2}");
    Ok(())
}

fn calculate_part1(input: &Computer) -> String {
    let mut computer = input.clone();
    computer.compute();
    computer.get_output()
}

fn calculate_part2(input: &Computer) -> String {
    "".to_string()
}

#[derive(Debug, Clone)]
struct Computer {
    reg_a: Dt,
    reg_b: Dt,
    reg_c: Dt,
    program: Vec<u8>,
    ip: usize,
    out: Vec<Dt>,
}

impl Computer {
    fn compute(&mut self) {
        self.print();
        while self.do_next_instruction() {
            self.print();
        }
    }

    fn get_output(&self) -> String {
        self.out
            .iter()
            .map(Dt::to_string)
            .collect::<Vec<String>>()
            .join(",")
    }

    fn literal(&self, operand: u8) -> Dt {
        operand as Dt
    }

    fn combo(&self, operand: u8) -> Dt {
        match operand {
            num if num <= 3 => num as Dt,
            4 => self.reg_a,
            5 => self.reg_b,
            6 => self.reg_c,
            _ => unreachable!("FIRE!!!"),
        }
    }

    fn instruction(&self) -> Instruction {
        FromPrimitive::from_u8(self.program[self.ip]).unwrap()
    }

    fn operand(&self) -> u8 {
        self.program[self.ip + 1]
    }

    fn lit_op(&self) -> Dt {
        self.literal(self.operand())
    }

    fn combo_op(&self) -> Dt {
        self.combo(self.operand())
    }

    /// Returns false if the Computer has halted
    fn do_next_instruction(&mut self) -> bool {
        if self.ip >= self.program.len() {
            return false;
        }

        let instruction = self.instruction();
        match instruction {
            Instruction::Adv => self.do_adv(),
            Instruction::Bxl => self.do_bxl(),
            Instruction::Bst => self.do_bst(),
            Instruction::Jnz => self.do_jnz(),
            Instruction::Bxc => self.do_bxc(),
            Instruction::Out => self.do_out(),
            Instruction::Bdv => self.do_bdv(),
            Instruction::Cdv => self.do_cdv(),
        }

        true
    }

    fn do_dv(&mut self, reg: char) {
        match reg {
            'a' => self.reg_a = self.reg_a / 2_i128.pow(self.combo_op().try_into().unwrap()),
            'b' => self.reg_b = self.reg_a / 2_i128.pow(self.combo_op().try_into().unwrap()),
            'c' => self.reg_c = self.reg_a / 2_i128.pow(self.combo_op().try_into().unwrap()),
            _ => unreachable!(),
        }
        self.ip += 2;
    }

    fn do_adv(&mut self) {
        self.do_dv('a');
    }

    fn do_bxl(&mut self) {
        self.reg_b = self.reg_b ^ self.lit_op();
        self.ip += 2;
    }

    fn do_bst(&mut self) {
        self.reg_b = self.combo_op() % 8;
        self.ip += 2;
    }

    fn do_jnz(&mut self) {
        if self.reg_a == 0 {
            self.ip += 2; // TODO: or just += 1?
            return;
        }

        self.ip = self.lit_op().try_into().unwrap();
    }

    fn do_bxc(&mut self) {
        self.reg_b = self.reg_b ^ self.reg_c;
        self.ip += 2;
    }

    fn do_out(&mut self) {
        self.out.push(self.combo_op() % 8);
        self.ip += 2;
    }

    fn do_bdv(&mut self) {
        self.do_dv('b');
    }

    fn do_cdv(&mut self) {
        self.do_dv('c');
    }

    fn print(&self) {
        println!("A: {}", self.reg_a);
        println!("B: {}", self.reg_b);
        println!("C: {}", self.reg_c);
        println!("out: {:?}", self.out);
        println!("{:?}", self.program);
        println!("{}^", " ".repeat(1 + self.ip * 3));
        println!("-----------------------------------------------------------");
    }
}

#[repr(u8)]
#[derive(Debug, Eq, PartialEq, FromPrimitive)]
enum Instruction {
    Adv, // reg_a / (2^combo(op)) -> reg_a
    Bxl, // reg_b ^ literal(op) -> reg_b
    Bst, // combo(op) % 8 -> reg_b
    Jnz, // if reg_a == 0 { does nothing } else { ip = literal(op) // don't += 2 }
    Bxc, // reg_b ^ reg_c -> reg_b (reads an operand but ignores it)
    Out, // combo(op) % 8 -> output (comma separated)
    Bdv, // reg_a / (2^combo(op)) -> reg_b (like Adv, but stores in reg_b)
    Cdv, // reg_a / (2^combo(op)) -> reg_b (like Adv, but stores in reg_c)
}

fn parse_file(file_path: &str) -> io::Result<Computer> {
    let path = Path::new(file_path);
    let file = File::open(&path)?;
    let reader = io::BufReader::new(file);

    let mut register_a = 0;
    let mut register_b = 0;
    let mut register_c = 0;
    let mut program = Vec::new();

    for line in reader.lines() {
        let line = line?;
        if line.starts_with("Register A:") {
            if let Some(value) = line.split(":").nth(1) {
                register_a = value.trim().parse::<Dt>().unwrap_or(0);
            }
        } else if line.starts_with("Register B:") {
            if let Some(value) = line.split(":").nth(1) {
                register_b = value.trim().parse::<Dt>().unwrap_or(0);
            }
        } else if line.starts_with("Register C:") {
            if let Some(value) = line.split(":").nth(1) {
                register_c = value.trim().parse::<Dt>().unwrap_or(0);
            }
        } else if line.starts_with("Program:") {
            if let Some(values) = line.split(":").nth(1) {
                program = values
                    .split(',')
                    .filter_map(|v| v.trim().parse::<u8>().ok())
                    .collect();
            }
        }
    }

    Ok(Computer {
        reg_a: register_a,
        reg_b: register_b,
        reg_c: register_c,
        program,
        ip: 0,
        out: vec![],
    })
}
