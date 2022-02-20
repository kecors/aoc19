use std::io::{stdin, Read};
use std::result::Result;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Debug)]
struct Amplifier {
    memory: Vec<i32>,
    ip: usize,
    rx: Receiver<i32>,
    tx: Sender<i32>,
}

impl Amplifier {
    fn new(program: &[i32], rx: Receiver<i32>, tx: Sender<i32>) -> Amplifier {
        Amplifier {
            memory: program.to_vec(),
            ip: 0,
            rx,
            tx,
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn step(&mut self) -> bool {
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

                self.memory[parameter_1 as usize] = self.rx.recv().unwrap();

                true
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

                let _ = self.tx.send(value_1);

                true
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

                true
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

                true
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

                true
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

                true
            }
            99 => false,
            _ => {
                unimplemented!();
            }
        }
    }
}

fn run(program: &[i32], phase_settings: &[i32]) -> i32 {
    let (tx_a, rx_a) = channel();
    let (tx_b, rx_b) = channel();
    let (tx_c, rx_c) = channel();
    let (tx_d, rx_d) = channel();
    let (tx_e, rx_e) = channel();

    let (tx_master, rx_master) = channel();

    tx_a.send(phase_settings[0]).unwrap();
    tx_b.send(phase_settings[1]).unwrap();
    tx_c.send(phase_settings[2]).unwrap();
    tx_d.send(phase_settings[3]).unwrap();
    tx_e.send(phase_settings[4]).unwrap();

    let mut amplifier_a = Amplifier::new(program, rx_a, tx_b);
    let mut amplifier_b = Amplifier::new(program, rx_b, tx_c);
    let mut amplifier_c = Amplifier::new(program, rx_c, tx_d);
    let mut amplifier_d = Amplifier::new(program, rx_d, tx_e);
    let mut amplifier_e = Amplifier::new(program, rx_e, tx_master);

    thread::spawn(move || amplifier_a.run());
    thread::spawn(move || amplifier_b.run());
    thread::spawn(move || amplifier_c.run());
    thread::spawn(move || amplifier_d.run());
    thread::spawn(move || amplifier_e.run());

    tx_a.send(0).unwrap();
    loop {
        let value = rx_master.recv().unwrap();
        match tx_a.send(value) {
            Result::Ok(_) => (),
            Result::Err(_) => return value,
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let program: Vec<i32> = input
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
                        let output = run(&program, &[a, b, c, d, e]);
                        if output > maximum_output {
                            maximum_output = output;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: the highest signal is {}", maximum_output);

    // Part 2
    let mut maximum_output = 0;

    for a in 5..=9 {
        for b in 5..=9 {
            if b == a {
                continue;
            }
            for c in 5..=9 {
                if c == a || c == b {
                    continue;
                }
                for d in 5..=9 {
                    if d == a || d == b || d == c {
                        continue;
                    }
                    for e in 5..=9 {
                        if e == a || e == b || e == c || e == d {
                            continue;
                        }
                        let output = run(&program, &[a, b, c, d, e]);
                        if output > maximum_output {
                            maximum_output = output;
                        }
                    }
                }
            }
        }
    }

    println!("Part 2: the highest signal is {}", maximum_output);
}
