use std::collections::HashMap;
use std::io::{stdin, Read};

#[derive(Debug)]
struct Calculator {
    fuel_reqs: HashMap<u32, u32>,
}

impl Calculator {
    fn calculate_fuel(mass: u32) -> u32 {
        if mass <= 6 {
            return 0;
        }
        (mass / 3) - 2
    }

    fn new() -> Calculator {
        let mut fuel_reqs: HashMap<u32, u32> = HashMap::new();
        fuel_reqs.insert(0, 0);

        Calculator { fuel_reqs }
    }

    fn calculate_fuel_extended(&mut self, mass: u32) -> u32 {
        if let Some(requirement) = self.fuel_reqs.get(&mass) {
            return *requirement;
        }

        let base: u32 = Calculator::calculate_fuel(mass);
        let extension: u32 = self.calculate_fuel_extended(base);

        self.fuel_reqs.insert(mass, base + extension);

        base + extension
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let masses: Vec<u32> = input
        .lines()
        .map(|x| x.trim().parse::<u32>().unwrap())
        .collect();

    let mut calculator = Calculator::new();

    let total_fuel_part1 = masses
        .iter()
        .fold(0, |acc, x| acc + Calculator::calculate_fuel(*x));

    println!(
        "Part 1: the sum of the fuel requirements is {}",
        total_fuel_part1
    );

    let total_fuel_part2 = masses
        .iter()
        .fold(0, |acc, x| acc + calculator.calculate_fuel_extended(*x));

    println!(
        "Part 2: the sum of the fuel requirements is {}",
        total_fuel_part2
    );
}
