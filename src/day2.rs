use std::num::ParseIntError;

pub fn intcode() -> Result<u32, ParseIntError> {
    // Part A: return element at index 0 after executing intcode
    let mut input = include_str!("../input/day2.txt")
        .trim()
        .split(',')
        .map(|x| x.parse())
        .collect::<Result<Vec<_>, ParseIntError>>()?;

    input[1] = 12;
    input[2] = 2;

    execute_intcode(&mut input);

    Ok(input[0])
}

pub fn intcode_2() -> Result<Option<u32>, ParseIntError> {
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
            execute_intcode(&mut temp);
            if temp[0] == target {
                return Ok(Some(100 * a + b));
            };
        }
    }

    Ok(None)
}

fn execute_intcode(v: &mut [u32]) {
    let mut counter = 0;
    loop {
        match v[counter] {
            1 => {
                v[v[counter + 3] as usize] =
                    v[v[counter + 1] as usize] + v[v[counter + 2] as usize];
                counter += 4;
            }
            2 => {
                v[v[counter + 3] as usize] =
                    v[v[counter + 1] as usize] * v[v[counter + 2] as usize];
                counter += 4;
            }
            99 => return,
            _ => panic!("unreachable"),
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intcode() {
        let mut v = vec![1, 0, 0, 0, 99];
        execute_intcode(&mut v);
        assert_eq!(v, vec![2, 0, 0, 0, 99]);

        let mut v = vec![2, 3, 0, 3, 99];
        execute_intcode(&mut v);
        assert_eq!(v, vec![2, 3, 0, 6, 99]);

        let mut v = vec![2, 4, 4, 5, 99, 0];
        execute_intcode(&mut v);
        assert_eq!(v, vec![2, 4, 4, 5, 99, 9801]);

        let mut v = vec![1, 1, 1, 4, 99, 5, 6, 0, 99];
        execute_intcode(&mut v);
        assert_eq!(v, vec![30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }
}
