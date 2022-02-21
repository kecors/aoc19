use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, Read};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

enum Mode {
    Position,
    Immediate,
    Relative,
}

struct Parameter {
    integer: i64,
    mode: Mode,
    address: usize,
    value: i64,
}

impl fmt::Display for Parameter {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            match self.mode {
                Mode::Position => format!("[P|{}|{}]", self.address, self.value),
                Mode::Immediate => format!("[I|{}]", self.value),
                Mode::Relative => format!("[R|{}|{}]", self.address, self.value),
            }
        )
    }
}

struct Computer {
    memory: HashMap<usize, i64>,
    ip: usize,
    rb: i64,
    rx: Receiver<i64>,
    tx: Sender<i64>,
    debug_flag: bool,
}

impl Computer {
    fn new(program: &[i64], rx: Receiver<i64>, tx: Sender<i64>, debug_flag: bool) -> Computer {
        let mut memory = HashMap::new();

        for (index, &integer) in program.iter().enumerate() {
            memory.insert(index, integer);
        }

        Computer {
            memory,
            ip: 0,
            rb: 0,
            rx,
            tx,
            debug_flag,
        }
    }

    fn run(&mut self) {
        while self.step() {}
    }

    fn get_opcode(&self) -> i64 {
        if let Some(&integer) = self.memory.get(&self.ip) {
            integer
        } else {
            panic!("Opcode not found at ip {}", self.ip);
        }
    }

    fn get_parameter(&self, offset: usize) -> Parameter {
        let integer = if let Some(&integer) = self.memory.get(&(self.ip + offset)) {
            integer
        } else {
            panic!("Parameter not found at offset {} of ip {}", offset, self.ip);
        };

        let mode = if let Some(&integer) = self.memory.get(&self.ip) {
            let mode_value = match offset {
                1 => (integer / 100) % 10,
                2 => (integer / 1000) % 10,
                3 => (integer / 10000) % 10,
                _ => panic!("Unsupported offset {}", offset),
            };
            match mode_value {
                0 => Mode::Position,
                1 => Mode::Immediate,
                2 => Mode::Relative,
                _ => panic!("Unsupported mode {}", mode_value),
            }
        } else {
            panic!("Parameter mode not found for ip {}", self.ip);
        };

        let (address, value) = match mode {
            Mode::Position => {
                let address = integer as usize;
                let value = if let Some(&value) = self.memory.get(&address) {
                    value
                } else {
                    0
                };
                (address, value)
            }
            Mode::Immediate => {
                let address = self.ip + offset;
                let value = integer;
                (address, value)
            }
            Mode::Relative => {
                let address = (self.rb + integer) as usize;
                let value = if let Some(&value) = self.memory.get(&address) {
                    value
                } else {
                    0
                };
                (address, value)
            }
        };

        Parameter {
            integer,
            mode,
            address,
            value,
        }
    }

    fn set_value(&mut self, address: usize, value: i64) {
        self.memory.insert(address, value);
    }

    fn step(&mut self) -> bool {
        match self.get_opcode() % 100 {
            1 => {
                // Add
                let parameter_1 = self.get_parameter(1);
                let parameter_2 = self.get_parameter(2);
                let parameter_3 = self.get_parameter(3);

                let new_value = parameter_1.value + parameter_2.value;

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} {} {} | add | {} + {} | {}: {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_2.integer,
                        parameter_3.integer,
                        parameter_1,
                        parameter_2,
                        parameter_3.address,
                        parameter_3.value,
                        new_value
                    );
                }

                self.set_value(parameter_3.address, new_value);
                self.ip += 4;

                true
            }
            2 => {
                // Multiply
                let parameter_1 = self.get_parameter(1);
                let parameter_2 = self.get_parameter(2);
                let parameter_3 = self.get_parameter(3);

                let new_value = parameter_1.value * parameter_2.value;

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} {} {} | multiply | {} * {} | {}: {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_2.integer,
                        parameter_3.integer,
                        parameter_1,
                        parameter_2,
                        parameter_3.address,
                        parameter_3.value,
                        new_value
                    );
                }

                self.set_value(parameter_3.address, new_value);
                self.ip += 4;

                true
            }
            3 => {
                // Input
                let parameter_1 = self.get_parameter(1);

                let new_value = self.rx.recv().unwrap();

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} | input | address {} <<< input | {}: {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_1.address,
                        parameter_1.address,
                        parameter_1.value,
                        new_value
                    );
                }

                self.set_value(parameter_1.address, new_value);
                self.ip += 2;

                true
            }
            4 => {
                // Output
                let parameter_1 = self.get_parameter(1);

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} | output | >>> {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_1.value
                    );
                }

                let _ = self.tx.send(parameter_1.value);
                self.ip += 2;

                true
            }
            5 => {
                // Jump if true
                let parameter_1 = self.get_parameter(1);
                let parameter_2 = self.get_parameter(2);

                let new_ip = if parameter_1.value != 0 {
                    parameter_2.value as usize
                } else {
                    self.ip + 3
                };

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} {} | jump if true | if {} != 0 then ip = {} | ip {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_2.integer,
                        parameter_1,
                        parameter_2,
                        self.ip,
                        new_ip
                    );
                }

                self.ip = new_ip;

                true
            }
            6 => {
                // Jump if false
                let parameter_1 = self.get_parameter(1);
                let parameter_2 = self.get_parameter(2);

                let new_ip = if parameter_1.value == 0 {
                    parameter_2.value as usize
                } else {
                    self.ip + 3
                };

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} {} | jump if false | if {} == 0 then ip = {} | ip {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_2.integer,
                        parameter_1,
                        parameter_2,
                        self.ip,
                        new_ip
                    );
                }

                self.ip = new_ip;

                true
            }
            7 => {
                // Less than
                let parameter_1 = self.get_parameter(1);
                let parameter_2 = self.get_parameter(2);
                let parameter_3 = self.get_parameter(3);

                let new_value = if parameter_1.value < parameter_2.value {
                    1
                } else {
                    0
                };

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} {} {} | less than | if {} < {} then 1 else 0 | {}: {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_2.integer,
                        parameter_3.integer,
                        parameter_1,
                        parameter_2,
                        parameter_3.address,
                        parameter_3.value,
                        new_value
                    );
                }

                self.set_value(parameter_3.address, new_value);
                self.ip += 4;

                true
            }
            8 => {
                // Equal
                let parameter_1 = self.get_parameter(1);
                let parameter_2 = self.get_parameter(2);
                let parameter_3 = self.get_parameter(3);

                let new_value = if parameter_1.value == parameter_2.value {
                    1
                } else {
                    0
                };

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} {} {} | equal | if {} == {} then 1 else 0 | {}: {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_2.integer,
                        parameter_3.integer,
                        parameter_1,
                        parameter_2,
                        parameter_3.address,
                        parameter_3.value,
                        new_value
                    );
                }

                self.set_value(parameter_3.address, new_value);
                self.ip += 4;

                true
            }
            9 => {
                // Adjust relative base
                let parameter_1 = self.get_parameter(1);

                let new_rb = self.rb + parameter_1.value;

                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> {} | adjust relative base | rb += {} | rb {} => {}",
                        self.ip,
                        self.rb,
                        self.get_opcode(),
                        parameter_1.integer,
                        parameter_1,
                        self.rb,
                        new_rb
                    );
                }

                self.rb = new_rb;
                self.ip += 2;

                true
            }
            99 => {
                // Halt
                if self.debug_flag {
                    println!(
                        "ip: {:>3} rb: {:>4} | <{}> | halt",
                        self.ip,
                        self.rb,
                        self.get_opcode()
                    );
                }

                false
            }
            _ => {
                unimplemented!();
            }
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let program: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    // Part 1

    let (tx_computer, rx_computer) = channel();
    let (tx_master, rx_master) = channel();

    let mut computer = Computer::new(&program, rx_computer, tx_master, false);
    thread::spawn(move || computer.run());

    tx_computer.send(1).unwrap();
    while let Ok(keycode) = rx_master.recv() {
        println!("Part 1: the BOOST keycode is {}", keycode);
    }
}
