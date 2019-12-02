pub fn intcode() -> u32 {
    // part a: return element at index 0 after executing intcode
    let mut input: Vec<u32> = include_str!("../input/day2.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    input[1] = 12;
    input[2] = 2;

    execute_intcode(&mut input);

    return input[0];
}
pub fn intcode_2() -> u32 {
    // part b: pair of values (a,b) such that output is 19690720
    // return 100 * a + b
    let target = 19690720;
    let mut input: Vec<u32> = include_str!("../input/day2.txt")
        .trim()
        .split(',')
        .map(|x| x.parse().unwrap())
        .collect();

    for a in 0..100 {
        for b in 0..100 {
            input[1] = a;
            input[2] = b;
            let mut temp = input.clone();
            execute_intcode(&mut temp);
            if temp[0] == target {
                println!("{} {}", a, b);
                return 100 * a + b;
            };
        }
    }

    panic!("Could not find values to produce target output")
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
