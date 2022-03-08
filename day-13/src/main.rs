mod computer;

use computer::Computer;
use std::collections::HashMap;
use std::io::{stdin, Read};
use std::sync::mpsc::channel;
use std::thread;

#[derive(Debug, PartialEq, Eq)]
enum TileId {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    let program: Vec<i64> = input
        .split(',')
        .map(|x| x.trim().parse::<i64>().unwrap())
        .collect();

    // Part 1

    let (_tx_computer, rx_computer) = channel();
    let (tx_master, rx_master) = channel();

    let mut computer = Computer::new(&program, rx_computer, tx_master, false);
    thread::spawn(move || computer.run());

    let mut screen: HashMap<(usize, usize), TileId> = HashMap::new();

    loop {
        let x = match rx_master.recv() {
            Ok(integer) => integer as usize,
            Err(_) => break,
        };
        let y = match rx_master.recv() {
            Ok(integer) => integer as usize,
            Err(_) => break,
        };
        let tile_id = match rx_master.recv() {
            Ok(integer) => match integer {
                0 => TileId::Empty,
                1 => TileId::Wall,
                2 => TileId::Block,
                3 => TileId::Paddle,
                4 => TileId::Ball,
                _ => panic!("Unexpected tile id {}", integer),
            },
            Err(_) => break,
        };
        screen.insert((x, y), tile_id);
    }

    let block_count = screen
        .iter()
        .filter(|((_x, _y), tile_id)| **tile_id == TileId::Block)
        .count();
    println!(
        "Part 1: there are {} block tiles on the screen",
        block_count
    );
}
