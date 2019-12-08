use bytecount::count;

pub fn check() -> Option<usize> {
    // Part A
    let mut input = include_str!("../input/day8.txt").trim();
    let width = 25;
    let height = 6;

    let resized = resize(input, width, height);
    let min = resized.iter().min_by_key(|&&x| count(x, b'0'))?;
    Some(count(min, b'1') * count(min, b'2'))
}

pub fn decode() -> Option<String> {
    // Part B
    let mut input = include_str!("../input/day8.txt").trim();
    let width = 25;
    let height = 6;

    let resized = resize(input, width, height);
    let length = resized[0].len();
    let ans = (0..length)
        .map(|i| {
            resized.iter().find(|s| s[i] != b'2').unwrap()[i]
        })
        .collect::<Vec<_>>();

    Some(ans
        .chunks_exact(width)
        .map(|chunk| {
            chunk
                .iter()
                .map(|&c| if c == b'1' { "*" } else { " " })
                .collect::<String>()
        })
        .collect::<Vec<_>>()
        .join("\n"))
}

fn resize(image: &str, width: usize, height: usize) -> Vec<&[u8]> {
    image
        .as_bytes()
        .chunks_exact(width * height)
        .collect::<Vec<_>>()
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    fn testssdfsdf() {
        assert_eq!(resize("123456789012", 3, 2), vec![b"123456", b"789012"]);
    }
}
