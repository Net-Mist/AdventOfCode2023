use std::str::from_utf8;

pub fn generator(input: &[u8]) -> &str {
    let input = from_utf8(input).unwrap();
    input
}

pub fn part1(_input: &str) -> usize {
    1
}

pub fn part2(_input: &str) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    // use aoc_macro::test_parts;
    // test_parts!(2, 1698735, 1594785890);

    #[test]
    fn test_base() {
        let example = "\n".as_bytes();
        assert_eq!(part1(generator(example)), 1);
        assert_eq!(part2(generator(example)), 1);
    }
}
