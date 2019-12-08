use std::num::ParseIntError;

pub fn max_signal() -> Result<i32, ParseIntError> {
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
pub fn max_signal_feedback() -> Result<i32, ParseIntError> {
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

fn calculate_signal(machine: &mut [i32], input: &[i32]) -> i32 {
    input
        .iter()
        .fold(0, |acc, i| execute_intcode(machine, &[*i, acc])[0])
}

fn signal_feedback(machine: &[i32], input: &[i32]) -> i32 {
    let mut machines = input
        .iter()
        .map(|i| {
            let mut mac = Machine::new(machine);
            mac.execute(Some(&[*i]));
            mac
        })
        .collect::<Vec<_>>();
    let mut acc = 0;
    while machines
        .iter()
        .any(|machine| machine.state != MachineState::Stopped)
    {
        for (mac, i) in machines.iter_mut().zip(input.iter()) {
            acc = mac.execute(Some(&[acc]))[0];
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

fn generate(k: usize, a: &mut Vec<i32>) -> Vec<Vec<i32>> {
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
fn permutations(list: &mut Vec<i32>) -> Vec<Vec<i32>> {
    generate(list.len(), list)
}
fn execute_intcode(v: &mut [i32], input: &[i32]) -> Vec<i32> {
    let mut machine = Machine::new(v);
    machine.execute(Some(input))
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

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
}

#[derive(Debug)]
enum Operation {
    Add(ParameterMode, ParameterMode, ParameterMode),
    Multiply(ParameterMode, ParameterMode, ParameterMode),
    Input(ParameterMode),
    Output(ParameterMode),
    JumpIfTrue(ParameterMode, ParameterMode),
    JumpIfFalse(ParameterMode, ParameterMode),
    LessThan(ParameterMode, ParameterMode, ParameterMode),
    Equals(ParameterMode, ParameterMode, ParameterMode),
    Terminate,
}

impl Operation {
    fn from(n: i32) -> Self {
        use Operation::*;

        match n % 100 {
            1 => {
                let modes = get_mode(n / 100, 3);
                Add(modes[0], modes[1], modes[2])
            }
            2 => {
                let modes = get_mode(n / 100, 3);
                Multiply(modes[0], modes[1], modes[2])
            }
            3 => {
                let modes = get_mode(n / 100, 1);
                Input(modes[0])
            }
            4 => {
                let modes = get_mode(n / 100, 1);
                Output(modes[0])
            }
            5 => {
                let modes = get_mode(n / 100, 2);
                JumpIfTrue(modes[0], modes[1])
            }
            6 => {
                let modes = get_mode(n / 100, 2);
                JumpIfFalse(modes[0], modes[1])
            }
            7 => {
                let modes = get_mode(n / 100, 3);
                LessThan(modes[0], modes[1], modes[2])
            }
            8 => {
                let modes = get_mode(n / 100, 3);
                Equals(modes[0], modes[1], modes[2])
            }
            99 => Terminate,
            _ => unimplemented!(),
        }
    }
    fn num_params(&self) -> usize {
        use Operation::*;
        match &self {
            Terminate => 0,
            Input(_) | Output(_) => 1,
            JumpIfTrue(_, _) | JumpIfFalse(_, _) => 2,
            Add(_, _, _) | Multiply(_, _, _) | LessThan(_, _, _) | Equals(_, _, _) => 3,
        }
    }
}
#[derive(Debug, PartialEq)]
enum MachineState {
    Running,
    Blocked,
    Stopped,
}
#[derive(Debug)]
struct Machine {
    ram: Vec<i32>,
    pointer: usize,
    state: MachineState,
}
impl Machine {
    fn new(state: &[i32]) -> Self {
        Self {
            ram: state.to_vec(),
            pointer: 0,
            state: MachineState::Running,
        }
    }
    fn execute(&mut self, input: Option<&[i32]>) -> Vec<i32> {
        use Operation::*;

        if self.state == MachineState::Stopped {
            return input.map(Vec::from).unwrap_or_default();
        }

        let mut input = input.unwrap_or_default().iter();
    
        let mut output = Vec::new();
        let mut increment_pointer = true;

        loop {
            let op = Operation::from(self.ram[self.pointer]);
            match op {
                Add(first, second, third) => {
                    let param1 = self.resolve_get(first, 1);
                    let param2 = self.resolve_get(second, 2);
                    let param3 = match third {
                        ParameterMode::Position => self.ram[self.pointer + 3] as usize,
                        ParameterMode::Immediate => self.pointer + 3, // unreachable?
                    };

                    self.ram[param3] = param1 + param2;
                }
                Multiply(first, second, third) => {
                    let param1 = match first {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 1] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 1],
                    };
                    let param2 = match second {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 2] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 2],
                    };
                    let param3 = match third {
                        ParameterMode::Position => self.ram[self.pointer + 3] as usize,
                        ParameterMode::Immediate => self.pointer + 3, // unreachable?
                    };

                    self.ram[param3] = param1 * param2;
                }
                Input(first) => {
                    let param1 = match first {
                        ParameterMode::Position => self.ram[self.pointer + 1] as usize,
                        ParameterMode::Immediate => self.pointer + 1,
                    };
                    if let Some(i) = input.next() {
                        self.ram[param1] = *i;
                    } else {
                        self.state = MachineState::Blocked;
                        return output;
                    }
                }
                Output(first) => {
                    let param1 = match first {
                        ParameterMode::Position => self.ram[self.pointer + 1] as usize,
                        ParameterMode::Immediate => self.pointer + 1,
                    };
                    output.push(self.ram[param1]);
                }
                JumpIfTrue(first, second) => {
                    let param1 = match first {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 1] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 1],
                    };
                    let param2 = match second {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 2] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 2],
                    };
                    if param1 != 0 {
                        self.pointer = param2 as usize;
                        increment_pointer = false;
                    }
                }
                JumpIfFalse(first, second) => {
                    let param1 = match first {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 1] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 1],
                    };
                    let param2 = match second {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 2] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 2],
                    };
                    if param1 == 0 {
                        self.pointer = param2 as usize;
                        increment_pointer = false;
                    }
                }
                LessThan(first, second, third) => {
                    let param1 = match first {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 1] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 1],
                    };
                    let param2 = match second {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 2] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 2],
                    };
                    let param3 = match third {
                        ParameterMode::Position => self.ram[self.pointer + 3] as usize,
                        ParameterMode::Immediate => self.pointer + 3, // unreachable?
                    };

                    self.ram[param3] = if param1 < param2 { 1 } else { 0 };
                }
                Equals(first, second, third) => {
                    let param1 = match first {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 1] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 1],
                    };
                    let param2 = match second {
                        ParameterMode::Position => self.ram[self.ram[self.pointer + 2] as usize],
                        ParameterMode::Immediate => self.ram[self.pointer + 2],
                    };
                    let param3 = match third {
                        ParameterMode::Position => self.ram[self.pointer + 3] as usize,
                        ParameterMode::Immediate => self.pointer + 3, // unreachable?
                    };

                    self.ram[param3] = if param1 == param2 { 1 } else { 0 };
                }
                Terminate => {
                    self.state = MachineState::Stopped;
                    return output;
                }
            }
            if increment_pointer {
                self.increment_pointer(&op);
            }
            increment_pointer = true;
        }
    }

    fn increment_pointer(&mut self, op: &Operation) {
        self.pointer += op.num_params() + 1;
    }
    fn resolve_get(&self, mode: ParameterMode, count: usize) -> i32 {
        use ParameterMode::*;

        match mode {
            Position => self.ram[self.ram[self.pointer + count] as usize],
            Immediate => self.ram[self.pointer + count],
        }
    }
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
