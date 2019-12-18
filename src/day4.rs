use std::num::ParseIntError;

pub fn num_valid_passwords() -> Result<usize, ParseIntError> {
    // Part A
    let input = include_str!("../input/day4.txt")
        .split('-')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let range = input[0]..input[1];

    Ok(range.filter(|&i| is_valid(i)).count())
}

pub fn num_valid_passwords2() -> Result<usize, ParseIntError> {
    // Part B
    let input = include_str!("../input/day4.txt")
        .split('-')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let range = input[0]..input[1];

    Ok(range.filter(|&i| is_valid2(i)).count())
}

fn is_valid(password: u32) -> bool {
    if password > 999_999 || password < 100_000 {
        // 6 digits long
        return false;
    }

    let mut doubled_digits = false;
    let mut not_decreasing = true;
    let mut password = password;

    let mut current_digit = password % 10;

    while password > 10 {
        let next_digit = (password / 10) % 10;

        if next_digit == current_digit {
            doubled_digits = true;
        } else if next_digit > current_digit {
            not_decreasing = false;
        }

        current_digit = next_digit;
        password /= 10;
    }

    doubled_digits && not_decreasing
}

fn is_valid2(password: u32) -> bool {
    if password > 999_999 || password < 100_000 {
        // 6 digits long
        return false;
    }

    let mut doubled_digits = false;
    let mut two_digits = false;
    let mut more_than_two = false;
    let mut not_decreasing = true;
    let mut password = password;

    let mut current_digit = password % 10;

    while password > 10 {
        let next_digit = (password / 10) % 10;

        if next_digit == current_digit {
            if two_digits {
                more_than_two = true;
            }
            // doubled_digits = true;
            else {
                two_digits = true;
                more_than_two = false;
            }
        } else if next_digit > current_digit {
            not_decreasing = false;
        }

        if next_digit != current_digit && two_digits && !more_than_two {
            two_digits = false;
            doubled_digits = true;
        }

        if next_digit != current_digit {
            more_than_two = false;
            two_digits = false
        }

        current_digit = next_digit;
        password /= 10;
    }

    if two_digits && !more_than_two {
        doubled_digits = true;
    }

    doubled_digits && not_decreasing
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_valid() {
        assert!(is_valid(111111));
        assert!(!is_valid(223450));
        assert!(!is_valid(123789));
    }

    #[test]
    fn test_valid2() {
        assert!(is_valid2(112233));
        assert!(!is_valid2(123444));
        assert!(is_valid2(111122));
        assert!(is_valid2(668889));
        assert!(!is_valid2(666666));
    }
}
