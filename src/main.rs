#![allow(unused)]

mod day1;
mod day2;
mod day3;
mod day4;
mod day5;
mod day6;
mod day7;

use day7::max_signal_feedback;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    println!("{:?}", max_signal_feedback()?);

    Ok(())
}
