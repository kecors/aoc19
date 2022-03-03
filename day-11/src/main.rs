mod computer;

use computer::Computer;
use std::collections::HashSet;
use std::io::{stdin, Read};
use std::sync::mpsc::channel;
use std::thread;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

#[derive(Debug)]
enum Orientation {
    North,
    East,
    South,
    West,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let program: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    // Part 1

    let (tx_computer, rx_computer) = channel();
    let (tx_master, rx_master) = channel();

    let mut computer = Computer::new(&program, rx_computer, tx_master, false);
    thread::spawn(move || computer.run());

    let mut whites = HashSet::new();
    let mut painted_positions = HashSet::new();
    let mut position = Position { x: 0, y: 0 };
    let mut orientation = Orientation::North;

    loop {
        let mut color = if whites.contains(&position) { 1 } else { 0 };
        if tx_computer.send(color).is_err() {
            break;
        }

        color = match rx_master.recv() {
            Ok(new_color) => new_color,
            Err(_) => break,
        };
        match color {
            0 => whites.remove(&position),
            1 => whites.insert(position),
            _ => panic!("Received unexpected color {}", color),
        };
        painted_positions.insert(position);

        let turn = match rx_master.recv() {
            Ok(turn) => turn,
            Err(_) => break,
        };
        orientation = match (turn, &orientation) {
            (0, Orientation::North) => Orientation::West,
            (1, Orientation::North) => Orientation::East,
            (0, Orientation::East) => Orientation::North,
            (1, Orientation::East) => Orientation::South,
            (0, Orientation::South) => Orientation::East,
            (1, Orientation::South) => Orientation::West,
            (0, Orientation::West) => Orientation::South,
            (1, Orientation::West) => Orientation::North,
            _ => panic!("Unexpected turn {} and orientation {:?}", turn, orientation),
        };
        match orientation {
            Orientation::North => position.y -= 1,
            Orientation::East => position.x += 1,
            Orientation::South => position.y += 1,
            Orientation::West => position.x -= 1,
        }
    }

    println!(
        "Part 1: {} panels are painted at least once",
        painted_positions.len()
    );

    if false {
        let min_x = whites.iter().map(|position| position.x).min().unwrap();
        let max_x = whites.iter().map(|position| position.x).max().unwrap();
        let min_y = whites.iter().map(|position| position.y).min().unwrap();
        let max_y = whites.iter().map(|position| position.y).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!(
                    "{}",
                    if whites.contains(&Position { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }

    // Part 2

    let (tx_computer, rx_computer) = channel();
    let (tx_master, rx_master) = channel();

    let mut computer = Computer::new(&program, rx_computer, tx_master, false);
    thread::spawn(move || computer.run());

    let mut whites = HashSet::new();
    let mut position = Position { x: 0, y: 0 };
    let mut orientation = Orientation::North;

    whites.insert(position);

    loop {
        let mut color = if whites.contains(&position) { 1 } else { 0 };
        if tx_computer.send(color).is_err() {
            break;
        }

        color = match rx_master.recv() {
            Ok(new_color) => new_color,
            Err(_) => break,
        };
        match color {
            0 => whites.remove(&position),
            1 => whites.insert(position),
            _ => panic!("Received unexpected color {}", color),
        };

        let turn = match rx_master.recv() {
            Ok(turn) => turn,
            Err(_) => break,
        };
        orientation = match (turn, &orientation) {
            (0, Orientation::North) => Orientation::West,
            (1, Orientation::North) => Orientation::East,
            (0, Orientation::East) => Orientation::North,
            (1, Orientation::East) => Orientation::South,
            (0, Orientation::South) => Orientation::East,
            (1, Orientation::South) => Orientation::West,
            (0, Orientation::West) => Orientation::South,
            (1, Orientation::West) => Orientation::North,
            _ => panic!("Unexpected turn {} and orientation {:?}", turn, orientation),
        };
        match orientation {
            Orientation::North => position.y -= 1,
            Orientation::East => position.x += 1,
            Orientation::South => position.y += 1,
            Orientation::West => position.x -= 1,
        }
    }

    if true {
        let min_x = whites.iter().map(|position| position.x).min().unwrap();
        let max_x = whites.iter().map(|position| position.x).max().unwrap();
        let min_y = whites.iter().map(|position| position.y).min().unwrap();
        let max_y = whites.iter().map(|position| position.y).max().unwrap();
        for y in min_y..=max_y {
            for x in min_x..=max_x {
                print!(
                    "{}",
                    if whites.contains(&Position { x, y }) {
                        '#'
                    } else {
                        '.'
                    }
                );
            }
            println!();
        }
    }
}
