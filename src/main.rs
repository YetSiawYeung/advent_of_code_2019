#![allow(unused)]

extern crate bytecount;

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;
mod day8;

use day8::decode;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{}", decode().unwrap());

    Ok(())
}
