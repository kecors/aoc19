use std::collections::HashSet;
use std::fmt;
use std::io::{stdin, Read};

#[derive(Debug, PartialEq, Eq, Hash, Clone)]
struct Position {
    x: usize,
    y: usize,
}

impl Position {
    fn new(x: usize, y: usize) -> Position {
        Position { x, y }
    }

    fn relation_to(&self, other: &Position) -> Relation {
        let x_delta = other.x as isize - self.x as isize;
        let y_delta = other.y as isize - self.y as isize;

        Relation {
            position: other.clone(),
            x_delta,
            y_delta,
            visible: true,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash)]
struct Relation {
    position: Position,
    x_delta: isize,
    y_delta: isize,
    visible: bool,
}

impl fmt::Display for Relation {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "{}",
            &format!(
                "({},{}): ({},{}) {:?}",
                self.position.x, self.position.y, self.x_delta, self.y_delta, self.visible
            )
        )
    }
}

impl Relation {
    fn is_hidden_by(&self, other: &Relation) -> bool {
        if (self.x_delta > 0) != (other.x_delta > 0) {
            false
        } else if (self.y_delta > 0) != (other.y_delta > 0) {
            false
        } else if self.x_delta == 0 {
            other.x_delta == 0 && self.y_delta.abs() > other.y_delta.abs()
        } else if self.y_delta == 0 {
            other.y_delta == 0 && self.x_delta.abs() > other.x_delta.abs()
        } else if self.x_delta.abs() <= other.x_delta.abs() {
            false
        } else if self.y_delta.abs() <= other.y_delta.abs() {
            false
        } else {
            self.x_delta * other.y_delta == other.x_delta * self.y_delta
        }
    }
}

#[derive(Debug)]
struct Region {
    positions: Vec<Position>,
}

impl Region {
    fn new(input: &str) -> Region {
        let mut positions: Vec<Position> = Vec::new();

        input.lines().enumerate().for_each(|(y, line)| {
            line.chars().enumerate().for_each(|(x, ch)| {
                if ch != '.' {
                    positions.push(Position::new(x, y));
                }
            });
        });

        Region { positions }
    }

    fn calculate_maximum_detectable_relations(&mut self) -> usize {
        let mut maximum_detectable_relations = 0;

        for position_1 in self.positions.iter() {
            let mut relations: Vec<Relation> = Vec::new();
            for position_2 in self.positions.iter() {
                if position_1 == position_2 {
                    continue;
                }
                relations.push(position_1.relation_to(position_2));
            }

            let mut undetectable_relations = HashSet::new();
            for (index, relation_1) in relations.iter().enumerate() {
                for relation_2 in relations.iter() {
                    if relation_1 == relation_2 {
                        continue;
                    }
                    if relation_1.is_hidden_by(relation_2) {
                        undetectable_relations.insert(index);
                        break;
                    }
                }
            }

            let detectable_relations = relations.len() - undetectable_relations.len();
            if detectable_relations > maximum_detectable_relations {
                maximum_detectable_relations = detectable_relations;
            }
        }

        maximum_detectable_relations
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut region = Region::new(&input);
    println!(
        "Part 1: {} asteroids can be detected from the best station",
        region.calculate_maximum_detectable_relations()
    );
}
