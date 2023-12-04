use std::collections::HashSet;

pub fn generator(input: &str) -> Vec<(HashSet<u16>, HashSet<u16>)> {
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (wining, numbers) = line.split_once(" | ").unwrap();
            let wining = wining
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
                .collect();
            let numbers = numbers
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| n.parse().unwrap())
                .collect();
            (wining, numbers)
        })
        .collect()
}

pub fn part1(input: &Vec<(HashSet<u16>, HashSet<u16>)>) -> usize {
    input
        .iter()
        .map(|(w, n)| {
            let i = w.intersection(n).count();
            if i == 0 {
                return 0;
            }
            1 << (i - 1)
        })
        .sum()
}

pub fn part2(input: &Vec<(HashSet<u16>, HashSet<u16>)>) -> usize {
    let mut weight_vector = vec![1; input.len()];
    for (i, (w, n)) in input.iter().enumerate() {
        for j in 0..w.intersection(n).count() {
            if i + j + 1 < weight_vector.len() {
                weight_vector[i + j + 1] += weight_vector[i];
            }
        }
    }
    weight_vector.iter().sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(4, 25231, 9721255);

    #[test]
    fn test_base() {
        let example = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53\n\
            Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19\n\
            Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1\n\
            Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83\n\
            Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36\n\
            Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11\n";
        assert_eq!(part1(&generator(example)), 13);
        assert_eq!(part2(&generator(example)), 30);
    }
}
