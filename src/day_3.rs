use std::collections::{HashMap, HashSet};

type Map = Vec<Vec<u8>>;

const ZERO: u8 = 48;
const NINE: u8 = 57;

fn is_digit(n: u8) -> bool {
    (ZERO..=NINE).contains(&n)
}

fn check_valid(x: usize, y: usize, map: &Map) -> bool {
    for x_d in 0..3 {
        for y_d in 0..3 {
            let x = (x + x_d).wrapping_sub(1);
            let y = (y + y_d).wrapping_sub(1);
            if x > map.len() - 1 || y > map[0].len() - 1 {
                continue;
            }
            let n = map[x][y];
            if !is_digit(n) && n != b'.' {
                return true;
            }
        }
    }
    false
}

fn find_gears(x: usize, y: usize, map: &Map) -> HashSet<(usize, usize)> {
    let mut out = HashSet::new();
    for x_d in 0..3 {
        for y_d in 0..3 {
            let x = (x + x_d).wrapping_sub(1);
            let y = (y + y_d).wrapping_sub(1);
            if x > map.len() - 1 || y > map[0].len() - 1 {
                continue;
            }
            if map[x][y] == b'*' {
                out.insert((x, y));
            }
        }
    }
    out
}

pub fn generator(input: &str) -> Vec<Vec<u8>> {
    input.lines().map(|line| line.bytes().collect()).collect()
}

pub fn part1(input: &Map) -> u64 {
    let mut number;
    let mut is_valid_number;
    let mut numbers = vec![];
    for x in 0..input.len() {
        number = 0;
        is_valid_number = false;
        for y in 0..input[0].len() {
            if is_digit(input[x][y]) {
                number *= 10;
                number += (input[x][y] - ZERO) as u64;
                is_valid_number |= check_valid(x, y, input);
            } else {
                if is_valid_number {
                    numbers.push(number);
                }
                number = 0;
                is_valid_number = false;
            }
        }
        if is_valid_number {
            numbers.push(number);
        }
    }
    numbers.iter().sum()
}

pub fn part2(input: &Map) -> u64 {
    let mut number;
    let mut gear_to_numbers: HashMap<(usize, usize), Vec<u64>> = HashMap::new();
    let mut valid_gears;
    for x in 0..input.len() {
        number = 0;
        valid_gears = HashSet::new();
        for y in 0..input[0].len() {
            if is_digit(input[x][y]) {
                number *= 10;
                number += (input[x][y] - ZERO) as u64;
                for g in find_gears(x, y, input) {
                    valid_gears.insert(g);
                }
            } else {
                for g in valid_gears {
                    if let std::collections::hash_map::Entry::Vacant(e) = gear_to_numbers.entry(g) {
                        e.insert(vec![number]);
                    } else {
                        gear_to_numbers.get_mut(&g).unwrap().push(number);
                    }
                }
                number = 0;
                valid_gears = HashSet::new();
            }
        }

        for g in valid_gears {
            if let std::collections::hash_map::Entry::Vacant(e) = gear_to_numbers.entry(g) {
                e.insert(vec![number]);
            } else {
                gear_to_numbers.get_mut(&g).unwrap().push(number);
            }
        }
    }
    gear_to_numbers
        .values()
        .filter(|v| v.len() == 2)
        .map(|v| v.iter().product::<u64>())
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(3, 507214, 72553319);

    #[test]
    fn test_base() {
        let example = "467..114..\n\
            ...*......\n\
            ..35..633.\n\
            ......#...\n\
            617*......\n\
            .....+.58.\n\
            ..592.....\n\
            ......755.\n\
            ...$.*....\n\
            .664.598..";
        assert_eq!(part1(&generator(example)), 4361);
        assert_eq!(part2(&generator(example)), 467835);
    }
}
