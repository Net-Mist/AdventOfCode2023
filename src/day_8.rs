use nohash_hasher::NoHashHasher;
use std::{collections::HashMap, hash::BuildHasherDefault};

use num::integer::lcm;

const MIN_ASCII: u8 = 48;
const RANGE_ASCII: u32 = 90 - 48;

type Map = HashMap<u32, (u32, u32), BuildHasherDefault<NoHashHasher<u32>>>;
type Type = (Vec<u8>, Map);

fn perfect_hash(text: &str) -> u32 {
    text.bytes()
        .map(|x| (x - MIN_ASCII) as u32)
        .reduce(|acc, x| acc * RANGE_ASCII + x)
        .unwrap()
}

pub fn generator(input: &str) -> Type {
    let (directions, maps) = input.split_once("\n\n").unwrap();
    let maps = maps
        .lines()
        .map(|l| {
            let (map, links) = l.split_once(" = ").unwrap();
            let links = links[1..links.len() - 1].split_once(", ").unwrap();
            (
                perfect_hash(map),
                (perfect_hash(links.0), perfect_hash(links.1)),
            )
        })
        .collect();
    (directions.bytes().collect(), maps)
}

pub fn part1(input: &Type) -> usize {
    let directions = &input.0;
    let maps = &input.1;
    let mut current_map = perfect_hash("AAA");

    let mut i = 0;
    while current_map != perfect_hash("ZZZ") {
        let m = maps.get(&current_map).unwrap();
        if directions[i % directions.len()] == b'R' {
            current_map = m.1;
        } else {
            current_map = m.0;
        }
        i += 1;
    }
    i
}

pub fn part2(input: &Type) -> usize {
    let directions = &input.0;
    let maps = &input.1;

    maps.keys()
        .filter(|v| **v % RANGE_ASCII == (b'A' - MIN_ASCII) as u32)
        .map(|v| compute_n(*v, maps, directions))
        .reduce(lcm)
        .unwrap()
}

fn compute_n(current_map: u32, maps: &Map, directions: &Vec<u8>) -> usize {
    let mut i = 0;
    let mut current_map = current_map;
    while 0 != (current_map % RANGE_ASCII) as u8 {
        let m = maps.get(&current_map).unwrap();
        if directions[i % directions.len()] == b'R' {
            current_map = m.1;
        } else {
            current_map = m.0;
        }
        i += 1;
    }
    i
}
#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(8, 20513, 15995167053923);

    #[test]
    fn test_base() {
        let example = "LLR\n\
            \n\
            AAA = (BBB, BBB)\n\
            BBB = (AAA, ZZZ)\n\
            ZZZ = (ZZZ, ZZZ)";
        assert_eq!(part1(&generator(example)), 6);

        let example = "LR\n\
                            \n\
                            11A = (11B, XXX)\n\
                            11B = (XXX, 11Z)\n\
                            11Z = (11B, XXX)\n\
                            22A = (22B, XXX)\n\
                            22B = (22C, 22C)\n\
                            22C = (22Z, 22Z)\n\
                            22Z = (22B, 22B)\n\
                            XXX = (XXX, XXX)";

        assert_eq!(part2(&generator(example)), 6);
    }
}
