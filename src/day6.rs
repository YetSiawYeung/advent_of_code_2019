use std::collections::HashMap;

pub fn count_orbits() -> usize {
    // Part A
    let input = include_str!("../input/day6.txt");
    total_orbits(input)
}
pub fn find_santa() -> usize {
    let input = include_str!("../input/day6.txt");
    santa(input)
}
fn total_orbits(map: &str) -> usize {
    let x = map
        .lines()
        .map(|x| x.split(')').map(str::trim).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut hm = HashMap::new();
    hm.insert("COM", 0);

    while hm.len() < x.len() + 1 {
        for xx in x.iter() {
            if hm.contains_key(xx[0]) && !hm.contains_key(xx[1]) {
                hm.insert(xx[1], 1 + hm[xx[0]]);
            }
        }
    }
    hm.values().sum()
}

fn santa(map: &str) -> usize {
    let x = map
        .lines()
        .map(|x| x.split(')').map(str::trim).collect::<Vec<_>>())
        .collect::<Vec<_>>();
    let mut hm = HashMap::new();
    hm.insert("YOU", 0);
    while !hm.contains_key("SAN") {
        for xx in x.iter() {
            if hm.contains_key(xx[0]) && !hm.contains_key(xx[1]) {
                hm.insert(xx[1], 1 + hm[xx[0]]);
            }
            if hm.contains_key(xx[1]) && !hm.contains_key(xx[0]) {
                hm.insert(xx[0], 1 + hm[xx[1]]);
            }
        }
    }

    // subtract distance from YOU to first node and from last node to SAN
    hm["SAN"] - 2
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn test_count() {
        let map = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L";
        assert_eq!(total_orbits(map), 42);
    }
    #[test]
    fn test_find() {
        let map = "COM)B
        B)C
        C)D
        D)E
        E)F
        B)G
        G)H
        D)I
        E)J
        J)K
        K)L
        K)YOU
        I)SAN";
        assert_eq!(santa(map), 4);
    }
}
