fn main() {
    let mut count = 0;

    for a in 1..=9 {
        for b in a..=9 {
            for c in b..=9 {
                for d in c..=9 {
                    for e in d..=9 {
                        for f in e..=9 {
                            let value = a * 100_000 + b * 10_000 + c * 1_000 + d * 100 + e * 10 + f;
                            if value < 273_035 || value > 767_253 {
                                continue;
                            }
                            if a != b && b != c && c != d && d != e && e != f {
                                continue;
                            }

                            count += 1;
                        }
                    }
                }
            }
        }
    }

    println!("Part 1: there are {} different passwords", count);
}
