use std::collections::BinaryHeap;

use aoc_macro::p;
use itertools::Itertools;

#[derive(PartialEq, Eq, PartialOrd, Ord, Clone, Debug)]
pub struct Hand {
    strength: u8,
    cards: Vec<u8>,
    bid: u32,
}

fn compute_strength1(cards: &[u8]) -> u8 {
    let mut c = cards.iter().counts();
    let l = c.len();
    let mut m = c.values().max().unwrap().to_owned();

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
    let j = cards.iter().filter(|v| **v == 0).count();
    let mut c = cards.iter().filter(|c| **c != 0).counts();

    let l = c.len();

    if l == 0 {
        return 7;
    }
    let mut m = c.values().max().unwrap().to_owned();

    let mut v_to_change = 0;
    for (&&k, &v) in c.iter() {
        if v == m {
            v_to_change = k;
        }
    }
    let new_v = c[&v_to_change] + j;
    c.insert(&v_to_change, new_v);
    // c[k] += j;
    m += j;

    let l = c.len();
    let mut m = c.values().max().unwrap().to_owned();

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
                .collect::<Vec<_>>();
            let strength = compute_strength1(&cards);
            Hand {
                cards,
                strength,
                bid,
            }
        })
        .collect::<Vec<Hand>>();
    v.sort_unstable();
    v.into_iter()
        .enumerate()
        .map(|(i, v)| {
            p!(v);
            p!(i);
            let b = v.bid;
            p!(b);
            (i as u64 + 1) * v.bid as u64
        })
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
                .collect::<Vec<_>>();
            let strength = compute_strength2(&cards);
            Hand {
                cards,
                strength,
                bid,
            }
        })
        .collect::<Vec<Hand>>();
    v.sort_unstable();
    v.into_iter()
        .enumerate()
        .map(|(i, v)| {
            p!(v);
            p!(i);
            let b = v.bid;
            p!(b);
            (i as u64 + 1) * v.bid as u64
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    // use helper_macro::test_parts;
    // test_parts!(2, 1698735, 1594785890);

    #[test]
    fn test_base() {
        let example = "32T3K 765\n\
                        T55J5 684\n\
                        KK677 28\n\
                        KTJJT 220\n\
                        QQQJA 483";
        assert_eq!(part1(&generator(example)), 6440);
        assert_eq!(part2(&generator(example)), 248652697);
    }
}
