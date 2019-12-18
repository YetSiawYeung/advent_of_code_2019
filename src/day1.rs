use std::num::ParseIntError;

pub fn total_fuel() -> Result<u32, ParseIntError> {
    // Part A
    let input = include_str!("../input/day1.txt");

    input.lines().map(|i| i.parse().map(fuel)).sum()
}

pub fn total_fuel2() -> Result<u32, ParseIntError> {
    // Part B
    let input = include_str!("../input/day1.txt");

    input.lines().map(|i| i.parse().map(fuel2)).sum()
}

fn fuel(n: u32) -> u32 {
    (n / 3).saturating_sub(2)
}

fn fuel2(n: u32) -> u32 {
    let mut total = 0;
    let mut required = fuel(n);
    while required > 0 {
        total += required;
        required = fuel(required);
    }
    total
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_fuel() {
        assert_eq!(fuel(0), 0);
        assert_eq!(fuel(12), 2);
        assert_eq!(fuel(14), 2);
        assert_eq!(fuel(1969), 654);
        assert_eq!(fuel(100756), 33583);
    }

    #[test]
    fn test_fuel2() {
        assert_eq!(fuel2(14), 2);
        assert_eq!(fuel2(1969), 966);
        assert_eq!(fuel2(100756), 50346);
    }
}
