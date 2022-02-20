use std::io::{stdin, Read};

#[derive(Debug)]
enum Status {
    Normal,
    Output(i32),
    Terminated,
}

#[derive(Debug)]
struct Program {
    memory: Vec<i32>,
    ip: usize,
    input: Vec<i32>,
}

impl Program {
    fn new(integers: &[i32], input: Vec<i32>) -> Program {
        Program {
            memory: integers.to_vec(),
            ip: 0,
            input,
        }
    }

    fn step(&mut self) -> Status {
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

                Status::Normal
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

                Status::Normal
            }
            3 => {
                // Input
                let parameter_1 = self.memory[self.ip + 1];
                self.ip += 2;

                self.memory[parameter_1 as usize] = self.input.pop().unwrap();

                Status::Normal
            }
            4 => {
                // Output
                let parameter_1_mode = (self.memory[self.ip] / 100) & 1;

                let parameter_1 = self.memory[self.ip + 1];
                self.ip += 2;

                let value_1 = match parameter_1_mode {
                    0 => self.memory[parameter_1 as usize],
                    1 => parameter_1,
                    _ => panic!("Unknown parameter mode {}", parameter_1_mode),
                };

                Status::Output(value_1)
            }
            5 => {
                // Jump if true
                let parameter_1_mode = (self.memory[self.ip] / 100) & 1;
                let parameter_2_mode = (self.memory[self.ip] / 1000) & 1;

                let parameter_1 = self.memory[self.ip + 1];
                let parameter_2 = self.memory[self.ip + 2];
                self.ip += 3;

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

                if value_1 != 0 {
                    self.ip = value_2 as usize;
                }

                Status::Normal
            }
            6 => {
                // Jump if false
                let parameter_1_mode = (self.memory[self.ip] / 100) & 1;
                let parameter_2_mode = (self.memory[self.ip] / 1000) & 1;

                let parameter_1 = self.memory[self.ip + 1];
                let parameter_2 = self.memory[self.ip + 2];
                self.ip += 3;

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

                if value_1 == 0 {
                    self.ip = value_2 as usize;
                }

                Status::Normal
            }
            7 => {
                // Less than
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

                self.memory[parameter_3 as usize] = if value_1 < value_2 { 1 } else { 0 };

                Status::Normal
            }
            8 => {
                // Equal
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

                self.memory[parameter_3 as usize] = if value_1 == value_2 { 1 } else { 0 };

                Status::Normal
            }
            99 => Status::Terminated,
            _ => {
                unimplemented!();
            }
        }
    }
}

fn run(integers: &[i32], phase_settings: &[i32]) -> i32 {
    let mut input = 0;

    for &phase_setting in phase_settings.iter() {
        let mut program = Program::new(integers, vec![input, phase_setting]);
        loop {
            match program.step() {
                Status::Normal => (),
                Status::Output(output) => input = output,
                Status::Terminated => break,
            }
        }
    }

    input
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let integers: Vec<i32> = input
        .split(',')
        .map(|x| x.trim().parse::<i32>().unwrap())
        .collect();

    // Part 1
    let mut maximum_output = 0;

    for a in 0..=4 {
        for b in 0..=4 {
            if b == a {
                continue;
            }
            for c in 0..=4 {
                if c == a || c == b {
                    continue;
                }
                for d in 0..=4 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 0..=4 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let output = run(&integers, &[a, b, c, d, e]);
                        if output > maximum_output {
                            maximum_output = output;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: the highest signal is {}", maximum_output);
}
