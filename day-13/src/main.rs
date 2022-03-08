mod computer;

use computer::Computer;
use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt;
use std::io::{stdin, Read};
use std::sync::mpsc::channel;
use std::thread;

#[derive(Debug, PartialEq, Eq, Hash)]
struct Position {
    x: isize,
    y: isize,
}

impl Position {
    fn new(x: isize, y: isize) -> Position {
        Position { x, y }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}

struct Game {
    program: Vec<i64>,
    screen: HashMap<Position, Tile>,
    score: i64,
    paddle: Position,
    ball: Position,
}

impl fmt::Display for Game {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let mut result = String::new();

        let (x_min, x_max, y_min, y_max) = self.screen_extents();
        for y in y_min..=y_max {
            for x in x_min..=x_max {
                let tile = if let Some(tile) = self.screen.get(&Position::new(x, y)) {
                    *tile
                } else {
                    Tile::Empty
                };
                result.push(match tile {
                    Tile::Empty => ' ',
                    Tile::Wall => '#',
                    Tile::Block => '-',
                    Tile::Paddle => '=',
                    Tile::Ball => 'o',
                });
            }
            result.push('\n');
        }

        result.push_str(&format!("\n\t\t\tSCORE [{}]", self.score));

        write!(f, "{}", result)
    }
}

impl Game {
    fn new(input: &str) -> Game {
        let program: Vec<i64> = input
            .split(',')
            .map(|x| x.trim().parse::<i64>().unwrap())
            .collect();
        let screen = HashMap::new();
        let score = 0;
        let paddle = Position::new(0, 0);
        let ball = Position::new(0, 0);

        Game {
            program,
            screen,
            score,
            paddle,
            ball,
        }
    }

    fn insert_quarters(&mut self) {
        self.program[0] = 2;
    }

    fn run(&mut self, play: bool) {
        let (tx_computer, rx_computer) = channel();
        let (tx_master, rx_master) = channel();

        let mut computer = Computer::new(&self.program, rx_computer, tx_master, false);
        thread::spawn(move || computer.run());

        loop {
            let mut redraw = false;

            let x = match rx_master.recv() {
                Ok(integer) => integer as isize,
                Err(_) => break,
            };
            let y = match rx_master.recv() {
                Ok(integer) => integer as isize,
                Err(_) => break,
            };
            if x == -1 && y == 0 {
                self.score = match rx_master.recv() {
                    Ok(integer) => integer,
                    Err(_) => break,
                };
            } else {
                let tile = match rx_master.recv() {
                    Ok(integer) => match integer {
                        0 => Tile::Empty,
                        1 => Tile::Wall,
                        2 => Tile::Block,
                        3 => Tile::Paddle,
                        4 => Tile::Ball,
                        _ => panic!("Unexpected tile {}", integer),
                    },
                    Err(_) => break,
                };

                self.screen.insert(Position::new(x, y), tile);

                match tile {
                    Tile::Paddle => {
                        self.paddle = Position::new(x, y);
                        redraw = true;
                    }
                    Tile::Ball => {
                        self.ball = Position::new(x, y);
                        redraw = true;
                        let joystick = match self.paddle.x.cmp(&self.ball.x) {
                            Ordering::Less => 1,
                            Ordering::Equal => 0,
                            Ordering::Greater => -1,
                        };
                        match tx_computer.send(joystick) {
                            Ok(_) => (),
                            Err(_) => break,
                        }
                    }
                    _ => (),
                }
            }

            if play && redraw {
                println!("{}", self);
            }
        }
    }

    fn block_count(&self) -> usize {
        self.screen
            .iter()
            .filter(|(_, tile)| **tile == Tile::Block)
            .count()
    }

    fn screen_extents(&self) -> (isize, isize, isize, isize) {
        let x_min = self
            .screen
            .iter()
            .map(|(position, _)| position.x)
            .min()
            .unwrap();
        let x_max = self
            .screen
            .iter()
            .map(|(position, _)| position.x)
            .max()
            .unwrap();
        let y_min = self
            .screen
            .iter()
            .map(|(position, _)| position.y)
            .min()
            .unwrap();
        let y_max = self
            .screen
            .iter()
            .map(|(position, _)| position.y)
            .max()
            .unwrap();

        (x_min, x_max, y_min, y_max)
    }
}

fn main() {
    let mut input = String::new();
    stdin().read_to_string(&mut input).unwrap();

    // Part 1

    let mut game = Game::new(&input);
    game.run(false);
    println!(
        "Part 1: there are {} block tiles on the screen",
        game.block_count()
    );

    // Part 2

    let mut game = Game::new(&input);
    game.insert_quarters();
    game.run(false);
    println!("Part 2: the final score is {}", game.score);
}
