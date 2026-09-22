use std::num::ParseIntError;

fn main() {
    match run(include_str!("day17-data.txt")) {
        Err(err) => {
            println!("Error {:?}", err);
            return;
        }
        Ok(output) => {
            println!("{}", output);
        }
    };
}

fn run(bootstrap: &str) -> Result<isize, Error> {
    let mut computer = Computer::load(bootstrap)?;
    let mut a: isize = 0;
    loop {
        computer.reset(a);
        computer.run()?;
        if computer.program == computer.output {
            return Ok(a);
        }
        if a % 100000 == 0 {
            println!("{}: {:?}", a, computer.output);
        }
        a += 1;
    }
}

#[derive(Debug)]
struct Computer {
    registers: [isize; 3],
    program: Vec<u8>,
    pc: usize,
    output: Vec<u8>,
}

type Opcode = fn(&mut Computer, u8) -> Result<(), Error>;
const OPCODES: [Opcode; 8] = [
    Computer::adv, // 0
    Computer::bxl, // 1
    Computer::bst, // 2
    Computer::jnz, // 3
    Computer::bxc, // 4
    Computer::out, // 5
    Computer::bdv, // 6
    Computer::cdv, // 7
];

impl Computer {
    fn load(bootstrap: &str) -> Result<Self, Error> {
        let mut result = Self {
            registers: [0; 3],
            program: Vec::new(),
            pc: 0,
            output: Vec::new(),
        };

        for line in bootstrap.lines() {
            let mut words = line.split_ascii_whitespace();
            let first_word = words.next();

            if first_word == Some("Register") {
                let second_word = words.next();
                let register = if second_word == Some("A:") {
                    0
                } else if second_word == Some("B:") {
                    1
                } else if second_word == Some("C:") {
                    2
                } else {
                    return Err(Error::CorruptBootstrap);
                };
                match words.next() {
                    None => return Err(Error::CorruptBootstrap),
                    Some(third_word) => match isize::from_str_radix(third_word, 10) {
                        Err(_) => return Err(Error::CorruptBootstrap),
                        Ok(value) => result.registers[register] = value,
                    },
                }
            } else if first_word == Some("Program:") {
                match words.next() {
                    None => return Err(Error::CorruptBootstrap),
                    Some(second_word) => {
                        let program: Vec<Result<u8, ParseIntError>> = second_word
                            .split(",")
                            .map(|word| u8::from_str_radix(word, 10))
                            .collect();
                        if program.iter().any(|r| r.is_err()) {
                            return Err(Error::CorruptBootstrap);
                        }
                        result.program = program.into_iter().map(|r| r.unwrap()).collect();
                    }
                };
            }
        }
        Ok(result)
    }

    fn reset(&mut self, a: isize) {
        self.registers[0] = a;
        self.registers[1] = 0;
        self.registers[2] = 0;
        self.pc = 0;
        self.output.truncate(0);
    }

    fn run(&mut self) -> Result<(), Error> {
        while self.pc < self.program.len() {
            let opcode = self.program[self.pc];
            let operand = self.program[self.pc + 1];
            self.pc += 2;

            OPCODES[opcode as usize](self, operand)?;
        }
        Ok(())
    }

    fn adv(&mut self, operand: u8) -> Result<(), Error> {
        self.registers[0] = self.registers[0] >> self.combo(operand)? as usize;
        Ok(())
    }

    fn bxl(&mut self, operand: u8) -> Result<(), Error> {
        self.registers[1] = self.registers[1] ^ operand as isize;
        Ok(())
    }

    fn bst(&mut self, operand: u8) -> Result<(), Error> {
        self.registers[1] = self.combo(operand)? % 8;
        Ok(())
    }

    fn jnz(&mut self, operand: u8) -> Result<(), Error> {
        if self.registers[0] != 0 {
            self.pc = operand as usize;
        }
        Ok(())
    }

    fn bxc(&mut self, _: u8) -> Result<(), Error> {
        self.registers[1] = self.registers[1] ^ self.registers[2];
        Ok(())
    }

    fn out(&mut self, operand: u8) -> Result<(), Error> {
        self.output.push((self.combo(operand)? % 8) as u8);
        Ok(())
    }

    fn bdv(&mut self, operand: u8) -> Result<(), Error> {
        self.registers[1] = self.registers[0] >> self.combo(operand)? as usize;
        Ok(())
    }

    fn cdv(&mut self, operand: u8) -> Result<(), Error> {
        self.registers[2] = self.registers[0] >> self.combo(operand)? as usize;
        Ok(())
    }

    fn combo(&self, operand: u8) -> Result<isize, Error> {
        if operand < 4 {
            return Ok(operand as isize);
        } else if operand < 7 {
            return Ok(self.registers[operand as usize - 4]);
        }
        Err(Error::NotImplemented)
    }
}

#[derive(Debug)]
enum Error {
    NotImplemented,
    CorruptBootstrap,
}
