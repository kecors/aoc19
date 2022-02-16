use std::io::{stdin, Read};

#[derive(Debug)]
struct Program {
    memory: Vec<u32>,
    ip: usize,
}

impl Program {
    fn new(int_list: &[u32]) -> Program {
        Program {
            memory: int_list.iter().cloned().collect(),
            ip: 0
        }
    }

    fn operate(&mut self) -> bool {
        match self.memory[self.ip] {
            1 => {
                let index1: usize = self.memory[self.ip + 1] as usize;
                let val1 = self.memory[index1];
                let index2: usize = self.memory[self.ip + 2] as usize;
                let val2 = self.memory[index2];
                let index3: usize = self.memory[self.ip + 3] as usize;
                self.memory[index3] = val1 + val2;
                self.ip = (self.ip + 4) % self.memory.len();
                true
            },
            2 => {
                let index1: usize = self.memory[self.ip + 1] as usize;
                let val1 = self.memory[index1];
                let index2: usize = self.memory[self.ip + 2] as usize;
                let val2 = self.memory[index2];
                let index3: usize = self.memory[self.ip + 3] as usize;
                self.memory[index3] = val1 * val2;
                self.ip = (self.ip + 4) % self.memory.len();
                true
            },
            99 => {
                false
            },
            _ => {
                unimplemented!();
            },
        }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let int_list: Vec<u32> = input
        .split(",")
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    // Part 1
    let mut program_p1 = Program::new(&int_list);
    program_p1.memory[1] = 12;
    program_p1.memory[2] = 2;
    while program_p1.operate() { }
    println!("Part 1: the value at position 0 is {}", program_p1.memory[0]);

    'outer: for noun in 0..=99 {
        for verb in 0..=99 {
            let mut program = Program::new(&int_list);
            program.memory[1] = noun;
            program.memory[2] = verb;
            while program.operate() { }
            if 19690720 == program.memory[0] {
                println!("Part 2: 100 * noun + verb = {}", 100 * noun + verb);
                break 'outer;
            }
        }
    }
}
