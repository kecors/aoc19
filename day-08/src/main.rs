use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, Read};

const WIDTH: usize = 25;
const HEIGHT: usize = 6;
const LAYER_SIZE: usize = WIDTH * HEIGHT;

struct Image {
    layers: Vec<Vec<u8>>,
}

impl fmt::Display for Image {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut visible_digits = vec![2; LAYER_SIZE];

        for layer in self.layers.iter() {
            for y in 0..HEIGHT {
                for x in 0..WIDTH {
                    let offset = y * WIDTH + x;
                    if visible_digits[offset] == 2 {
                        visible_digits[offset] = layer[offset];
                    }
                }
            }
        }

        let mut result = String::new();

        for y in 0..HEIGHT {
            for x in 0..WIDTH {
                let offset = y * WIDTH + x;
                result.push(match visible_digits[offset] {
                    0 => ' ',
                    1 => '#',
                    _ => panic!("Unexpected visible digit {}", visible_digits[offset]),
                });
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

impl Image {
    fn new(digits: &[u8]) -> Image {
        let mut layers: Vec<Vec<u8>> = Vec::new();
        let mut layer_base = 0;

        loop {
            let layer: Vec<u8> = digits
                .iter()
                .cloned()
                .skip(layer_base)
                .take(LAYER_SIZE)
                .collect();

            layers.push(layer);

            layer_base += LAYER_SIZE;
            if layer_base >= digits.len() {
                break;
            }
        }

        Image { layers }
    }

    fn calculate_distributions(&self) -> Vec<(u32, u32, u32)> {
        let mut distributions = Vec::new();

        for layer in self.layers.iter() {
            let mut digit_counts: HashMap<u8, u32> = HashMap::new();

            for &digit in layer.iter() {
                let o = digit_counts.entry(digit).or_insert(0);
                *o += 1;
            }

            let &zeros = digit_counts.get(&0).unwrap();
            let &ones = digit_counts.get(&1).unwrap();
            let &twos = digit_counts.get(&2).unwrap();

            distributions.push((zeros, ones, twos));
        }

        distributions
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let digits: Vec<u8> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as u8)
        .collect();

    let image = Image::new(&digits);

    // Part 1

    let distributions = image.calculate_distributions();
    let minimum_zero_distribution = distributions
        .iter()
        .reduce(|acc, item| if acc.0 < item.0 { acc } else { item })
        .unwrap();
    let product = minimum_zero_distribution.1 * minimum_zero_distribution.2;
    println!("Part 1: the product is {}", product);

    // Part 2

    println!("{}", image);
}
