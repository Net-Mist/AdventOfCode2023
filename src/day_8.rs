use arrayvec::ArrayVec;
use nohash_hasher::{BuildNoHashHasher, NoHashHasher};
use std::{collections::HashMap, hash::BuildHasherDefault};

use num::integer::lcm;

const MIN_ASCII: u8 = 48;
const RANGE_ASCII: u32 = 90 - 48;

type Map = HashMap<u32, u16, BuildHasherDefault<NoHashHasher<u32>>>;
type Type = (Vec<u8>, Maps);

type Int = u16;

#[derive(Default, Debug)]
struct Node {
    left: Int,
    right: Int,
}

#[derive(Default, Debug)]
pub struct Maps {
    nodes: ArrayVec<Node, 766>,
    begining_part2: ArrayVec<Int, 10>,
    end_part2: ArrayVec<Int, 10>,
    beginning_part1: Int,
    end_part1: Int,
}

impl Maps {
    fn left_node(&self, id: Int) -> Int {
        self.nodes[id as usize].left
    }

    fn right_node(&self, id: Int) -> Int {
        self.nodes[id as usize].right
    }

    fn new_node(&mut self) -> Int {
        let id = self.nodes.len() as Int;
        self.nodes.push(Node::default());
        id
    }
}

fn perfect_hash(text: &str) -> u32 {
    text.bytes()
        .map(|x| (x - MIN_ASCII) as u32)
        .reduce(|acc, x| acc * RANGE_ASCII + x)
        .unwrap()
}

pub fn generator(input: &str) -> Type {
    let (directions, maps) = input.split_once("\n\n").unwrap();

    let mut str_to_id: Map = HashMap::with_capacity_and_hasher(766, BuildNoHashHasher::default());
    let mut all_maps = Maps::default();

    for l in maps.lines() {
        let (map, links) = l.split_once(" = ").unwrap();
        let links = links[1..links.len() - 1].split_once(", ").unwrap();

        let map_id = str_to_id
            .entry(perfect_hash(map))
            .or_insert_with(|| all_maps.new_node())
            .to_owned();
        let left_id = str_to_id
            .entry(perfect_hash(links.0))
            .or_insert_with(|| all_maps.new_node())
            .to_owned();
        let right_id = str_to_id
            .entry(perfect_hash(links.1))
            .or_insert_with(|| all_maps.new_node())
            .to_owned();
        all_maps.nodes[map_id as usize] = Node {
            left: left_id.to_owned() as u16,
            right: right_id.to_owned() as u16,
        };

        if map == "AAA" {
            all_maps.beginning_part1 = map_id;
        }
        if map == "ZZZ" {
            all_maps.end_part1 = map_id;
        }
        if map.ends_with("A") {
            all_maps.begining_part2.push(map_id);
        }
        if map.ends_with("Z") {
            all_maps.end_part2.push(map_id);
        }
    }
    (directions.bytes().collect(), all_maps)
}

pub fn part1(input: &Type) -> usize {
    let directions = &input.0;
    let maps = &input.1;
    let mut current_map = maps.beginning_part1;

    let mut i = 0;
    while current_map != maps.end_part1 {
        if directions[i % directions.len()] == b'R' {
            current_map = maps.right_node(current_map);
        } else {
            current_map = maps.left_node(current_map);
        }
        i += 1;
    }
    i
}

pub fn part2(input: &Type) -> usize {
    let directions = &input.0;
    let maps = &input.1;

    maps.begining_part2
        .iter()
        .map(|v| compute_n(*v, maps, directions))
        .reduce(lcm)
        .unwrap()
}

fn compute_n(current_map: Int, maps: &Maps, directions: &Vec<u8>) -> usize {
    let mut i = 0;
    let mut current_map = current_map;
    while !maps.end_part2.contains(&current_map) {
        if directions[i % directions.len()] == b'R' {
            current_map = maps.right_node(current_map);
        } else {
            current_map = maps.left_node(current_map);
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
