use std::num::ParseIntError;

#[cfg(test)]
const X_LEN: usize = 1000;
#[cfg(not(test))]
const X_LEN: usize = 35000;

#[cfg(test)]
const Y_LEN: usize = 1000;
#[cfg(not(test))]
const Y_LEN: usize = 35000;

pub fn get_man_dist() -> Option<u32> {
    // Part A
    let mut input = include_str!("../input/day3.txt").lines();

    let first = input.next()?;
    let second = input.next()?;

    Some(get_closest_intersection(first, second).ok()?)
}

pub fn get_shortest_dist() -> Option<u32> {
    // Part B
    let mut input = include_str!("../input/day3.txt").lines();

    let first = input.next()?;
    let second = input.next()?;

    Some(get_shortest_intersection(first, second).ok()?)
}

fn get_closest_intersection(first: &str, second: &str) -> Result<u32, ParseMovementErr> {
    use Movement::*;
    use VisitedBy::*;

    let first: Vec<Movement> = first
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<_, _>>()?;
    let second: Vec<Movement> = second
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<_, _>>()?;

    let mut grid = vec![[Neither; X_LEN]; Y_LEN];
    let central_port = ((X_LEN / 2) as u32, (Y_LEN / 2) as u32);
    let mut current = central_port;

    first.iter().for_each(|m| match m {
        Up(x) => {
            for i in 1..=*x {
                grid[(current.1 + i) as usize][current.0 as usize] = First;
            }
            current = (current.0, current.1 + x);
        }
        Down(x) => {
            for i in 1..=*x {
                grid[(current.1 - i) as usize][current.0 as usize] = First;
            }
            current = (current.0, current.1 - x);
        }
        Left(x) => {
            for i in 1..=*x {
                grid[current.1 as usize][(current.0 - i) as usize] = First;
            }
            current = (current.0 - x, current.1);
        }
        Right(x) => {
            for i in 1..=*x {
                grid[current.1 as usize][(current.0 + i) as usize] = First;
            }
            current = (current.0 + x, current.1);
        }
    });

    let mut current = central_port;

    second.iter().for_each(|m| match m {
        Up(x) => {
            for i in 1..=*x {
                if grid[(current.1 + i) as usize][current.0 as usize] == First {
                    grid[(current.1 + i) as usize][current.0 as usize] = Both
                };
            }
            current = (current.0, current.1 + x);
        }
        Down(x) => {
            for i in 1..=*x {
                if grid[(current.1 - i) as usize][current.0 as usize] == First {
                    grid[(current.1 - i) as usize][current.0 as usize] = Both
                };
            }
            current = (current.0, current.1 - x);
        }
        Left(x) => {
            for i in 1..=*x {
                if grid[current.1 as usize][(current.0 - i) as usize] == First {
                    grid[current.1 as usize][(current.0 - i) as usize] = Both
                };
            }
            current = (current.0 - x, current.1);
        }
        Right(x) => {
            for i in 1..=*x {
                if grid[current.1 as usize][(current.0 + i) as usize] == First {
                    grid[current.1 as usize][(current.0 + i) as usize] = Both
                };
            }
            current = (current.0 + x, current.1);
        }
    });

    let mut shortest_dist = std::u32::MAX;

    for (y, row) in grid.iter().enumerate().take(Y_LEN) {
        for (x, cell) in row.iter().enumerate().take(X_LEN) {
            if *cell == VisitedBy::Both && (x as u32, y as u32) != central_port {
                let dist = manhattan_distance((x as u32, y as u32), central_port);
                if dist < shortest_dist {
                    shortest_dist = dist;
                }
            }
        }
    }

    Ok(shortest_dist)
}

fn get_shortest_intersection(first: &str, second: &str) -> Result<u32, ParseMovementErr> {
    use Movement::*;
    use VisitDistance::*;

    let first = first
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;
    let second = second
        .split(',')
        .map(|x| x.trim().parse())
        .collect::<Result<Vec<_>, _>>()?;

    let mut grid = vec![[Neither; X_LEN]; Y_LEN];
    let central_port = ((X_LEN / 2) as u32, (Y_LEN / 2) as u32);
    let mut current = central_port;
    let mut travelled = 0;

    first.iter().for_each(|m| match m {
        Up(x) => {
            for i in 1..=*x {
                grid[(current.1 + i) as usize][current.0 as usize] = First(travelled + i);
            }
            current = (current.0, current.1 + x);
            travelled += x;
        }
        Down(x) => {
            for i in 1..=*x {
                grid[(current.1 - i) as usize][current.0 as usize] = First(travelled + i);
            }
            current = (current.0, current.1 - x);
            travelled += x;
        }
        Left(x) => {
            for i in 1..=*x {
                grid[current.1 as usize][(current.0 - i) as usize] = First(travelled + i);
            }
            current = (current.0 - x, current.1);
            travelled += x;
        }
        Right(x) => {
            for i in 1..=*x {
                grid[current.1 as usize][(current.0 + i) as usize] = First(travelled + i);
            }
            current = (current.0 + x, current.1);
            travelled += x;
        }
    });

    let mut current = central_port;
    let mut travelled = 0;

    second.iter().for_each(|m| match m {
        Up(x) => {
            for i in 1..=*x {
                if grid[(current.1 + i) as usize][current.0 as usize].is_first() {
                    grid[(current.1 + i) as usize][current.0 as usize] = Both(
                        travelled
                            + i
                            + grid[(current.1 + i) as usize][current.0 as usize].get_dist(),
                    )
                };
            }
            current = (current.0, current.1 + x);
            travelled += x;
        }
        Down(x) => {
            for i in 1..=*x {
                if grid[(current.1 - i) as usize][current.0 as usize].is_first() {
                    grid[(current.1 - i) as usize][current.0 as usize] = Both(
                        travelled
                            + i
                            + grid[(current.1 - i) as usize][current.0 as usize].get_dist(),
                    )
                };
            }
            current = (current.0, current.1 - x);
            travelled += x;
        }
        Left(x) => {
            for i in 1..=*x {
                if grid[current.1 as usize][(current.0 - i) as usize].is_first() {
                    grid[current.1 as usize][(current.0 - i) as usize] = Both(
                        travelled
                            + i
                            + grid[current.1 as usize][(current.0 - i) as usize].get_dist(),
                    )
                };
            }
            current = (current.0 - x, current.1);
            travelled += x;
        }
        Right(x) => {
            for i in 1..=*x {
                if grid[current.1 as usize][(current.0 + i) as usize].is_first() {
                    grid[current.1 as usize][(current.0 + i) as usize] = Both(
                        travelled
                            + i
                            + grid[current.1 as usize][(current.0 + i) as usize].get_dist(),
                    )
                };
            }
            current = (current.0 + x, current.1);
            travelled += x;
        }
    });

    let mut shortest_dist = std::u32::MAX;

    for (y, row) in grid.iter().enumerate().take(Y_LEN) {
        for (x, cell) in row.iter().enumerate().take(X_LEN) {
            if cell.is_both() && (x as u32, y as u32) != central_port {
                let dist = cell.get_dist();
                if dist < shortest_dist {
                    shortest_dist = dist;
                }
            }
        }
    }

    Ok(shortest_dist)
}

fn manhattan_distance(first: (u32, u32), second: (u32, u32)) -> u32 {
    abs_dist(first.0, second.0) + abs_dist(first.1, second.1)
}

fn abs_dist(first: u32, second: u32) -> u32 {
    if first > second {
        first - second
    } else {
        second - first
    }
}

#[derive(Debug)]
enum Movement {
    Up(u32),
    Down(u32),
    Left(u32),
    Right(u32),
}

#[derive(Debug, PartialEq)]
struct ParseMovementErr {}

impl std::str::FromStr for Movement {
    type Err = ParseMovementErr;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        use Movement::*;

        match s.chars().next().ok_or(ParseMovementErr {})? {
            'U' => Ok(Up(s[1..].parse().map_err(|_err| Self::Err {})?)),
            'D' => Ok(Down(s[1..].parse().map_err(|_err| Self::Err {})?)),
            'L' => Ok(Left(s[1..].parse().map_err(|_err| Self::Err {})?)),
            'R' => Ok(Right(s[1..].parse().map_err(|_err| Self::Err {})?)),
            _ => Err(Self::Err {}),
        }
    }
}

#[derive(Clone, Copy, PartialEq)]
enum VisitedBy {
    Neither,
    First,
    Both,
}

#[derive(Clone, Copy, PartialEq)]
enum VisitDistance {
    Neither,
    First(u32),
    Both(u32),
}

impl VisitDistance {
    fn is_both(self) -> bool {
        match self {
            VisitDistance::Both(_) => true,
            _ => false,
        }
    }

    fn is_first(self) -> bool {
        match self {
            VisitDistance::First(_) => true,
            _ => false,
        }
    }

    fn get_dist(self) -> u32 {
        match self {
            VisitDistance::Neither => 0,
            VisitDistance::First(x) => x,
            VisitDistance::Both(x) => x,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_closest() {
        assert_eq!(
            get_closest_intersection(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            Ok(159)
        );
        assert_eq!(
            get_closest_intersection(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Ok(135)
        );
    }

    #[test]
    fn test_shortest() {
        assert_eq!(
            get_shortest_intersection(
                "R75,D30,R83,U83,L12,D49,R71,U7,L72",
                "U62,R66,U55,R34,D71,R55,D58,R83"
            ),
            Ok(610)
        );
        assert_eq!(
            get_shortest_intersection(
                "R98,U47,R26,D63,R33,U87,L62,D20,R33,U53,R51",
                "U98,R91,D20,R16,D67,R40,U7,R15,U6,R7"
            ),
            Ok(410)
        );
    }
}
