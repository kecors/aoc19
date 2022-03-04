extern crate pest;
#[macro_use]
extern crate pest_derive;

use pest::Parser;
use std::cmp::Ordering;
use std::fmt;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "space.pest"]
struct SpaceParser;

#[derive(Debug, Clone, Copy)]
struct Delta {
    x: isize,
    y: isize,
    z: isize,
}

impl Delta {
    fn new() -> Delta {
        Delta { x: 0, y: 0, z: 0 }
    }
}

#[derive(Debug)]
struct Moon {
    x: isize,
    y: isize,
    z: isize,
    velocity_x: isize,
    velocity_y: isize,
    velocity_z: isize,
}

impl fmt::Display for Moon {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "pos=<x={:>2}, y={:>2}, z={:>2}>, vel=<x={:>2}, y={:>2}, z={:>2}>",
            self.x, self.y, self.z, self.velocity_x, self.velocity_y, self.velocity_z
        )
    }
}

impl Moon {
    fn new(x: isize, y: isize, z: isize) -> Moon {
        let velocity_x = 0;
        let velocity_y = 0;
        let velocity_z = 0;

        Moon {
            x,
            y,
            z,
            velocity_x,
            velocity_y,
            velocity_z,
        }
    }

    fn potential_energy(&self) -> u32 {
        (self.x.abs() + self.y.abs() + self.z.abs()) as u32
    }

    fn kinetic_energy(&self) -> u32 {
        (self.velocity_x.abs() + self.velocity_y.abs() + self.velocity_z.abs()) as u32
    }

    fn total_energy(&self) -> u32 {
        self.potential_energy() * self.kinetic_energy()
    }
}

#[derive(Debug)]
struct Space {
    moons: Vec<Moon>,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for moon in self.moons.iter() {
            result.push_str(&format!("{}\n", moon));
        }

        write!(f, "{}", result)
    }
}

impl Space {
    fn new(input: &str) -> Space {
        let pairs = SpaceParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut moons = Vec::new();
        let mut x: isize = 0;
        let mut y: isize = 0;
        let mut z: isize;

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::x => {
                    x = text.parse::<isize>().unwrap();
                }
                Rule::y => {
                    y = text.parse::<isize>().unwrap();
                }
                Rule::z => {
                    z = text.parse::<isize>().unwrap();
                    let moon = Moon::new(x, y, z);
                    moons.push(moon);
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        Space { moons }
    }

    fn step(&mut self) {
        // Apply gravity
        let mut deltas = vec![Delta::new(); self.moons.len()];

        for index_a in 0..self.moons.len() {
            for index_b in (index_a + 1)..self.moons.len() {
                match self.moons[index_a].x.cmp(&self.moons[index_b].x) {
                    Ordering::Less => {
                        deltas[index_a].x += 1;
                        deltas[index_b].x -= 1;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        deltas[index_a].x -= 1;
                        deltas[index_b].x += 1;
                    }
                }
                match self.moons[index_a].y.cmp(&self.moons[index_b].y) {
                    Ordering::Less => {
                        deltas[index_a].y += 1;
                        deltas[index_b].y -= 1;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        deltas[index_a].y -= 1;
                        deltas[index_b].y += 1;
                    }
                }
                match self.moons[index_a].z.cmp(&self.moons[index_b].z) {
                    Ordering::Less => {
                        deltas[index_a].z += 1;
                        deltas[index_b].z -= 1;
                    }
                    Ordering::Equal => {}
                    Ordering::Greater => {
                        deltas[index_a].z -= 1;
                        deltas[index_b].z += 1;
                    }
                }
            }
        }
        for index in 0..self.moons.len() {
            self.moons[index].velocity_x += deltas[index].x;
            self.moons[index].velocity_y += deltas[index].y;
            self.moons[index].velocity_z += deltas[index].z;
        }

        // Apply velocity
        for index in 0..self.moons.len() {
            self.moons[index].x += self.moons[index].velocity_x;
            self.moons[index].y += self.moons[index].velocity_y;
            self.moons[index].z += self.moons[index].velocity_z;
        }
    }

    fn total_energy(&self) -> u32 {
        self.moons.iter().map(|moon| moon.total_energy()).sum()
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut space = Space::new(&input);
    for _step in 1..=1000 {
        space.step();
        //println!("After {} steps:\n{}", step, space);
    }
    println!("Part 1: the total energy is {}", space.total_energy());
}
