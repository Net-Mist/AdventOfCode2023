use std::collections::{HashMap, HashSet};

type Map<'a> = Vec<&'a [u8]>;

const ZERO: u8 = 48;
const NINE: u8 = 57;

fn is_digit(n: u8) -> bool {
    // Keep this notation as it seems to be slightly faster than clippy proposal
    // (ZERO..=NINE).contains(&n)
    // TODO look at ASM code
    #![allow(clippy::all)]
    ZERO <= n && NINE >= n
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

pub fn generator(input: &[u8]) -> Vec<&[u8]> {
    input[0..input.len() - 1].split(|b| b == &b'\n').collect()
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

pub fn part2_hash(input: &Map) -> u64 {
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

pub fn part2(input: &Map) -> u64 {
    let mut s = 0;
    for (x, line) in input.iter().enumerate() {
        for (y, e) in line.iter().enumerate() {
            if *e != b'*' {
                continue;
            }
            // at this point, we know that (x, y) is not on the border of the map

            // check if 2 numbers are present
            let mut n = 0;
            if is_digit(input[x][y - 1]) {
                n += 1;
            }
            if is_digit(input[x][y + 1]) {
                n += 1;
            }
            if is_digit(input[x - 1][y]) {
                n += 1;
            } else {
                if is_digit(input[x - 1][y - 1]) {
                    n += 1
                }
                if is_digit(input[x - 1][y + 1]) {
                    n += 1
                }
            }
            if is_digit(input[x + 1][y]) {
                n += 1;
            } else {
                if is_digit(input[x + 1][y - 1]) {
                    n += 1
                }
                if is_digit(input[x + 1][y + 1]) {
                    n += 1
                }
            }
            if n != 2 {
                continue;
            }

            // parse the 2 numbers
            let mut p = 1;
            if is_digit(input[x][y - 1]) {
                p *= parse_left(input, x, y - 1);
            }
            if is_digit(input[x][y + 1]) {
                p *= parse_right(input, x, y + 1);
            }
            if is_digit(input[x - 1][y]) {
                p *= parse_left_right(input, x - 1, y);
            } else {
                if is_digit(input[x - 1][y - 1]) {
                    p *= parse_left(input, x - 1, y - 1);
                }
                if is_digit(input[x - 1][y + 1]) {
                    p *= parse_right(input, x - 1, y + 1);
                }
            }
            if is_digit(input[x + 1][y]) {
                p *= parse_left_right(input, x + 1, y);
            } else {
                if is_digit(input[x + 1][y - 1]) {
                    p *= parse_left(input, x + 1, y - 1);
                }
                if is_digit(input[x + 1][y + 1]) {
                    p *= parse_right(input, x + 1, y + 1);
                }
            }

            s += p;
        }
    }
    s
}

fn parse_left_right(input: &[&[u8]], x: usize, y: usize) -> u64 {
    match (is_digit(input[x][y - 1]), is_digit(input[x][y + 1])) {
        (true, true) => parse_left(input, x, y + 1),
        (true, false) => parse_left(input, x, y),
        (false, true) => parse_right(input, x, y),
        (false, false) => (input[x][y] - ZERO) as u64,
    }
}

fn parse_right(input: &[&[u8]], x: usize, y: usize) -> u64 {
    let mut n = (input[x][y] - ZERO) as u64;
    if y < input[0].len() - 1 {
        if is_digit(input[x][y + 1]) {
            n *= 10;
            n += (input[x][y + 1] - ZERO) as u64;
        } else {
            return n;
        }
    }
    if y < input[0].len() - 2 && is_digit(input[x][y + 2]) {
        n *= 10;
        n += (input[x][y + 2] - ZERO) as u64;
    }
    n
}

fn parse_left(input: &[&[u8]], x: usize, y: usize) -> u64 {
    let mut n = (input[x][y] - ZERO) as u64;
    if y > 0 {
        if is_digit(input[x][y - 1]) {
            n += (input[x][y - 1] - ZERO) as u64 * 10;
        } else {
            return n;
        }
    }
    if y > 1 && is_digit(input[x][y - 2]) {
        n += (input[x][y - 2] - ZERO) as u64 * 100;
    }
    n
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
            .664.598..\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 4361);
        assert_eq!(part2(&generator(example)), 467835);
        assert_eq!(part2_hash(&generator(example)), 467835);
    }
}
