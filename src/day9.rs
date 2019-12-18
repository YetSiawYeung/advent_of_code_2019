use crate::utils::IntcodeMachine;
use std::convert::TryFrom;
use std::num::ParseIntError;

pub fn boost_keycode() -> Result<Vec<i64>, ParseIntError> {
    // Part A
    let mut input = include_str!("../input/day9.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(IntcodeMachine::create_and_execute(&input, Some(&[1])).unwrap())
}
pub fn sensor_boost() -> Result<Vec<i64>, ParseIntError> {
    // Part B
    let mut input = include_str!("../input/day9.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(IntcodeMachine::create_and_execute(&input, Some(&[2])).unwrap())
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_full_intcode() {
        let arr = vec![
            109, 1, 204, -1, 1001, 100, 1, 100, 1008, 100, 16, 101, 1006, 101, 0, 99,
        ];
        let mut machine = IntcodeMachine::new(&arr);
        assert_eq!(machine.execute(None), Some(arr));

        IntcodeMachine::new(&[1102, 34915192, 34915192, 7, 4, 7, 99, 0]).execute(None);

        assert_eq!(
            IntcodeMachine::new(&[104, 1125899906842624, 99]).execute(None),
            Some(vec![1125899906842624])
        );
    }
}
