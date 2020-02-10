use std::io::{stdin, Read};

#[derive(Debug)]
struct Program {
    ints: Vec<u32>,
    base: usize,
}

impl Program {
    fn operate(&mut self) -> bool {
        dbg!(&self.base);
        match self.ints[self.base] {
            1 => {
                let index1: usize = self.ints[self.base + 1] as usize;
                let val1 = self.ints[index1];
                let index2: usize = self.ints[self.base + 2] as usize;
                let val2 = self.ints[index2];
                let index3: usize = self.ints[self.base + 3] as usize;
                self.ints[index3] = val1 + val2;
                self.base = (self.base + 4) % self.ints.len();
                true
            },
            2 => {
                let index1: usize = self.ints[self.base + 1] as usize;
                let val1 = self.ints[index1];
                let index2: usize = self.ints[self.base + 2] as usize;
                let val2 = self.ints[index2];
                let index3: usize = self.ints[self.base + 3] as usize;
                self.ints[index3] = val1 * val2;
                self.base = (self.base + 4) % self.ints.len();
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

    let ints: Vec<u32> = input
        .split(",")
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    let mut program = Program { ints: ints, base: 0 };

    program.ints[1] = 12;
    program.ints[2] = 2;

    while program.operate() {
        //println!("{:?}", program);
    }

    println!("{:?}", program);
}
