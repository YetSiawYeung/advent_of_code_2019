use std::{
    cmp::Ordering,
    collections::{HashMap, HashSet},
    convert::TryFrom,
    error::Error,
    fmt::Display,
};

pub fn find_best_spot() -> Result<usize, ParseAsteroidFieldError> {
    // Part A
    let mut input = include_str!("../input/day10.txt");

    Ok(best_spot(input)?.1)
}

pub fn vaporize_200th_asteroid() -> Result<usize, ParseAsteroidFieldError> {
    // Part B
    let mut input = include_str!("../input/day10.txt");

    let ans = vaporize_n_asteroids(input, 200)?;
    Ok(ans.0 * 100 + ans.1)
}

fn best_spot(s: &str) -> Result<((usize, usize), usize), ParseAsteroidFieldError> {
    let field = AsteroidField::try_from(s)?;
    Ok(field.find_best())
}

fn vaporize_n_asteroids(s: &str, n: usize) -> Result<(usize, usize), ParseAsteroidFieldError> {
    let mut field = AsteroidField::try_from(s)?;
    Ok(field.vaporize(n, field.find_best().0))
}

#[derive(Debug)]
struct AsteroidField {
    field: Vec<Vec<Location>>,
}
impl AsteroidField {
    fn new(field: Vec<Vec<Location>>) -> Self {
        Self { field }
    }
    fn asteroids(&self) -> Vec<(usize, usize)> {
        self.field
            .iter()
            .enumerate()
            .flat_map(|(y, y_array)| {
                y_array.iter().enumerate().filter_map(move |(x, loc)| {
                    if *loc == Location::Asteroid {
                        Some((x, y))
                    } else {
                        None
                    }
                })
            })
            .collect::<Vec<_>>()
    }
    fn find_best(&self) -> ((usize, usize), usize) {
        let asteroids = self.asteroids();
        asteroids
            .iter()
            .map(|&a| {
                (
                    a,
                    asteroids
                        .iter()
                        .filter_map(|&aa| {
                            let dist = reduce_factor(distance(
                                (a.0 as isize, a.1 as isize),
                                (aa.0 as isize, aa.1 as isize),
                            ));
                            if dist != (0, 0) {
                                Some(dist)
                            } else {
                                None
                            }
                        })
                        .collect::<HashSet<_>>()
                        .len(),
                )
            })
            .max_by_key(|x| x.1)
            .unwrap()
    }
    fn vaporize(&mut self, n: usize, from: (usize, usize)) -> (usize, usize) {
        let asteroids = self.asteroids();
        assert!(asteroids.contains(&from));

        let mut map: HashMap<Angle, Vec<(usize, usize)>> = HashMap::new();
        asteroids.iter().for_each(|&(x, y)| {
            let dist = distance((from.0 as isize, from.1 as isize), (x as isize, y as isize));
            if dist != (0, 0) {
                let v = map
                    .entry(Angle::new(-dist.0, dist.1))
                    .or_insert_with(Vec::new);
                v.push((x, y));
            }
        });
        map.values_mut().for_each(|vec| {
            vec.sort_by_key(|&(x, y)| {
                (from.0 as isize - x as isize).abs() + (from.1 as isize - y as isize).abs()
            })
        });

        let mut order = map.keys().copied().collect::<Vec<_>>();
        let mut count = 0;
        order.sort();
        loop {
            for i in &order {
                if !map[i].is_empty() {
                    let asteroid = map.entry(*i).or_default().remove(0);
                    count += 1;
                    if count == n {
                        return asteroid;
                    }
                }
            }
        }
    }
}

#[derive(Debug)]
pub struct ParseAsteroidFieldError {}
impl Display for ParseAsteroidFieldError {
    fn fmt(self: &Self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
        write!(f, "Error while parsing asteroid field")?;
        Ok(())
    }
}
impl Error for ParseAsteroidFieldError {}

impl TryFrom<&str> for AsteroidField {
    type Error = ParseAsteroidFieldError;
    fn try_from(value: &str) -> Result<Self, Self::Error> {
        Ok(AsteroidField::new(
            value
                .lines()
                .map(|x_arr| {
                    x_arr
                        .trim()
                        .bytes()
                        .map(|c| match c {
                            b'#' => Ok(Location::Asteroid),
                            b'.' => Ok(Location::Empty),
                            _ => Err(Self::Error {}),
                        })
                        .collect::<Result<_, _>>()
                })
                .collect::<Result<_, _>>()?,
        ))
    }
}

#[derive(Debug, PartialEq)]
enum Location {
    Asteroid,
    Empty,
}

fn reduce_factor(tuple: (isize, isize)) -> (isize, isize) {
    let (x, y) = tuple;
    let gcd = gcd(x, y).abs();

    if gcd != 0 {
        (x / gcd, y / gcd)
    } else {
        (x, y)
    }
}

fn gcd(x: isize, y: isize) -> isize {
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

fn distance(from: (isize, isize), to: (isize, isize)) -> (isize, isize) {
    (from.0 - to.0, (from.1 - to.1))
}

#[derive(Debug, PartialEq, Eq, Hash, Clone, Copy)]
struct Angle(isize, isize);
impl Angle {
    fn new(x: isize, y: isize) -> Self {
        let (x, y) = reduce_factor((x, y));
        Self(x, y)
    }
    fn new_unreduced(x: isize, y: isize) -> Self {
        Self(x, y)
    }
    fn angle(&self) -> f64 {
        match self {
            Self(0, 0) => panic!("invalid"),
            Self(0, y) => {
                if *y > 0 {
                    180.0
                } else {
                    0.0
                }
            }
            Self(x, 0) => {
                if *x > 0 {
                    90.0
                } else {
                    270.0
                }
            }
            Self(x, y) => {
                let quadrant = self.quadrant();
                let constant = quadrant.angle();
                let angle = match quadrant {
                    Quadrant::I | Quadrant::III => {
                        ((x.abs() as f64) / (y.abs() as f64)).atan().to_degrees()
                    }
                    Quadrant::II | Quadrant::IV => {
                        ((y.abs() as f64) / (x.abs() as f64)).atan().to_degrees()
                    }
                };
                constant + angle
            }
        }
    }
    fn quadrant(&self) -> Quadrant {
        match (self.0 > 0, self.1 > 0) {
            (true, true) => Quadrant::I,
            (true, false) => Quadrant::II,
            (false, true) => Quadrant::IV,
            (false, false) => Quadrant::III,
        }
    }
}
impl PartialOrd for Angle {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        if self.angle() < other.angle() {
            Some(Ordering::Less)
        } else if self.angle() > other.angle() {
            Some(Ordering::Greater)
        } else {
            Some(Ordering::Equal)
        }
    }
}
impl Ord for Angle {
    fn cmp(&self, other: &Self) -> Ordering {
        self.partial_cmp(other).unwrap()
    }
}

///
///        |       
///   IV   |   I   
///        |       
/// -------+-------
///        |       
///   III  |   II  
///        |       
///
#[derive(Debug, PartialEq)]
enum Quadrant {
    I,
    II,
    III,
    IV,
}
impl Quadrant {
    fn angle(&self) -> f64 {
        use Quadrant::*;

        match &self {
            I => 0.0,
            II => 90.0,
            III => 180.0,
            IV => 270.0,
        }
    }
}

#[cfg(test)]
mod test {
    use super::*;
    use std::f64::consts::PI;

    #[test]
    fn test_gcd() {
        assert_eq!(gcd(-2, 2), 2);
    }

    #[test]
    fn test_reduce_factor() {
        assert_eq!(reduce_factor((-2, 2)), (-1, 1));
    }

    #[test]
    fn test_best_monitoring_spot() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            best_spot(
                ".#..#
                .....
                #####
                ....#
                ...##"
            )?,
            ((3, 4), 8)
        );
        assert_eq!(
            best_spot(
                "......#.#.
                #..#.#....
                ..#######.
                .#.#.###..
                .#..#.....
                ..#....#.#
                #..#....#.
                .##.#..###
                ##...#..#.
                .#....####"
            )?,
            ((5, 8), 33)
        );
        assert_eq!(
            best_spot(
                "#.#...#.#.
                .###....#.
                .#....#...
                ##.#.#.#.#
                ....#.#.#.
                .##..###.#
                ..#...##..
                ..##....##
                ......#...
                .####.###."
            )?,
            ((1, 2), 35)
        );
        assert_eq!(
            best_spot(
                ".#..#..###
                ####.###.#
                ....###.#.
                ..###.##.#
                ##.##.#.#.
                ....###..#
                ..#.#..#.#
                #..#.#.###
                .##...##.#
                .....#.#.."
            )?,
            ((6, 3), 41)
        );
        assert_eq!(
            best_spot(
                ".#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##"
            )?,
            ((11, 13), 210)
        );
        Ok(())
    }

    #[test]
    fn test_vaporize() -> Result<(), Box<dyn std::error::Error>> {
        assert_eq!(
            vaporize_n_asteroids(
                ".#..##.###...#######
                ##.############..##.
                .#.######.########.#
                .###.#######.####.#.
                #####.##.#.##.###.##
                ..#####..#.#########
                ####################
                #.####....###.#.#.##
                ##.#################
                #####.##.###..####..
                ..######..##.#######
                ####.##.####...##..#
                .#####..#.######.###
                ##...#.##########...
                #.##########.#######
                .####.#.###.###.#.##
                ....##.##.###..#####
                .#.#.###########.###
                #.#.#.#####.####.###
                ###.##.####.##.#..##",
                200
            )?,
            (8, 2)
        );
        Ok(())
    }

    #[test]
    fn test_quadrant() {
        assert_eq!(Angle(5, 2).quadrant(), Quadrant::I);
        assert_eq!(Angle(5, -2).quadrant(), Quadrant::II);
        assert_eq!(Angle(-5, -2).quadrant(), Quadrant::III);
        assert_eq!(Angle(-5, 2).quadrant(), Quadrant::IV);
    }
}
