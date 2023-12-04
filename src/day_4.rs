use ahash::AHashSet;

type Data = Vec<(AHashSet<u8>, AHashSet<u8>)>;

pub fn generator(input: &str) -> &str {
    input
}

pub fn generator_bitset(input: &str) -> Vec<u32> {
    // number are all lower than 100. So instead of using a hashing function,
    // we could set bit of a u128 to 1 if the number is present, and compute the
    // intersection with a logical or
    input
        .lines()
        .map(|line| {
            let (_, line) = line.split_once(": ").unwrap();
            let (wining, numbers) = line.split_once(" | ").unwrap();
            let wining = wining
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| 1 << n.parse::<u8>().unwrap())
                .reduce(|acc: u128, e| acc | e)
                .unwrap();
            let numbers = numbers
                .split(' ')
                .filter(|n| !n.is_empty())
                .map(|n| 1 << n.parse::<u8>().unwrap())
                .reduce(|acc: u128, e| acc | e)
                .unwrap();
            let r: u128 = wining & numbers;
            r.count_ones()
        })
        .collect()
}

pub fn part1(input: &str) -> usize {
    generator_bitset(input)
        .iter()
        .map(|i| {
            if *i == 0 {
                return 0;
            }
            1 << (*i - 1)
        })
        .sum()
}

pub fn part2(input: &str) -> usize {
    let input = generator_bitset(input);
    let mut weight_vector = vec![1; input.len()];
    for (i, n) in input.iter().enumerate() {
        for j in 0..*n as usize {
            if i + j + 1 < weight_vector.len() {
                weight_vector[i + j + 1] += weight_vector[i];
            }
        }
    }
    weight_vector.iter().sum()
}

pub fn generator_set(input: &str) -> Data {
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

pub fn part1_set(input: &str) -> usize {
    generator_set(input)
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

pub fn part2_set(input: &str) -> usize {
    let input = generator_set(input);
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
