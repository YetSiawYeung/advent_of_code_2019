use crate::utils::IntcodeMachine;
use std::{collections::HashSet, num::ParseIntError};

const GRID_LENGTH: usize = 200;

pub fn painted_at_least_once() -> Result<usize, ParseIntError> {
    // Part A
    let mut input = include_str!("../input/day11.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    let mut machine = IntcodeMachine::new(&input);
    let mut grid = Grid::new();
    let mut visited = HashSet::with_capacity(GRID_LENGTH);

    while !machine.stopped() {
        let input = grid.colour().int();

        let output = machine.execute(Some(&[input])).unwrap();
        let colour = Tile::from_int(output[0]);
        let direction = output[1];

        grid.paint_and_move(colour, direction);
        visited.insert(grid.location());
    }

    Ok(visited.len())
}
pub fn reggo() -> Result<String, ParseIntError> {
    // Part B not done
    let mut input = include_str!("../input/day11.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    let mut machine = IntcodeMachine::new(&input);
    let mut grid = Grid::new();

    let (x, y) = grid.location();
    grid.set(x, y, Tile::White);

    while !machine.stopped() {
        let input = grid.colour().int();

        let output = machine.execute(Some(&[input])).unwrap();
        let colour = Tile::from_int(output[0]);
        let direction = output[1];

        grid.paint_and_move(colour, direction);
    }

    todo!("fix intcode for valid output");
    Ok(grid.to_string())
}

#[derive(Debug, Clone, Copy, PartialEq)]
enum Tile {
    Black,
    White,
}
impl Tile {
    fn int(&self) -> i64 {
        match &self {
            Tile::Black => 0,
            Tile::White => 1,
        }
    }
    fn from_int(n: i64) -> Self {
        if n == 0 {
            Tile::Black
        } else {
            Tile::White
        }
    }
}

#[derive(Debug, Clone, Copy)]
enum Direction {
    Up,
    Left,
    Right,
    Down,
}
impl Direction {
    fn left_turn(self) -> Direction {
        use Direction::*;

        match self {
            Up => Left,
            Left => Down,
            Down => Right,
            Right => Up,
        }
    }

    fn right_turn(self) -> Direction {
        use Direction::*;

        match self {
            Up => Right,
            Right => Down,
            Down => Left,
            Left => Up,
        }
    }
}

#[derive(Debug)]
struct Grid {
    length: usize,
    grid: Vec<Tile>,
    robot: (Direction, usize, usize),
}
impl Grid {
    fn new() -> Self {
        Self {
            length: GRID_LENGTH,
            grid: vec![Tile::Black; GRID_LENGTH * GRID_LENGTH],
            robot: (Direction::Up, GRID_LENGTH / 2, GRID_LENGTH / 2),
        }
    }
    fn get(&self, x: usize, y: usize) -> Tile {
        self.grid[y * GRID_LENGTH + x]
    }
    fn set(&mut self, x: usize, y: usize, colour: Tile) {
        self.grid[y * GRID_LENGTH + x] = colour;
    }
    fn colour(&self) -> Tile {
        self.get(self.robot.1, self.robot.2)
    }
    fn paint_and_move(&mut self, colour: Tile, direction: i64) {
        self.grid[self.robot.2 * GRID_LENGTH + self.robot.1] = colour;
        self.robot.0 = match direction {
            0 => self.robot.0.left_turn(),
            1 => self.robot.0.right_turn(),
            _ => panic!("Invalid direction: {}", direction),
        };

        match self.robot.0 {
            Direction::Up => self.robot.1 -= 1,
            Direction::Down => self.robot.1 += 1,
            Direction::Left => self.robot.2 -= 1,
            Direction::Right => self.robot.2 += 1,
        };
    }
    fn location(&self) -> (usize, usize) {
        (self.robot.1, self.robot.2)
    }
    fn to_string(&self) -> String {
        self.grid
            .chunks_exact(GRID_LENGTH)
            .map(|row| {
                row.iter()
                    .map(|&tile| if tile == Tile::Black { ' ' } else { '█' })
                    .collect::<String>()
            })
            .collect::<Vec<_>>()
            .join("\n")
    }
}
