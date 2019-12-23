use crate::utils::IntcodeMachine;
use std::cmp::Ordering;
use std::convert::TryInto;
use std::num::ParseIntError;

pub fn count_blocks() -> Result<usize, ParseIntError> {
    let input = include_str!("../input/day13.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let mut machine = IntcodeMachine::new(&input);
    let out = machine.execute(None).unwrap();
    let game = Game::new(&out);

    Ok(game.blocks())
}

pub fn end_score() -> Result<usize, ParseIntError> {
    let input = include_str!("../input/day13.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let mut machine = IntcodeMachine::new(&input);
    machine.set_memory(0, 2);
    let mut out = machine.execute(None).unwrap();
    let mut game = Game::new(&out);

    while game.blocks() > 0 {
        if machine.stopped() {
            panic!("AAAAAAAAAAAA: {} // {}", game.blocks(), game.score());
        }
        let (x, y) = game.ball();
        let (pad_x, pad_y) = game.paddle();
        let input = match pad_x.cmp(&x) {
            Ordering::Greater => -1,
            Ordering::Less => 1,
            Ordering::Equal => 0,
        };

        out = machine.execute(Some(&[input])).unwrap();
        game.update(&out);
    }

    Ok(game.score())
}

#[derive(Debug)]
struct Game {
    grid: Vec<Tile>,
    width: usize,
    score: usize,
}
impl Game {
    fn new(initial_state: &[i64]) -> Self {
        let grid: Vec<Tile> = initial_state
            .chunks_exact(3)
            .filter_map(|chunk| {
                let (x, y) = (chunk[0], chunk[1]);

                if (x, y) != (-1, 0) {
                    Some(Tile::from_int(chunk[2]))
                } else {
                    None
                }
            })
            .collect();

        let width = 1 + TryInto::<usize>::try_into(initial_state[initial_state.len() - 6]).unwrap();
        let height =
            1 + TryInto::<usize>::try_into(initial_state[initial_state.len() - 5]).unwrap();

        Game {
            grid,
            width,
            score: 0,
        }
    }
    fn set(&mut self, x: usize, y: usize, tile: Tile) {
        self.grid[y * self.width + x] = tile
    }
    fn blocks(&self) -> usize {
        self.grid
            .iter()
            .filter(|tile| **tile == Tile::Block)
            .count()
    }
    fn ball(&self) -> (usize, usize) {
        let pos = self
            .grid
            .iter()
            .enumerate()
            .find_map(|(i, tile)| if *tile == Tile::Ball { Some(i) } else { None })
            .unwrap();

        (pos % self.width, pos / self.width)
    }
    fn paddle(&self) -> (usize, usize) {
        let pos = self
            .grid
            .iter()
            .enumerate()
            .find_map(|(i, tile)| if *tile == Tile::Paddle { Some(i) } else { None })
            .unwrap();

        (pos % self.width, pos / self.width)
    }
    fn score(&self) -> usize {
        self.score
    }
    fn update(&mut self, new_info: &[i64]) {
        new_info.chunks_exact(3).for_each(|chunk| {
            let (x, y) = (chunk[0], chunk[1]);
            if (x, y) == (-1, 0) {
                self.score = chunk[2].try_into().unwrap();
            } else {
                self.set(
                    x.try_into().unwrap(),
                    y.try_into().unwrap(),
                    Tile::from_int(chunk[2]),
                );
            }
        });
    }
}

#[derive(Debug, PartialEq)]
enum Tile {
    Empty,
    Wall,
    Block,
    Paddle,
    Ball,
}
impl Tile {
    fn from_int(n: i64) -> Self {
        use Tile::*;

        match n {
            0 => Empty,
            1 => Wall,
            2 => Block,
            3 => Paddle,
            4 => Ball,
            _ => unreachable!(),
        }
    }
}
