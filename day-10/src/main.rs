use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Clone)]
enum State {
    Unscanned,
    Detectable,
    Undetectable,
    Vaporized,
}

#[derive(Clone)]
struct Neighbor {
    x: usize,
    y: usize,
    delta_x: isize,
    delta_y: isize,
    state: State,
}

impl fmt::Display for Neighbor {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            &format!(
                "<{},{}> ({},{}) {:?}",
                self.x, self.y, self.delta_x, self.delta_y, self.state
            )
        )
    }
}

impl Neighbor {
    fn new(x: usize, y: usize, scanner_x: usize, scanner_y: usize) -> Neighbor {
        let delta_x = x as isize - scanner_x as isize;
        let delta_y = y as isize - scanner_y as isize;
        let state = State::Unscanned;

        Neighbor {
            x,
            y,
            delta_x,
            delta_y,
            state,
        }
    }

    fn is_hidden_by(&self, other: &Neighbor) -> bool {
        if (self.delta_x > 0) != (other.delta_x > 0) {
            false
        } else if (self.delta_y > 0) != (other.delta_y > 0) {
            false
        } else if self.delta_x == 0 {
            other.delta_x == 0 && self.delta_y.abs() > other.delta_y.abs()
        } else if self.delta_y == 0 {
            other.delta_y == 0 && self.delta_x.abs() > other.delta_x.abs()
        } else if self.delta_x.abs() <= other.delta_x.abs() {
            false
        } else if self.delta_y.abs() <= other.delta_y.abs() {
            false
        } else {
            self.delta_x * other.delta_y == other.delta_x * self.delta_y
        }
    }
}

#[derive(Debug)]
enum Direction {
    Up,
    Left,
    Down,
    Right,
}

#[derive(Debug)]
enum Quadrant {
    UpperRight,
    LowerRight,
    LowerLeft,
    UpperLeft,
}

struct Scanner {
    x: usize,
    y: usize,
    neighbors: Vec<Neighbor>,
    vaporizations: Vec<Neighbor>,
}

impl fmt::Display for Scanner {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        result.push_str(&format!("scanner at ({},{}):\n", self.x, self.y));

        for neighbor in self.neighbors.iter() {
            result.push_str(&format!("\t{}\n", neighbor));
        }

        if !self.vaporizations.is_empty() {
            result.push_str("Vaporizations:\n");
        }
        for (index, vaporization) in self.vaporizations.iter().enumerate() {
            result.push_str(&format!("\t{}: {}\n", index, vaporization));
        }

        write!(f, "{}", result)
    }
}

impl Scanner {
    fn new(x: usize, y: usize, positions: &[(usize, usize)]) -> Scanner {
        let mut neighbors = Vec::new();

        for position in positions.iter() {
            if x == position.0 && y == position.1 {
                continue;
            }
            neighbors.push(Neighbor::new(position.0, position.1, x, y));
        }

        let vaporizations = Vec::new();

        Scanner {
            x,
            y,
            neighbors,
            vaporizations,
        }
    }

    fn scan(&mut self) {
        for j in 0..self.neighbors.len() {
            if self.neighbors[j].state == State::Vaporized {
                continue;
            }
            self.neighbors[j].state = State::Detectable;

            for k in 0..self.neighbors.len() {
                if j == k {
                    continue;
                }
                if self.neighbors[k].state == State::Vaporized {
                    continue;
                }
                if self.neighbors[j].is_hidden_by(&self.neighbors[k]) {
                    self.neighbors[j].state = State::Undetectable;
                }
            }
        }
    }

    fn detectable_positions(&self) -> usize {
        self.neighbors
            .iter()
            .filter(|neighbor| neighbor.state == State::Detectable)
            .count()
    }

    fn vaporize_all_asteroids(&mut self) {
        loop {
            self.scan();

            self.vaporize_direction(Direction::Up);
            self.vaporize_quadrant(Quadrant::UpperRight, false);
            self.vaporize_direction(Direction::Right);
            self.vaporize_quadrant(Quadrant::LowerRight, true);
            self.vaporize_direction(Direction::Down);
            self.vaporize_quadrant(Quadrant::LowerLeft, false);
            self.vaporize_direction(Direction::Left);
            self.vaporize_quadrant(Quadrant::UpperLeft, true);

            if self
                .neighbors
                .iter()
                .all(|neighbor| neighbor.state == State::Vaporized)
            {
                break;
            }
        }
    }

    fn vaporize_direction(&mut self, direction: Direction) {
        if let Some(neighbor) = self
            .neighbors
            .iter_mut()
            .filter(|neighbor| neighbor.state == State::Detectable)
            .find(|neighbor| match direction {
                Direction::Up => neighbor.delta_x == 0 && neighbor.delta_y < 0,
                Direction::Left => neighbor.delta_y == 0 && neighbor.delta_x > 0,
                Direction::Down => neighbor.delta_x == 0 && neighbor.delta_y > 0,
                Direction::Right => neighbor.delta_y == 0 && neighbor.delta_x < 0,
            })
        {
            neighbor.state = State::Vaporized;
            self.vaporizations.push(neighbor.clone());
        }
    }

    fn vaporize_quadrant(&mut self, quadrant: Quadrant, flip: bool) {
        let mut indexes = Vec::new();
        for (index, _neighbor) in self
            .neighbors
            .iter()
            .enumerate()
            .filter(|(_index, neighbor)| neighbor.state == State::Detectable)
            .filter(|(_index, neighbor)| match quadrant {
                Quadrant::UpperRight => neighbor.delta_x > 0 && neighbor.delta_y < 0,
                Quadrant::LowerRight => neighbor.delta_x > 0 && neighbor.delta_y > 0,
                Quadrant::LowerLeft => neighbor.delta_x < 0 && neighbor.delta_y > 0,
                Quadrant::UpperLeft => neighbor.delta_x < 0 && neighbor.delta_y < 0,
            })
        {
            indexes.push(index);
        }

        indexes.sort_by(|&a, &b| {
            let a_delta_x: f32 = self.neighbors[a].delta_x.abs() as f32;
            let a_delta_y: f32 = self.neighbors[a].delta_y.abs() as f32;
            let a_ratio = a_delta_x / a_delta_y;

            let b_delta_x: f32 = self.neighbors[b].delta_x.abs() as f32;
            let b_delta_y: f32 = self.neighbors[b].delta_y.abs() as f32;
            let b_ratio = b_delta_x / b_delta_y;

            if flip {
                b_ratio.partial_cmp(&a_ratio).unwrap()
            } else {
                a_ratio.partial_cmp(&b_ratio).unwrap()
            }
        });

        for &index in indexes.iter() {
            self.neighbors[index].state = State::Vaporized;
            self.vaporizations.push(self.neighbors[index].clone());
        }
    }
}

struct Region {
    scanners: Vec<Scanner>,
}

impl fmt::Display for Region {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        for scanner in self.scanners.iter() {
            result.push_str(&format!("{}", scanner));
        }

        write!(f, "{}", result)
    }
}

impl Region {
    fn new(input: &str) -> Region {
        let mut positions: Vec<(usize, usize)> = Vec::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                if ch != '.' {
                    positions.push((x, y));
                }
            });
        });

        let mut scanners = Vec::new();
        for position in positions.iter() {
            let mut scanner = Scanner::new(position.0, position.1, &positions);
            scanner.scan();
            scanners.push(scanner);
        }

        Region { scanners }
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut region = Region::new(&input);
    let best_scanner = region
        .scanners
        .iter_mut()
        .max_by(|a, b| a.detectable_positions().cmp(&b.detectable_positions()))
        .unwrap();
    println!(
        "Part 1: {} asteroids can be detected from the best station",
        best_scanner.detectable_positions()
    );

    // Part 2

    best_scanner.vaporize_all_asteroids();
    if let Some(vaporization) = best_scanner.vaporizations.get(199) {
        let product = vaporization.x * 100 + vaporization.y;
        println!("Part 2: the product for the 200th asteroid is {}", product);
    }
}
