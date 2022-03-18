mod computer;

use computer::Computer;
use std::cmp::Ordering;
use std::collections::{HashMap, HashSet};
use std::fmt;
use std::io::{stdin, Read};
use std::sync::mpsc::{channel, Receiver, Sender};
use std::thread;

#[derive(Debug, PartialOrd, Ord, PartialEq, Eq, Clone, Copy)]
enum Direction {
    North = 1,
    South = 2,
    West = 3,
    East = 4,
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Place {
    Open(usize),
    OxygenSystem,
    Wall,
}

impl PartialOrd for Place {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        use crate::Place::*;

        match (self, other) {
            (Open(self_visits), Open(other_visits)) => Some(self_visits.cmp(other_visits)),
            (Open(_), OxygenSystem) => Some(Ordering::Less),
            (Open(_), Wall) => Some(Ordering::Less),
            (OxygenSystem, Open(_)) => Some(Ordering::Greater),
            (OxygenSystem, OxygenSystem) => Some(Ordering::Equal),
            (OxygenSystem, Wall) => Some(Ordering::Less),
            (Wall, Open(_)) => Some(Ordering::Greater),
            (Wall, OxygenSystem) => Some(Ordering::Greater),
            (Wall, Wall) => Some(Ordering::Equal),
        }
    }
}

impl Place {
    fn as_char(&self) -> char {
        match self {
            Place::Open(_) => '.',
            Place::OxygenSystem => 'O',
            Place::Wall => '#',
        }
    }
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Position {
    x: isize,
    y: isize,
}

impl fmt::Display for Position {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", &format!("({},{})", self.x, self.y))
    }
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }

    fn neighbor(&self, direction: Direction) -> Position {
        match direction {
            Direction::North => Position::new(self.x, self.y - 1),
            Direction::South => Position::new(self.x, self.y + 1),
            Direction::West => Position::new(self.x - 1, self.y),
            Direction::East => Position::new(self.x + 1, self.y),
        }
    }
}

struct Map {
    places: HashMap<Position, Place>,
}

impl fmt::Display for Map {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        let (min_x, max_x, min_y, max_y) = self.extents();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                result.push(if x == 0 && y == 0 {
                    'X'
                } else if let Some(place) = self.places.get(&Position::new(x, y)) {
                    place.as_char()
                } else {
                    '?'
                });
            }
            result.push('\n');
        }

        write!(f, "{}", result)
    }
}

impl Map {
    fn new() -> Map {
        let places = HashMap::new();

        Map { places }
    }

    fn extents(&self) -> (isize, isize, isize, isize) {
        let x_min = self
            .places
            .iter()
            .map(|(position, _place)| position.x)
            .min()
            .unwrap();
        let x_max = self
            .places
            .iter()
            .map(|(position, _place)| position.x)
            .max()
            .unwrap();
        let y_min = self
            .places
            .iter()
            .map(|(position, _place)| position.y)
            .min()
            .unwrap();
        let y_max = self
            .places
            .iter()
            .map(|(position, _place)| position.y)
            .max()
            .unwrap();

        (x_min, x_max, y_min, y_max)
    }

    // Note that this function will leave visit values for Place::Open
    // in an arbitrary and unuseful state
    fn combine(&mut self, other: &Self) {
        for (&position, &place) in other.places.iter() {
            self.places.insert(position, place);
        }
    }

    fn oxygen_system(&self) -> Position {
        if let Some((position, _place)) = self
            .places
            .iter()
            .find(|(_position, place)| **place == Place::OxygenSystem)
        {
            *position
        } else {
            panic!("Oxygen system not located");
        }
    }

    fn open_positions(&self) -> HashSet<Position> {
        let mut open_positions = HashSet::new();

        for (&position, &place) in self.places.iter() {
            if let Place::Open(_) = place {
                open_positions.insert(position);
            }
        }

        open_positions
    }
}

struct Explorer {
    tx_computer: Sender<i64>,
    rx_master: Receiver<i64>,
    bias: Direction,
    map: Map,
}

impl Explorer {
    fn new(program: &[i64], bias: Direction) -> Explorer {
        let (tx_computer, rx_computer) = channel();
        let (tx_master, rx_master) = channel();

        let mut computer = Computer::new(program, rx_computer, tx_master, false);
        thread::spawn(move || computer.run());

        let map = Map::new();

        Explorer {
            tx_computer,
            rx_master,
            bias,
            map,
        }
    }

    fn explore(&mut self) {
        let mut position = Position::new(0, 0);
        let mut direction = self.bias;
        let mut target = position.neighbor(direction);

        loop {
            self.tx_computer
                .send(direction as i64)
                .expect("Failed send to computer");

            let status = match self.rx_master.recv() {
                Ok(integer) => integer,
                Err(_) => panic!("Failed receive from computer"),
            };

            match status {
                0 => {
                    self.map.places.insert(target, Place::Wall);

                    direction = self.choose_direction(position);
                    target = position.neighbor(direction);
                }
                1 => {
                    if let Some(Place::Open(visits)) = self.map.places.get_mut(&target) {
                        *visits += 1;
                    } else {
                        self.map.places.insert(target, Place::Open(1));
                    }

                    position = target;

                    direction = self.choose_direction(position);
                    target = position.neighbor(direction);
                }
                2 => {
                    self.map.places.insert(target, Place::OxygenSystem);

                    break;
                }
                _ => panic!("Unexpected status {} received from computer", status),
            }
        }

        self.tx_computer.send(99).expect("Failed send to computer");
    }

    fn choose_direction(&mut self, position: Position) -> Direction {
        use crate::Direction::*;

        let mut criteria = Vec::new();

        for direction in [North, South, West, East] {
            let neighbor = position.neighbor(direction);
            let place_weight = match self.map.places.get(&neighbor) {
                None => 0,
                Some(Place::Open(visits)) => 10 + visits,
                Some(Place::OxygenSystem) => 20,
                Some(Place::Wall) => 30,
            };
            let direction_preference = match (self.bias, direction) {
                (North, North) => 0,
                (North, East) => 1,
                (North, South) => 2,
                (North, West) => 3,
                (East, East) => 0,
                (East, South) => 1,
                (East, West) => 2,
                (East, North) => 3,
                (South, South) => 0,
                (South, West) => 1,
                (South, North) => 2,
                (South, East) => 3,
                (West, West) => 0,
                (West, North) => 1,
                (West, East) => 2,
                (West, South) => 3,
            };
            criteria.push((place_weight, direction_preference, direction));
        }

        criteria.sort_by(|a, b| match a.0.cmp(&b.0) {
            Ordering::Less => Ordering::Less,
            Ordering::Equal => a.1.cmp(&b.1),
            Ordering::Greater => Ordering::Greater,
        });

        criteria[0].2
    }

    fn fewest_movement_commands(&self) -> u32 {
        use crate::Direction::*;
        use crate::Place::*;

        let mut counter = 0;
        let mut path = Vec::new();
        let mut position = Position::new(0, 0);

        loop {
            let mut criteria = Vec::new();

            for direction in [North, South, West, East] {
                let neighbor = position.neighbor(direction);
                let visits = match self.map.places.get(&neighbor) {
                    Some(OxygenSystem) => 0,
                    Some(Open(visits)) => *visits,
                    Some(Wall) | None => 10,
                };
                criteria.push((visits, neighbor));
            }

            criteria = criteria
                .into_iter()
                .filter(|&criterion| !path.contains(&criterion.1))
                .collect();
            criteria.sort_by(|a, b| a.0.cmp(&b.0));

            position = criteria[0].1;
            path.push(position);
            counter += 1;

            if let Some(OxygenSystem) = self.map.places.get(&position) {
                break;
            }
        }

        counter
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let program: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    // Part 1

    let mut explorer = Explorer::new(&program, Direction::North);
    explorer.explore();
    println!(
        "Part 1: the fewest number of movement commands is {}",
        explorer.fewest_movement_commands()
    );

    // Part 2

    use crate::Direction::*;

    let mut complete_map = Map::new();

    for direction in [North, South, West, East] {
        let mut explorer = Explorer::new(&program, direction);
        explorer.explore();
        complete_map.combine(&explorer.map);
    }

    let mut minutes = 0;
    let mut open_positions = complete_map.open_positions();
    let mut vanguard_positions = vec![complete_map.oxygen_system()];

    while !open_positions.is_empty() {
        let mut new_vanguard_positions = Vec::new();

        while let Some(position) = vanguard_positions.pop() {
            for direction in [North, South, West, East] {
                let neighbor = position.neighbor(direction);
                if open_positions.remove(&neighbor) {
                    new_vanguard_positions.push(neighbor);
                }
            }
        }

        minutes += 1;
        vanguard_positions = new_vanguard_positions;
    }

    println!(
        "Part 2: it will take {} minutes to fill with oxygen",
        minutes
    );
}
