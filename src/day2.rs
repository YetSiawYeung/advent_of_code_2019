use crate::utils::IntcodeMachine;
use std::num::ParseIntError;

pub fn intcode() -> Result<i64, ParseIntError> {
    // Part A: return element at index 0 after executing intcode
    let mut input = include_str!("../input/day2.txt")
        .trim()
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    input[1] = 12;
    input[2] = 2;

    IntcodeMachine::create_and_execute(&input, None);

    Ok(input[0])
}

pub fn intcode_2() -> Result<Option<i64>, ParseIntError> {
    // Part B: find pair of values (a,b) such that output is 19690720
    // return 100 * a + b
    let target = 19_690_720;
    let mut input = include_str!("../input/day2.txt")
        .trim()
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, _>>()?;

    for a in 0..100 {
        for b in 0..100 {
            input[1] = a;
            input[2] = b;
            let mut temp = input.clone();
            IntcodeMachine::create_and_execute(&input, None);
            if temp[0] == target {
                return Ok(Some(100 * a + b));
            };
        }
    }

    Ok(None)
}
