use std::io::{stdin, Read};

#[derive(Debug)]
struct Horizontal {
    y: i32,
    x0: i32,
    x1: i32,
}

#[derive(Debug)]
struct Vertical {
    x: i32,
    y0: i32,
    y1: i32,
}

#[derive(Debug)]
enum Segment {
    Horizontal(Horizontal),
    Vertical(Vertical),
}

impl Segment {
    fn intersects(&self, other: &Segment) -> Option<(i32, i32)> {
        match self {
            Segment::Horizontal(horizontal) => {
                if let Segment::Vertical(vertical) = other {
                    if [
                        horizontal.y >= vertical.y0,
                        horizontal.y <= vertical.y1,
                        vertical.x >= horizontal.x0,
                        vertical.x <= horizontal.x1,
                    ]
                    .iter()
                    .all(|&p| p)
                    {
                        return Some((vertical.x, horizontal.y));
                    }
                }
            }
            Segment::Vertical(vertical) => {
                if let Segment::Horizontal(horizontal) = other {
                    if [
                        horizontal.y >= vertical.y0,
                        horizontal.y <= vertical.y1,
                        vertical.x >= horizontal.x0,
                        vertical.x <= horizontal.x1,
                    ]
                    .iter()
                    .all(|&p| p)
                    {
                        return Some((vertical.x, horizontal.y));
                    }
                }
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

        for text in line.split(',') {
            let value = text[1..].parse::<i32>().unwrap();
            match text.chars().next() {
                Some('R') => {
                    let horizontal = Horizontal {
                        x0: x,
                        x1: x + value,
                        y,
                    };
                    segments.push(Segment::Horizontal(horizontal));
                    x += value;
                }
                Some('L') => {
                    let horizontal = Horizontal {
                        x0: x - value,
                        x1: x,
                        y,
                    };
                    segments.push(Segment::Horizontal(horizontal));
                    x -= value;
                }
                Some('U') => {
                    let vertical = Vertical {
                        x,
                        y0: y,
                        y1: y + value,
                    };
                    segments.push(Segment::Vertical(vertical));
                    y += value;
                }
                Some('D') => {
                    let vertical = Vertical {
                        x,
                        y0: y - value,
                        y1: y,
                    };
                    segments.push(Segment::Vertical(vertical));
                    y -= value;
                }
                _ => panic!("Unexpected text {}", text),
            }
        }

        Wire { segments }
    }

    fn intersections(&self, other: &Segment) -> Vec<(i32, i32)> {
        let mut intersections = Vec::new();

        for own in self.segments.iter() {
            if let Some((x, y)) = own.intersects(other) {
                intersections.push((x, y));
            }
        }

        intersections
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let lines: Vec<&str> = input.lines().collect();
    let wire_0 = Wire::new(lines[0]);
    let wire_1 = Wire::new(lines[1]);

    let mut minimum_distance = i32::MAX;

    for wire_1_segment in wire_1.segments.iter() {
        for (x, y) in wire_0.intersections(wire_1_segment) {
            if x == 0 && y == 0 {
                continue;
            }
            let distance = x.abs() + y.abs();
            if distance < minimum_distance {
                minimum_distance = distance;
            }
        }
    }

    println!(
        "Part 1: the Manhattan distance to the closest intersection is {}",
        minimum_distance
    );
}
