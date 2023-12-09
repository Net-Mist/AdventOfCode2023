use aoc_macro::p;

type Int = i64;
type Type = Vec<Vec<Int>>;

pub fn generator(input: &str) -> Type {
    input
        .lines()
        .map(|v| v.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

pub fn part1(input: &Type) -> Int {
    input.iter().map(|seq| get_next(seq)).sum()
}

fn get_next(seq: &Vec<Int>) -> Int {
    let diff = seq.windows(2).map(|s| s[1] - s[0]).collect::<Vec<_>>();
    if diff.iter().all(|v| *v == 0) {
        return seq[0];
    } else {
        let next = get_next(&diff);
        return seq[seq.len() - 1] + next;
    }
}

pub fn part2(input: &Type) -> Int {
    input.iter().map(|seq| get_previous(seq)).sum()
}
fn get_previous(seq: &Vec<Int>) -> Int {
    let diff = seq.windows(2).map(|s| s[1] - s[0]).collect::<Vec<_>>();
    if diff.iter().all(|v| *v == 0) {
        return seq[0];
    } else {
        let next = get_previous(&diff);
        return seq[0] - next;
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    // use helper_macro::test_parts;
    // test_parts!(2, 1698735, 1594785890);

    #[test]
    fn test_base() {
        let example = "0 3 6 9 12 15\n\
                            1 3 6 10 15 21\n\
                            10 13 16 21 30 45";
        assert_eq!(part1(&generator(example)), 114);
        assert_eq!(part2(&generator(example)), 1);
    }
}
