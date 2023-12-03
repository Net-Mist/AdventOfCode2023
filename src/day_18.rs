pub fn generator(input: &str) -> &str {
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

    // use helper_macro::test_parts;
    // test_parts!(2, 1698735, 1594785890);

    #[test]
    fn test_base() {
        let example = "forward 5\n\
        down 5\n\
        forward 8\n\
        up 3\n\
        down 8\n\
        forward 2";
        assert_eq!(part1(generator(example)), 1);
        assert_eq!(part2(generator(example)), 1);
    }
}
