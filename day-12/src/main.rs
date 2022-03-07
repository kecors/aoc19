use pest::Parser;
use pest_derive::Parser;
use std::cmp::Ordering;
use std::fmt;
use std::io::{stdin, Read};

#[derive(Parser)]
#[grammar = "space.pest"]
struct SpaceParser;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Axis {
    positions: Vec<isize>,
    velocities: Vec<isize>,
}

impl Axis {
    fn new(positions: Vec<isize>) -> Axis {
        let velocities = vec![0; positions.len()];

        Axis {
            positions,
            velocities,
        }
    }

    fn apply_gravity(&mut self) {
        for (index_a, position_a) in self.positions.iter().enumerate() {
            for position_b in self.positions.iter() {
                self.velocities[index_a] += match position_a.cmp(position_b) {
                    Ordering::Less => 1,
                    Ordering::Equal => 0,
                    Ordering::Greater => -1,
                };
            }
        }
    }

    fn apply_velocity(&mut self) {
        for (index, position) in self.positions.iter_mut().enumerate() {
            *position += self.velocities[index];
        }
    }

    fn count_to_reset(&mut self) -> u64 {
        let baseline = self.clone();
        let mut count = 0;
        loop {
            count += 1;
            self.apply_gravity();
            self.apply_velocity();
            if *self == baseline {
                break;
            }
        }

        count
    }
}

#[derive(Debug)]
struct Space {
    x_axis: Axis,
    y_axis: Axis,
    z_axis: Axis,
}

impl fmt::Display for Space {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for index in 0..self.x_axis.positions.len() {
            result.push_str(&format!(
                "pos=<x={:>3}, y={:>3}, z={:>3}>, vel=<{:>3}, y={:>3}, z={:>3}>\n",
                self.x_axis.positions[index],
                self.y_axis.positions[index],
                self.z_axis.positions[index],
                self.x_axis.velocities[index],
                self.y_axis.velocities[index],
                self.z_axis.velocities[index]
            ));
        }

        write!(f, "{}", result)
    }
}

impl Space {
    fn new(input: &str) -> Space {
        let pairs = SpaceParser::parse(Rule::main, input).unwrap_or_else(|e| panic!("{}", e));

        let mut x_positions = Vec::new();
        let mut y_positions = Vec::new();
        let mut z_positions = Vec::new();

        for pair in pairs {
            let rule = pair.as_rule();
            let text = pair.clone().as_span().as_str().to_string();

            match rule {
                Rule::x => {
                    x_positions.push(text.parse::<isize>().unwrap());
                }
                Rule::y => {
                    y_positions.push(text.parse::<isize>().unwrap());
                }
                Rule::z => {
                    z_positions.push(text.parse::<isize>().unwrap());
                }
                _ => {
                    panic!("Unknown rule {:?} with {:?}", rule, text);
                }
            }
        }

        Space {
            x_axis: Axis::new(x_positions),
            y_axis: Axis::new(y_positions),
            z_axis: Axis::new(z_positions),
        }
    }

    fn step(&mut self) {
        self.x_axis.apply_gravity();
        self.x_axis.apply_velocity();
        self.y_axis.apply_gravity();
        self.y_axis.apply_velocity();
        self.z_axis.apply_gravity();
        self.z_axis.apply_velocity();
    }

    fn total_energy(&self) -> isize {
        let mut sum = 0;

        for index in 0..self.x_axis.positions.len() {
            let potential = self.x_axis.positions[index].abs()
                + self.y_axis.positions[index].abs()
                + self.z_axis.positions[index].abs();
            let kinetic = self.x_axis.velocities[index].abs()
                + self.y_axis.velocities[index].abs()
                + self.z_axis.velocities[index].abs();
            let product = potential * kinetic;
            sum += product;
        }

        sum
    }
}

fn gcd(mut a: u64, mut b: u64) -> u64 {
    while a > 0 {
        let temp = a;
        a = b % a;
        b = temp;
    }

    b
}

fn lcm(a: u64, b: u64) -> u64 {
    a * b / gcd(a, b)
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut space = Space::new(&input);
    //println!("After 0 steps:\n{}", space);
    for _step in 1..=1000 {
        space.step();
        //println!("After {} steps:\n{}", step, space);
    }
    println!("Part 1: the total energy is {}", space.total_energy());

    // Part 2

    let mut space = Space::new(&input);
    let x_count = space.x_axis.count_to_reset();
    let y_count = space.y_axis.count_to_reset();
    let z_count = space.z_axis.count_to_reset();
    let steps = lcm(x_count, lcm(y_count, z_count));
    println!("Part 2: it takes {} steps", steps);
}
