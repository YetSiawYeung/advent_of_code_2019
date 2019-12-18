use crate::utils::IntcodeMachine;
use std::num::ParseIntError;

pub fn diag_air_conditioner_unit() -> Result<Vec<i64>, ParseIntError> {
    // Part A
    let mut input = include_str!("../input/day5.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(IntcodeMachine::create_and_execute(&input, Some(&[1])).unwrap())
}

pub fn diag_thermal_radiator_controller() -> Result<Vec<i64>, ParseIntError> {
    // Part B
    let mut input = include_str!("../input/day5.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(IntcodeMachine::create_and_execute(&input, Some(&[5])).unwrap())
}
