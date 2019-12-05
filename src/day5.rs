pub fn diag_air_conditioner_unit() -> Vec<i32> {
    // Part A
    let mut input = include_str!("../input/day5.txt")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<_>>();
    execute_intcode(&mut input, &[1])
}

pub fn diag_thermal_radiator_controller() -> Vec<i32> {
    // Part B
    let mut input = include_str!("../input/day5.txt")
        .split(',')
        .map(|x| x.trim().parse().unwrap())
        .collect::<Vec<_>>();
    execute_intcode(&mut input, &[5])
}

fn execute_intcode(v: &mut [i32], input: &[i32]) -> Vec<i32> {
    let mut input = input.iter();
    let mut counter = 0;
    let mut output = Vec::new();

    loop {
        match v[counter] % 100 {
            1 => {
                let modes = get_mode(v[counter] / 100, 3);

                let param1 = if modes[0] == ParameterMode::Position {
                    v[v[counter + 1] as usize]
                } else {
                    v[counter + 1]
                };
                let param2 = if modes[1] == ParameterMode::Position {
                    v[v[counter + 2] as usize]
                } else {
                    v[counter + 2]
                };

                v[v[counter + 3] as usize] = param1 + param2;
                counter += 4;
            }
            2 => {
                let modes = get_mode(v[counter] / 100, 3);

                let param1 = if modes[0] == ParameterMode::Position {
                    v[v[counter + 1] as usize]
                } else {
                    v[counter + 1]
                };
                let param2 = if modes[1] == ParameterMode::Position {
                    v[v[counter + 2] as usize]
                } else {
                    v[counter + 2]
                };

                v[v[counter + 3] as usize] = param1 * param2;
                counter += 4;
            }
            3 => {
                let modes = get_mode(v[counter] / 100, 1); // not needed
                v[v[counter + 1] as usize] = *input.next().unwrap();
                counter += 2;
            }
            4 => {
                let modes = get_mode(v[counter] / 100, 1);
                let param1 = if modes[0] == ParameterMode::Position {
                    v[v[counter + 1] as usize]
                } else {
                    v[counter + 1]
                };

                // println!("{}", param1);
                output.push(param1);
                counter += 2;
            }
            5 => {
                let modes = get_mode(v[counter] / 100, 2);
                let param1 = if modes[0] == ParameterMode::Position {
                    v[v[counter + 1] as usize]
                } else {
                    v[counter + 1]
                };
                let param2 = if modes[1] == ParameterMode::Position {
                    v[v[counter + 2] as usize]
                } else {
                    v[counter + 2]
                };

                if param1 != 0 {
                    counter = param2 as usize;
                } else {
                    counter += 3;
                }
            }
            6 => {
                let modes = get_mode(v[counter] / 100, 2);
                let param1 = if modes[0] == ParameterMode::Position {
                    v[v[counter + 1] as usize]
                } else {
                    v[counter + 1]
                };
                let param2 = if modes[1] == ParameterMode::Position {
                    v[v[counter + 2] as usize]
                } else {
                    v[counter + 2]
                };

                if param1 == 0 {
                    counter = param2 as usize;
                } else {
                    counter += 3;
                }
            }
            7 => {
                let modes = get_mode(v[counter] / 100, 3);
                let param1 = if modes[0] == ParameterMode::Position {
                    v[v[counter + 1] as usize]
                } else {
                    v[counter + 1]
                };
                let param2 = if modes[1] == ParameterMode::Position {
                    v[v[counter + 2] as usize]
                } else {
                    v[counter + 2]
                };
                let param3 = v[counter + 3];

                if param1 < param2 {
                    v[param3 as usize] = 1;
                } else {
                    v[param3 as usize] = 0;
                }

                counter += 4;
            }
            8 => {
                let modes = get_mode(v[counter] / 100, 3);
                let param1 = if modes[0] == ParameterMode::Position {
                    v[v[counter + 1] as usize]
                } else {
                    v[counter + 1]
                };
                let param2 = if modes[1] == ParameterMode::Position {
                    v[v[counter + 2] as usize]
                } else {
                    v[counter + 2]
                };
                let param3 = v[counter + 3];

                if param1 == param2 {
                    v[param3 as usize] = 1;
                } else {
                    v[param3 as usize] = 0;
                }

                counter += 4;
            }
            99 => return output,
            _ => panic!("unreachable"),
        }
    }
}

fn get_mode(n: i32, nargs: i32) -> Vec<ParameterMode> {
    let mut v = Vec::new();
    let mut n = n;

    for i in 0..nargs {
        v.push(match n % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            _ => unreachable!(),
        });
        n /= 10;
    }
    v
}

#[derive(Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_get_mode() {
        use ParameterMode::*;

        assert_eq!(get_mode(1002 / 100, 3), vec![Position, Immediate, Position]);
    }
}
