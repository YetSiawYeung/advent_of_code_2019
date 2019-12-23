use std::{cmp::Ordering, collections::HashSet, num::ParseIntError, str::FromStr};

pub fn simulate_moons() -> Result<i32, ParseIntError> {
    // Part A
    let mut moons: MoonSystem = include_str!("../input/day12.txt").parse()?;

    moons.steps(1000);
    Ok(moons.sum_energy())
}

pub fn cycle_universe() -> Result<usize, ParseIntError> {
    // Part B
    let mut moons: MoonSystem = include_str!("../input/day12.txt").parse()?;

    Ok(moons.ticks_until_repeat())
}

#[derive(Clone, Debug)]
struct Moon {
    position: (i32, i32, i32),
    velocity: (i32, i32, i32),
}
impl Moon {
    fn total_energy(&self) -> i32 {
        self.potential_energy() * self.kinetic_energy()
    }
    fn potential_energy(&self) -> i32 {
        self.position.0.abs() + self.position.1.abs() + self.position.2.abs()
    }
    fn kinetic_energy(&self) -> i32 {
        self.velocity.0.abs() + self.velocity.1.abs() + self.velocity.2.abs()
    }
    fn x_axis(&self) -> (i32, i32) {
        (self.position.0, self.velocity.0)
    }
    fn y_axis(&self) -> (i32, i32) {
        (self.position.1, self.velocity.1)
    }
    fn z_axis(&self) -> (i32, i32) {
        (self.position.2, self.velocity.2)
    }
}
impl FromStr for Moon {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let input = s.split(',').collect::<Vec<_>>();
        let x = input[0].split('=').nth(1).unwrap().parse()?;
        let y = input[1].split('=').nth(1).unwrap().parse()?;
        let z = input[2]
            .split('=')
            .nth(1)
            .unwrap()
            .split('>')
            .next()
            .unwrap()
            .parse()?;

        Ok(Moon {
            position: (x, y, z),
            velocity: (0, 0, 0),
        })
    }
}

#[derive(Clone, Debug)]
struct MoonSystem {
    moons: Vec<Moon>,
}
impl MoonSystem {
    fn sum_energy(&self) -> i32 {
        self.moons.iter().map(Moon::total_energy).sum()
    }
    fn tick(&mut self) {
        let mut changes = vec![(0, 0, 0); self.moons.len()];

        for i in 0..self.moons.len() {
            for j in i..self.moons.len() {
                match self.moons[i].position.0.cmp(&self.moons[j].position.0) {
                    Ordering::Greater => {
                        changes[i].0 -= 1;
                        changes[j].0 += 1;
                    }
                    Ordering::Less => {
                        changes[i].0 += 1;
                        changes[j].0 -= 1
                    }
                    Ordering::Equal => {}
                };
                match self.moons[i].position.1.cmp(&self.moons[j].position.1) {
                    Ordering::Greater => {
                        changes[i].1 -= 1;
                        changes[j].1 += 1;
                    }
                    Ordering::Less => {
                        changes[i].1 += 1;
                        changes[j].1 -= 1
                    }
                    Ordering::Equal => {}
                };
                match self.moons[i].position.2.cmp(&self.moons[j].position.2) {
                    Ordering::Greater => {
                        changes[i].2 -= 1;
                        changes[j].2 += 1;
                    }
                    Ordering::Less => {
                        changes[i].2 += 1;
                        changes[j].2 -= 1
                    }
                    Ordering::Equal => {}
                };
            }
        }
        self.moons
            .iter_mut()
            .zip(changes.iter())
            .for_each(|(moon, change)| {
                moon.velocity.0 += change.0;
                moon.velocity.1 += change.1;
                moon.velocity.2 += change.2;
                moon.position.0 += moon.velocity.0;
                moon.position.1 += moon.velocity.1;
                moon.position.2 += moon.velocity.2;
            });
    }
    fn steps(&mut self, n: usize) {
        for _ in 0..n {
            self.tick();
        }
    }
    fn ticks_until_repeat(&mut self) -> usize {
        let (mut count_x, mut count_y, mut count_z) = (0, 0, 0);
        let mut hs = HashSet::new();

        while !hs.contains(&(self.moons.iter().map(Moon::x_axis).collect::<Vec<_>>())) {
            hs.insert(self.moons.iter().map(Moon::x_axis).collect::<Vec<_>>());
            self.tick();
            count_x += 1;
        }

        hs.clear();
        while !hs.contains(&(self.moons.iter().map(Moon::y_axis).collect::<Vec<_>>())) {
            hs.insert(self.moons.iter().map(Moon::y_axis).collect::<Vec<_>>());
            self.tick();
            count_y += 1;
        }

        hs.clear();
        while !hs.contains(&(self.moons.iter().map(Moon::z_axis).collect::<Vec<_>>())) {
            hs.insert(self.moons.iter().map(Moon::z_axis).collect::<Vec<_>>());
            self.tick();
            count_z += 1;
        }

        lcm(lcm(count_x, count_y), count_z)
    }
}
impl FromStr for MoonSystem {
    type Err = ParseIntError;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let moons = s.lines().map(str::parse).collect::<Result<Vec<_>, _>>()?;
        Ok(Self { moons })
    }
}

fn lcm(a: usize, b: usize) -> usize {
    let gcd = gcd(a, b);
    (a / gcd) * (b / gcd) * gcd
}

fn gcd(x: usize, y: usize) -> usize {
    let mut x = x;
    let mut y = y;
    let mut t = 0;
    while y != 0 {
        t = y;
        y = x % y;
        x = t;
    }
    x
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_planets_moving() -> Result<(), ParseIntError> {
        let mut moons: MoonSystem = "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>"
            .parse()?;
        moons.steps(10);
        assert_eq!(moons.sum_energy(), 179);

        let mut moons: MoonSystem = "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>"
            .parse()?;
        moons.steps(100);
        assert_eq!(moons.sum_energy(), 1940);

        Ok(())
    }
    #[test]
    fn test_planet_repeat() -> Result<(), ParseIntError> {
        let mut moons: MoonSystem = "<x=-1, y=0, z=2>
        <x=2, y=-10, z=-7>
        <x=4, y=-8, z=8>
        <x=3, y=5, z=-1>"
            .parse()?;
        assert_eq!(moons.ticks_until_repeat(), 2772);

        let mut moons: MoonSystem = "<x=-8, y=-10, z=0>
        <x=5, y=5, z=10>
        <x=2, y=-7, z=3>
        <x=9, y=-8, z=-3>"
            .parse()?;
        assert_eq!(moons.ticks_until_repeat(), 4_686_774_924);
        Ok(())
    }
}
