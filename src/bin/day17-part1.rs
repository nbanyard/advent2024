use std::num::ParseIntError;

fn main() {
    match run(include_str!("day17-testdata.txt")) {
        Err(err) => {
            println!("Error {:?}", err);
            return;
        }
        Ok(output) => {
            println!("{}", output);
        }
    };
}

fn run(bootstrap: &str) -> Result<String, Error> {
    let computer = Computer::load(bootstrap);
    println!("{:?}", computer);
    Err(Error::NotImplemented)
}

#[derive(Debug)]
struct Computer {
    registers: [isize; 3],
    program: Vec<u8>,
}

impl Computer {
    fn load(bootstrap: &str) -> Result<Self, Error> {
        let mut result = Self {
            registers: [0; 3],
            program: Vec::new(),
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
}

#[derive(Debug)]
enum Error {
    NotImplemented,
    CorruptBootstrap,
}
