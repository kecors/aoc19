enum Amount {
    One,
    Two,
    Many,
}

fn part_2_validate(digits: &[u32]) -> bool {
    let mut value = 0;
    let mut amount = Amount::One;

    for &digit in digits.iter() {
        match amount {
            Amount::One => {
                if digit == value {
                    amount = Amount::Two;
                } else {
                    value = digit;
                }
            }
            Amount::Two => {
                if digit == value {
                    amount = Amount::Many;
                } else {
                    return true;
                }
            }
            Amount::Many => {
                if digit != value {
                    amount = Amount::One;
                    value = digit;
                }
            }
        }
    }

    // Check if e and f produce a match
    matches!(amount, Amount::Two)
}

fn count_valid_passwords(range_begin: u32, range_end: u32, part_1: bool) -> u32 {
    let mut count = 0;

    for a in 1..=9 {
        for b in a..=9 {
            for c in b..=9 {
                for d in c..=9 {
                    for e in d..=9 {
                        for f in e..=9 {
                            let value = a * 100_000 + b * 10_000 + c * 1_000 + d * 100 + e * 10 + f;
                            if value < range_begin || value > range_end {
                                continue;
                            }

                            if part_1 {
                                if a == b || b == c || c == d || d == e || e == f {
                                    count += 1;
                                }
                            } else if part_2_validate(&[a, b, c, d, e, f]) {
                                count += 1;
                            }
                        }
                    }
                }
            }
        }
    }

    count
}

fn main() {
    // Part 1
    let count = count_valid_passwords(273_025, 767_253, true);
    println!("Part 1: there are {} different passwords", count);

    // Part 2
    let count = count_valid_passwords(273_025, 767_253, false);
    println!("Part 2: there are {} different passwords", count);
}
