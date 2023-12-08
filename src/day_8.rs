use std::collections::HashMap;

use num::integer::lcm;

pub fn generator(input: &str) -> (&str, HashMap<&str, (&str, &str)>) {
    let (directions, maps) = input.split_once("\n\n").unwrap();
    let maps = maps
        .lines()
        .map(|l| {
            let (map, links) = l.split_once(" = ").unwrap();
            let links = links[1..links.len() - 1].split_once(", ").unwrap();
            (map, links)
        })
        .collect();
    (directions, maps)
}

pub fn part1(input: &(&str, HashMap<&str, (&str, &str)>)) -> usize {
    let directions = input.0.bytes().collect::<Vec<_>>();
    let maps = &input.1;
    let mut current_map = "AAA";

    let mut i = 0;
    while *current_map != *"ZZZ" {
        let m = maps.get(current_map).unwrap();
        if directions[i % directions.len()] == b'R' {
            current_map = m.1;
        } else {
            current_map = m.0;
        }
        i += 1;
    }
    i
}

pub fn part2(input: &(&str, HashMap<&str, (&str, &str)>)) -> usize {
    let directions = input.0.bytes().collect::<Vec<_>>();
    let maps = &input.1;

    maps.keys()
        .filter(|v| v.ends_with('A'))
        .map(|v| compute_n(v, maps, &directions))
        .reduce(lcm)
        .unwrap()
}

fn compute_n(current_map: &str, maps: &HashMap<&str, (&str, &str)>, directions: &Vec<u8>) -> usize {
    let mut i = 0;
    let mut current_map = current_map;
    while !current_map.ends_with('Z') {
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

    // use helper_macro::test_parts;
    // test_parts!(2, 1698735, 1594785890);

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
