use std::io::{stdin, Read};

#[derive(Debug)]
struct Program {
    memory: Vec<i32>,
    ip: usize,
}

impl Program {
    fn new(integers: &[i32]) -> Program {
        Program {
            memory: integers.to_vec(),
            ip: 0,
        }
    }

    fn run(&mut self, input: i32) -> bool {
        let opcode = self.memory[self.ip] % 100;

        match opcode {
            1 => {
                // Add
                let parameter_1_mode = (self.memory[self.ip] / 100) & 1;
                let parameter_2_mode = (self.memory[self.ip] / 1000) & 1;

                let parameter_1 = self.memory[self.ip + 1];
                let parameter_2 = self.memory[self.ip + 2];
                let parameter_3 = self.memory[self.ip + 3];
                self.ip += 4;

                let value_1 = match parameter_1_mode {
                    0 => self.memory[parameter_1 as usize],
                    1 => parameter_1,
                    _ => panic!("Unknown parameter mode {}", parameter_1_mode),
                };
                let value_2 = match parameter_2_mode {
                    0 => self.memory[parameter_2 as usize],
                    1 => parameter_2,
                    _ => panic!("Unknown parameter mode {}", parameter_2_mode),
                };

                self.memory[parameter_3 as usize] = value_1 + value_2;

                true
            }
            2 => {
                // Multiply
                let parameter_1_mode = (self.memory[self.ip] / 100) & 1;
                let parameter_2_mode = (self.memory[self.ip] / 1000) & 1;

                let parameter_1 = self.memory[self.ip + 1];
                let parameter_2 = self.memory[self.ip + 2];
                let parameter_3 = self.memory[self.ip + 3];
                self.ip += 4;

                let value_1 = match parameter_1_mode {
                    0 => self.memory[parameter_1 as usize],
                    1 => parameter_1,
                    _ => panic!("Unknown parameter mode {}", parameter_1_mode),
                };
                let value_2 = match parameter_2_mode {
                    0 => self.memory[parameter_2 as usize],
                    1 => parameter_2,
                    _ => panic!("Unknown parameter mode {}", parameter_2_mode),
                };

                self.memory[parameter_3 as usize] = value_1 * value_2;

                true
            }
            3 => {
                // Input
                let parameter_1 = self.memory[self.ip + 1];
                self.ip += 2;

                self.memory[parameter_1 as usize] = input;

                true
            }
            4 => {
                // Output
                let parameter_1 = self.memory[self.ip + 1];
                self.ip += 2;

                println!("{}", self.memory[parameter_1 as usize]);

                true
            }
            99 => false,
            _ => {
                unimplemented!();
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let integers: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    // Part 1

    let mut program = Program::new(&integers);
    while program.run(1) {}
}
