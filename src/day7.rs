use crate::utils::IntcodeMachine;
use std::num::ParseIntError;

pub fn max_signal() -> Result<i64, ParseIntError> {
    // Part A
    let mut input = include_str!("../input/day7.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(permutations(&mut vec![0, 1, 2, 3, 4])
        .iter()
        .map(|arr| calculate_signal(&mut input, arr))
        .max()
        .unwrap())
}
pub fn max_signal_feedback() -> Result<i64, ParseIntError> {
    // Part B
    let mut input = include_str!("../input/day7.txt")
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    Ok(permutations(&mut vec![5, 6, 7, 8, 9])
        .iter()
        .map(|arr| signal_feedback(&input, arr))
        .max()
        .unwrap())
}

fn calculate_signal(machine: &mut [i64], input: &[i64]) -> i64 {
    input.iter().fold(0, |acc, i| {
        IntcodeMachine::create_and_execute(machine, Some(&[*i, acc])).unwrap()[0]
    })
}

fn signal_feedback(machine: &[i64], input: &[i64]) -> i64 {
    let mut machines = input
        .iter()
        .map(|i| {
            let mut mac = IntcodeMachine::new(machine);
            mac.execute(Some(&[*i]));
            mac
        })
        .collect::<Vec<_>>();
    let mut acc = 0;
    while machines.iter().any(|machine| !machine.stopped()) {
        for (mac, i) in machines.iter_mut().zip(input.iter()) {
            acc = mac.execute(Some(&[acc])).unwrap()[0];
        }
    }
    acc
}

fn factorial(n: usize) -> usize {
    if n == 1 {
        1
    } else {
        n * factorial(n - 1)
    }
}

fn generate(k: usize, a: &mut Vec<i64>) -> Vec<Vec<i64>> {
    let mut ans = Vec::new();

    if k == 1 {
        ans.push((a).clone());
    } else {
        for i in 0..(k) {
            ans.append(&mut generate(k - 1, a));
            if k % 2 == 0 {
                a.swap(i, k - 1);
            } else {
                a.swap(0, k - 1);
            }
        }
    }
    ans
}
fn permutations(list: &mut Vec<i64>) -> Vec<Vec<i64>> {
    generate(list.len(), list)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_signal() {
        assert_eq!(
            calculate_signal(
                &mut [3, 15, 3, 16, 1002, 16, 10, 16, 1, 16, 15, 15, 4, 15, 99, 0, 0],
                &[4, 3, 2, 1, 0]
            ),
            43210
        );
        assert_eq!(
            calculate_signal(
                &mut [
                    3, 23, 3, 24, 1002, 24, 10, 24, 1002, 23, -1, 23, 101, 5, 23, 23, 1, 24, 23,
                    23, 4, 23, 99, 0, 0
                ],
                &[0, 1, 2, 3, 4]
            ),
            54321
        );
        assert_eq!(
            calculate_signal(
                &mut [
                    3, 31, 3, 32, 1002, 32, 10, 32, 1001, 31, -2, 31, 1007, 31, 0, 33, 1002, 33, 7,
                    33, 1, 33, 31, 31, 1, 32, 31, 31, 4, 31, 99, 0, 0, 0
                ],
                &[1, 0, 4, 3, 2]
            ),
            65210
        );
    }

    #[test]
    fn test_feedback() {
        assert_eq!(
            signal_feedback(
                &[
                    3, 26, 1001, 26, -4, 26, 3, 27, 1002, 27, 2, 27, 1, 27, 26, 27, 4, 27, 1001,
                    28, -1, 28, 1005, 28, 6, 99, 0, 0, 5
                ],
                &[9, 8, 7, 6, 5]
            ),
            139629729
        );
        assert_eq!(
            signal_feedback(
                &[
                    3, 52, 1001, 52, -5, 52, 3, 53, 1, 52, 56, 54, 1007, 54, 5, 55, 1005, 55, 26,
                    1001, 54, -5, 54, 1105, 1, 12, 1, 53, 54, 53, 1008, 54, 0, 55, 1001, 55, 1, 55,
                    2, 53, 55, 53, 4, 53, 1001, 56, -1, 56, 1005, 56, 6, 99, 0, 0, 0, 0, 10
                ],
                &[9, 7, 8, 5, 6]
            ),
            18216
        );
    }
}
