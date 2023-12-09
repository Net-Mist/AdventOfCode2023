pub fn generator(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> usize {
    let mut input = input.lines().map(|line| {
        line.split_whitespace()
            .skip(1)
            .map(|v| v.parse::<f64>().unwrap())
    });
    input
        .next()
        .unwrap()
        .zip(input.next().unwrap())
        .map(|(t, d)| extract_roots(t, d))
        .product()
}

pub fn part2(input: &str) -> usize {
    let input = input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .skip(1)
                .fold("".to_string(), |a, b| a + b)
                .parse()
                .unwrap()
        })
        .collect::<Vec<f64>>();
    let t = input[0];
    let d = input[1];
    extract_roots(t, d)
}

fn extract_roots(t: f64, d: f64) -> usize {
    let sqrt_delta: f64 = (t * t - 4. * d).sqrt();
    let s1 = (t + sqrt_delta) / 2. - 1e-5;
    let s2 = (t - sqrt_delta) / 2. + 1e-5;
    (s1.floor() - s2.ceil()) as usize + 1
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(6, 771628, 27363861);

    #[test]
    fn test_base() {
        let example = "Time:      7  15   30\n\
            Distance:  9  40  200\n\
            ";
        assert_eq!(part1(generator(example)), 288);
        assert_eq!(part2(generator(example)), 71503);
    }
}
