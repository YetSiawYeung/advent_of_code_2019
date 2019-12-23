use std::convert::TryFrom;

#[derive(Debug)]
pub struct IntcodeMachine {
    ram: Vec<i64>,
    pointer: usize,
    state: MachineState,
    relative_base: i64,
}
impl IntcodeMachine {
    pub fn new(state: &[i64]) -> Self {
        let mut state = state.to_vec();
        state.resize(10 * state.len(), 0);
        Self {
            ram: state,
            pointer: 0,
            state: MachineState::Running,
            relative_base: 0,
        }
    }

    pub fn create_and_execute(state: &[i64], input: Option<&[i64]>) -> Option<Vec<i64>> {
        Self::new(state).execute(input)
    }

    pub fn execute(&mut self, input: Option<&[i64]>) -> Option<Vec<i64>> {
        use Operation::*;

        if self.state == MachineState::Stopped {
            return input.map(Vec::from);
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
                    let param3 = self.resolve_set(third, 3);

                    self.ram[param3] = param1 + param2;
                }
                Multiply(first, second, third) => {
                    let param1 = self.resolve_get(first, 1);
                    let param2 = self.resolve_get(second, 2);
                    let param3 = self.resolve_set(third, 3);

                    self.ram[param3] = param1 * param2;
                }
                Input(first) => {
                    let param1 = self.resolve_set(first, 1);

                    if let Some(i) = input.next() {
                        self.ram[param1] = *i;
                    } else {
                        self.state = MachineState::Blocked;
                        return Some(output);
                    }
                }
                Output(first) => {
                    let param1 = self.resolve_set(first, 1);

                    output.push(self.ram[param1]);
                }
                JumpIfTrue(first, second) => {
                    let param1 = self.resolve_get(first, 1);
                    let param2 = self.resolve_get(second, 2);

                    if param1 != 0 {
                        self.pointer = param2 as usize;
                        increment_pointer = false;
                    }
                }
                JumpIfFalse(first, second) => {
                    let param1 = self.resolve_get(first, 1);
                    let param2 = self.resolve_get(second, 2);

                    if param1 == 0 {
                        self.pointer = param2 as usize;
                        increment_pointer = false;
                    }
                }
                LessThan(first, second, third) => {
                    let param1 = self.resolve_get(first, 1);
                    let param2 = self.resolve_get(second, 2);
                    let param3 = self.resolve_set(third, 3);

                    self.ram[param3] = if param1 < param2 { 1 } else { 0 };
                }
                Equals(first, second, third) => {
                    let param1 = self.resolve_get(first, 1);
                    let param2 = self.resolve_get(second, 2);
                    let param3 = self.resolve_set(third, 3);

                    self.ram[param3] = if param1 == param2 { 1 } else { 0 };
                }
                AdjustRelativeBase(first) => {
                    self.relative_base += self.resolve_get(first, 1);
                }
                Terminate => {
                    self.state = MachineState::Stopped;
                    return Some(output);
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

    fn resolve_get(&self, mode: ParameterMode, count: usize) -> i64 {
        use ParameterMode::*;

        match mode {
            Position => self.ram[self.ram[self.pointer + count] as usize],
            Immediate => self.ram[self.pointer + count],
            Relative => self.ram[(self.relative_base + self.ram[self.pointer + count]) as usize],
        }
    }
    fn resolve_set(&self, mode: ParameterMode, count: usize) -> usize {
        use ParameterMode::*;
        match mode {
            Position => self.ram[self.pointer + count] as usize,
            Immediate => self.pointer + count,
            Relative => {
                usize::try_from(self.relative_base + self.ram[self.pointer + count]).unwrap()
            }
        }
    }
    pub fn stopped(&self) -> bool {
        self.state == MachineState::Stopped
    }
    pub fn set_memory(&mut self, address: usize, value: i64) {
        self.ram[address] = value;
    }
    pub fn state(&self) -> MachineState {
        self.state
    }
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum MachineState {
    Running,
    Blocked,
    Stopped,
}

#[derive(Clone, Copy, Debug, PartialEq)]
enum ParameterMode {
    Position,
    Immediate,
    Relative,
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
    AdjustRelativeBase(ParameterMode),
    Terminate,
}
impl Operation {
    fn from(n: i64) -> Self {
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
            9 => {
                let modes = get_mode(n / 100, 1);
                AdjustRelativeBase(modes[0])
            }
            99 => Terminate,
            _ => unimplemented!(),
        }
    }
    fn num_params(&self) -> usize {
        use Operation::*;
        match &self {
            Terminate => 0,
            Input(_) | Output(_) | AdjustRelativeBase(_) => 1,
            JumpIfTrue(_, _) | JumpIfFalse(_, _) => 2,
            Add(_, _, _) | Multiply(_, _, _) | LessThan(_, _, _) | Equals(_, _, _) => 3,
        }
    }
}

fn get_mode(n: i64, nargs: i64) -> Vec<ParameterMode> {
    let mut v = Vec::new();
    let mut n = n;

    for i in 0..nargs {
        v.push(match n % 10 {
            0 => ParameterMode::Position,
            1 => ParameterMode::Immediate,
            2 => ParameterMode::Relative,
            _ => unreachable!(),
        });
        n /= 10;
    }
    v
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_intcode() {
        // from day 2
        let mut machine = IntcodeMachine::new(&[1, 0, 0, 0, 99]);
        machine.execute(None);
        assert_eq!(machine.ram[..5], [2, 0, 0, 0, 99]);

        let mut machine = IntcodeMachine::new(&[2, 3, 0, 3, 99]);
        machine.execute(None);
        assert_eq!(machine.ram[..5], [2, 3, 0, 6, 99]);

        let mut machine = IntcodeMachine::new(&[2, 4, 4, 5, 99, 0]);
        machine.execute(None);
        assert_eq!(machine.ram[..6], [2, 4, 4, 5, 99, 9801]);

        let mut machine = IntcodeMachine::new(&[1, 1, 1, 4, 99, 5, 6, 0, 99]);
        machine.execute(None);
        assert_eq!(machine.ram[..9], [30, 1, 1, 4, 2, 5, 6, 0, 99]);
    }

    #[test]
    fn test_get_mode() {
        // from day 5
        use ParameterMode::*;

        assert_eq!(get_mode(1002 / 100, 3), vec![Position, Immediate, Position]);
    }
}
