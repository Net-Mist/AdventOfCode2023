use std::str::from_utf8;

use ahash::{HashMap, HashMapExt};

type Input<'a> = Vec<(&'a u8, u8)>;

pub fn generator(input: &[u8]) -> &[u8] {
    input
}

pub fn generator1(input: &[u8]) -> Input {
    input[0..input.len() - 1]
        .split(|b| b == &b'\n')
        .map(|line| {
            let direction = line.first().unwrap();
            let mut iter = line[2..].splitn(2, |b| b == &b' ');
            let n = from_utf8(iter.next().unwrap()).unwrap().parse().unwrap();
            (direction, n)
        })
        .collect()
}

pub fn part1(input: &[u8]) -> usize {
    let input = generator1(input);

    let mut current_position = (0, 0);
    let mut walls = HashMap::<i128, Vec<(i128, i128)>>::new();
    for (direction, n) in input {
        let n = n as i128;
        let direction = match direction {
            b'U' => (0, -1),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => unreachable!(),
        };
        if direction.1 == 0 {
            let wall = if direction.0 == 1 {
                (current_position.0, current_position.0 + n)
            } else {
                (current_position.0 - n, current_position.0)
            };
            walls
                .entry(current_position.1)
                .and_modify(|v| v.push(wall))
                .or_insert(vec![wall]);
        }
        current_position.0 += direction.0 * n;
        current_position.1 += direction.1 * n;
    }

    area(walls)
}

fn area(walls: std::collections::HashMap<i128, Vec<(i128, i128)>, ahash::RandomState>) -> usize {
    let mut k = walls.keys().collect::<Vec<_>>();
    k.sort_unstable();

    let mut current_inside_segment = vec![];
    let mut previous_y = i128::MIN + 10;
    let mut area = 0;
    for y in k {
        let surface = current_inside_segment
            .iter()
            .map(|(a, b)| b - a + 1)
            .sum::<i128>();

        area += surface * (y - previous_y - 1);
        let w = &walls.get(y).unwrap();

        let union = union(&current_inside_segment, w);
        area += union.iter().map(|(a, b)| b - a + 1).sum::<i128>();

        previous_y = *y;
        current_inside_segment = xor(&current_inside_segment, w);
    }

    area as usize
}

pub fn part2(input: &[u8]) -> usize {
    let input = input[0..input.len() - 1]
        .split(|b| b == &b'\n')
        .map(|line| {
            let hex = line[0..line.len() - 1]
                .splitn(2, |b| b == &b'#')
                .nth(1)
                .unwrap();
            let (direction, n) = hex.split_last().unwrap();
            let mut num = 0u32;
            for e in n {
                num *= 16;
                if e.is_ascii_digit() {
                    num += (e - b'0') as u32;
                } else {
                    num += (e - b'a' + 10) as u32;
                }
            }
            (*direction - b'0', num)
        })
        .collect::<Vec<(u8, u32)>>();

    let mut current_position = (0, 0);
    let mut walls = HashMap::<i128, Vec<(i128, i128)>>::new();
    for (direction, n) in input {
        let n = n as i128;
        let direction = match direction {
            3 => (0, -1),
            1 => (0, 1),
            2 => (-1, 0),
            0 => (1, 0),
            _ => unreachable!(),
        };
        if direction.1 == 0 {
            let wall = if direction.0 == 1 {
                (current_position.0, current_position.0 + n)
            } else {
                (current_position.0 - n, current_position.0)
            };
            walls
                .entry(current_position.1)
                .and_modify(|v| v.push(wall))
                .or_insert(vec![wall]);
        }
        current_position.0 += direction.0 * n;
        current_position.1 += direction.1 * n;
    }

    area(walls)
}

fn xor(a: &Vec<(i128, i128)>, b: &Vec<(i128, i128)>) -> Vec<(i128, i128)> {
    let mut a = a.to_owned();
    a.append(&mut b.to_owned());
    a.sort_unstable();

    let mut xor = vec![];

    let (first, a) = a.split_first().unwrap();
    let mut current_open = first.0;
    let mut current_close = first.1;

    for e in a {
        if e.0 > current_close {
            if current_open != current_close {
                xor.push((current_open, current_close));
            }
            current_open = e.0;
            current_close = e.1;
            continue;
        }
        if e.0 == current_close {
            current_close = e.1;
            continue;
        }
        if e.0 != current_open {
            xor.push((current_open, e.0));
        }
        current_open = e.1.min(current_close);
        current_close = e.1.max(current_close);
    }
    if current_open != current_close {
        xor.push((current_open, current_close));
    }
    xor
}

fn union(a: &Vec<(i128, i128)>, b: &Vec<(i128, i128)>) -> Vec<(i128, i128)> {
    let mut a = a.to_owned();
    a.append(&mut b.to_owned());
    a.sort_unstable();

    let mut union = vec![];

    let (first, a) = a.split_first().unwrap();
    let mut current_open = first.0;
    let mut current_close = first.1;

    for e in a {
        if e.0 > current_close {
            union.push((current_open, current_close));
            current_open = e.0;
            current_close = e.1;
            continue;
        }
        current_close = e.1.max(current_close);
    }
    union.push((current_open, current_close));
    union
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_xor() {
        let a = vec![(2, 6)];
        let b = vec![(4, 6), (0, 2)];
        assert_eq!(xor(&a, &b), vec![(0, 4)]);
    }
    #[test]
    fn test_union() {
        let a = vec![(2, 6)];
        let b = vec![(4, 6), (0, 2)];
        assert_eq!(union(&a, &b), vec![(0, 6)]);
    }

    // use aoc_macro::test_parts;
    // test_parts!(2, 1698735, 1594785890);

    #[test]
    fn test_base() {
        let example = "R 6 (#70c710)\n\
                            D 5 (#0dc571)\n\
                            L 2 (#5713f0)\n\
                            D 2 (#d2c081)\n\
                            R 2 (#59c680)\n\
                            D 2 (#411b91)\n\
                            L 5 (#8ceee2)\n\
                            U 2 (#caa173)\n\
                            L 1 (#1b58a2)\n\
                            U 2 (#caa171)\n\
                            R 2 (#7807d2)\n\
                            U 3 (#a77fa3)\n\
                            L 2 (#015232)\n\
                            U 2 (#7a21e3)\n"
            .as_bytes();
        assert_eq!(part1(generator(example)), 62);
        assert_eq!(part2(generator(example)), 952408144115);
    }
}
