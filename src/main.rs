#![allow(unused)]

extern crate bytecount;

mod day1;
mod day10;
mod day11;
mod day12;
mod day13;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;
mod day9;
mod utils;

use day13::end_score;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", end_score()?);

    Ok(())
}
