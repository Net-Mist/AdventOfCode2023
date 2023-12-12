use arrayvec::ArrayVec;
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Hand {
    strength: u8,
    cards: ArrayVec<u8, 5>,
    bid: u32,
}

fn compute_strength1(cards: &[u8]) -> u8 {
    let c = cards.iter().counts();
    let l = c.len();
    let m = c.values().max().unwrap().to_owned();

    match (l, m) {
        (1, 5) => 7,
        (2, 4) => 6,
        (2, 3) => 5,
        (3, 3) => 4,
        (3, 2) => 3,
        (4, 2) => 2,
        (5, 1) => 1,
        _ => unreachable!(),
    }
}

fn compute_strength2(cards: &[u8]) -> u8 {
    let mut c = cards.iter().counts();
    if c.len() == 1 {
        return 7;
    }

    // handle jokers
    let n_j = c.remove(&0).unwrap_or_default();
    let (k, v) = c.iter().max_by(|x, y| x.1.cmp(y.1)).unwrap();
    c.insert(k, v + n_j);

    let l = c.len();
    let m = c.values().max().unwrap().to_owned();

    match (l, m) {
        (1, 5) => 7,
        (2, 4) => 6,
        (2, 3) => 5,
        (3, 3) => 4,
        (3, 2) => 3,
        (4, 2) => 2,
        (5, 1) => 1,
        _ => unreachable!(),
    }
}

pub fn generator(input: &str) -> &str {
    input
}

pub fn part1(input: &str) -> u64 {
    let mut v = input
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let cards = cards
                .bytes()
                .map(|v| match v {
                    b'A' => 15,
                    b'K' => 14,
                    b'Q' => 13,
                    b'J' => 12,
                    b'T' => 11,
                    v => v - 48,
                })
                .collect::<ArrayVec<_, 5>>();
            let strength = compute_strength1(&cards);
            Hand {
                cards,
                strength,
                bid,
            }
        })
        .collect::<ArrayVec<Hand, 1000>>();
    v.sort_unstable();
    v.into_iter()
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * v.bid as u64)
        .sum()
}

pub fn part2(input: &str) -> u64 {
    let mut v = input
        .lines()
        .map(|l| {
            let (cards, bid) = l.split_once(' ').unwrap();
            let bid = bid.parse().unwrap();
            let cards = cards
                .bytes()
                .map(|v| match v {
                    b'A' => 15,
                    b'K' => 14,
                    b'Q' => 13,
                    b'J' => 0,
                    b'T' => 11,
                    v => v - 48,
                })
                .collect::<ArrayVec<_, 5>>();
            let strength = compute_strength2(&cards);
            Hand {
                cards,
                strength,
                bid,
            }
        })
        .collect::<ArrayVec<Hand, 1000>>();
    v.sort_unstable();
    v.into_iter()
        .enumerate()
        .map(|(i, v)| (i as u64 + 1) * v.bid as u64)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(7, 250453939, 248652697);

    #[test]
    fn test_base() {
        let example = "32T3K 765\n\
                        T55J5 684\n\
                        KK677 28\n\
                        KTJJT 220\n\
                        QQQJA 483";
        assert_eq!(part1(generator(example)), 6440);
        assert_eq!(part2(generator(example)), 5905);
    }
}
