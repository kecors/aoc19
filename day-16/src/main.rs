use std::io::{stdin, Read};

fn fft(digits: &[i32]) -> Vec<i32> {
    let mut result = Vec::new();

    for j in 0..digits.len() {
        let mut pattern_contents = Vec::new();
        pattern_contents.append(&mut vec![0; j + 1]);
        pattern_contents.append(&mut vec![1; j + 1]);
        pattern_contents.append(&mut vec![0; j + 1]);
        pattern_contents.append(&mut vec![-1; j + 1]);
        let mut pattern_cycle = pattern_contents.iter().cycle();
        pattern_cycle.next();

        let mut sum = 0;
        for &digit in digits.iter() {
            sum += digit * pattern_cycle.next().unwrap();
        }

        let digit = sum.abs() % 10;
        result.push(digit);
    }

    result
}

fn print_first_eight(digits: &[i32]) {
    for digit in digits.iter().take(8) {
        print!("{}", digit);
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let mut digits: Vec<i32> = input
        .trim()
        .chars()
        .map(|ch| ch.to_digit(10).unwrap() as i32)
        .collect();

    for _ in 0..100 {
        digits = fft(&digits);
    }

    print!("Part 1: the first eight digits are ");
    print_first_eight(&digits);
    println!();
}
