use std::collections::HashMap;
use std::io::{stdin, Read};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let digits: Vec<u32> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap())
        .collect();

    let mut minimum_zero_distribution = (u32::MAX, u32::MAX, u32::MAX);
    let mut layer_base = 0;
    loop {
        let mut digit_counts: HashMap<u32, u32> = HashMap::new();

        for &digit in digits.iter().skip(layer_base).take(LAYER_SIZE) {
            let o = digit_counts.entry(digit).or_insert(0);
            *o += 1;
        }

        let &zeros = digit_counts.get(&0).unwrap();
        let &ones = digit_counts.get(&1).unwrap();
        let &twos = digit_counts.get(&2).unwrap();

        if zeros < minimum_zero_distribution.0 {
            minimum_zero_distribution = (zeros, ones, twos);
        }

        layer_base += LAYER_SIZE;
        if layer_base >= digits.len() {
            break;
        }
    }

    let product = minimum_zero_distribution.1 * minimum_zero_distribution.2;
    println!("Part 1: the product is {}", product);
}
