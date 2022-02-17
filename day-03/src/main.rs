use std::io::{stdin, Read};

#[derive(Debug)]
struct Horizontal {
    y: i32,
    x0: i32,
    x1: i32,
    rightward: bool,
    steps: i32,
}

#[derive(Debug)]
struct Vertical {
    x: i32,
    y0: i32,
    y1: i32,
    upward: bool,
    steps: i32,
}

#[derive(Debug)]
enum Segment {
    Horizontal(Horizontal),
    Vertical(Vertical),
}

impl Segment {
    fn intersects(&self, other: &Segment) -> Option<(i32, i32, i32)> {
        let segments = match self {
            Segment::Horizontal(horizontal) => match other {
                Segment::Horizontal(_) => None,
                Segment::Vertical(vertical) => Some((horizontal, vertical)),
            },
            Segment::Vertical(vertical) => match other {
                Segment::Horizontal(horizontal) => Some((horizontal, vertical)),
                Segment::Vertical(_) => None,
            },
        };

        if let Some((horizontal, vertical)) = segments {
            if [
                horizontal.y >= vertical.y0,
                horizontal.y <= vertical.y1,
                vertical.x >= horizontal.x0,
                vertical.x <= horizontal.x1,
            ]
            .iter()
            .all(|&p| p)
            {
                let horizontal_steps = horizontal.steps
                    + if horizontal.rightward {
                        vertical.x - horizontal.x0
                    } else {
                        horizontal.x1 - vertical.x
                    };
                let vertical_steps = vertical.steps
                    + if vertical.upward {
                        horizontal.y - vertical.y0
                    } else {
                        vertical.y1 - horizontal.y
                    };
                let steps = horizontal_steps + vertical_steps;
                return Some((vertical.x, horizontal.y, steps));
            }
        }

        None
    }
}

#[derive(Debug)]
struct Wire {
    segments: Vec<Segment>,
}

impl Wire {
    fn new(line: &str) -> Wire {
        let mut segments: Vec<Segment> = Vec::new();
        let mut x = 0;
        let mut y = 0;
        let mut steps = 0;

        for text in line.split(',') {
            let value = text[1..].parse::<i32>().unwrap();
            match text.chars().next() {
                Some('R') => {
                    let horizontal = Horizontal {
                        x0: x,
                        x1: x + value,
                        y,
                        rightward: true,
                        steps,
                    };
                    segments.push(Segment::Horizontal(horizontal));
                    x += value;
                    steps += value;
                }
                Some('L') => {
                    let horizontal = Horizontal {
                        x0: x - value,
                        x1: x,
                        y,
                        rightward: false,
                        steps,
                    };
                    segments.push(Segment::Horizontal(horizontal));
                    x -= value;
                    steps += value;
                }
                Some('U') => {
                    let vertical = Vertical {
                        x,
                        y0: y,
                        y1: y + value,
                        upward: true,
                        steps,
                    };
                    segments.push(Segment::Vertical(vertical));
                    y += value;
                    steps += value;
                }
                Some('D') => {
                    let vertical = Vertical {
                        x,
                        y0: y - value,
                        y1: y,
                        upward: false,
                        steps,
                    };
                    segments.push(Segment::Vertical(vertical));
                    y -= value;
                    steps += value;
                }
                _ => panic!("Unexpected text {}", text),
            }
        }

        Wire { segments }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let wire_a = Wire::new(lines[0]);
    let wire_b = Wire::new(lines[1]);

    // Part 1

    let mut minimum_distance = i32::MAX;

    for a in wire_a.segments.iter() {
        for b in wire_b.segments.iter() {
            if let Some((x, y, _steps)) = a.intersects(b) {
                if x == 0 && y == 0 {
                    continue;
                }
                let distance = x.abs() + y.abs();
                if distance < minimum_distance {
                    minimum_distance = distance;
                }
            }
        }
    }

    println!(
        "Part 1: the Manhattan distance to the closest intersection is {}",
        minimum_distance
    );

    // Part 2

    let mut minimum_steps = i32::MAX;

    for a in wire_a.segments.iter() {
        for b in wire_b.segments.iter() {
            if let Some((x, y, steps)) = a.intersects(b) {
                if x == 0 && y == 0 {
                    continue;
                }
                if steps < minimum_steps {
                    minimum_steps = steps;
                }
            }
        }
    }

    println!(
        "Part 2: the fewest combined steps to reach an intersection is {}",
        minimum_steps
    );
}
