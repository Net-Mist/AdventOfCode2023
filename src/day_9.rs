use arrayvec::ArrayVec;

type Int = i64;
type Line = ArrayVec<Int, 21>;
type Type = ArrayVec<Line, 200>;

pub fn generator(input: &str) -> Type {
    input
        .lines()
        .map(|v| v.split_whitespace().map(|n| n.parse().unwrap()).collect())
        .collect()
}

pub fn part1(input: &Type) -> Int {
    input.iter().map(get_next_decl).sum()
}

fn get_next_decl(seq: &Line) -> Int {
    let mut last = seq[seq.len() - 1];
    let mut diff = seq.windows(2).map(|s| s[1] - s[0]).collect::<Line>();

    while diff.iter().any(|v| *v != 0) {
        last += diff[diff.len() - 1];
        for i in 0..diff.len() - 1 {
            diff[i] = diff[i + 1] - diff[i]
        }
        diff.remove(diff.len() - 1);
    }
    last
}

pub fn part1_rec(input: &Type) -> Int {
    input.iter().map(get_next).sum()
}

fn get_next(seq: &Line) -> Int {
    let diff = seq.windows(2).map(|s| s[1] - s[0]).collect::<Line>();
    if diff.iter().all(|v| *v == 0) {
        seq[0]
    } else {
        seq[seq.len() - 1] + get_next(&diff)
    }
}

pub fn part2(input: &Type) -> Int {
    input
        .iter()
        .map(|s| get_next_decl(&s.iter().cloned().rev().collect()))
        .sum()
}

pub fn part2_get_previous(input: &Type) -> Int {
    input.iter().map(get_previous).sum()
}

fn get_previous(seq: &Line) -> Int {
    let diff = seq.windows(2).map(|s| s[1] - s[0]).collect::<Line>();
    if diff.iter().all(|v| *v == 0) {
        seq[0]
    } else {
        seq[0] - get_previous(&diff)
    }
}
#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(9, 1995001648, 988);

    #[test]
    fn test_base() {
        let example = "0 3 6 9 12 15\n\
                            1 3 6 10 15 21\n\
                            10 13 16 21 30 45";
        assert_eq!(part1(&generator(example)), 114);
        assert_eq!(part2(&generator(example)), 2);
    }
}
