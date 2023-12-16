use std::str::from_utf8;

use ahash::{HashSet, HashSetExt};

pub fn generator(input: &[u8]) -> Vec<&[u8]> {
    let input = from_utf8(input).unwrap();

    input[0..input.len() - 1]
        .as_bytes()
        .split(|b| b == &b'\n')
        .collect()
}

pub fn part1(input: &[&[u8]]) -> usize {
    let initial_position = (0, 0);
    let initial_direction = (0, 1);
    find_n_energized(initial_position, initial_direction, input)
}

fn find_n_energized(
    initial_position: (usize, usize),
    initial_direction: (i32, i32),
    input: &[&[u8]],
) -> usize {
    let mut to_process = vec![(initial_position, initial_direction)];
    let h = input.len();
    let w = input[0].len();
    let mut energized = vec![vec![false; w]; h];
    let mut seen = HashSet::new();
    while let Some((position, direction)) = to_process.pop() {
        if position.0 >= h || position.1 >= w {
            continue;
        }
        if seen.contains(&(position, direction)) {
            continue;
        }
        seen.insert((position, direction));
        energized[position.0][position.1] = true;
        let t = input.get(position.0).unwrap().get(position.1).unwrap();
        match direction {
            // right
            (0, 1) => match t {
                b'-' | b'.' => {
                    to_process.push(((position.0, position.1 + 1), direction));
                }
                b'|' => {
                    to_process.push(((position.0 + 1, position.1), (1, 0)));
                    to_process.push(((position.0.wrapping_sub(1), position.1), (-1, 0)));
                }
                b'\\' => {
                    to_process.push(((position.0 + 1, position.1), (1, 0)));
                }
                b'/' => {
                    to_process.push(((position.0.wrapping_sub(1), position.1), (-1, 0)));
                }
                _ => unreachable!(),
            },
            // left
            (0, -1) => match t {
                b'-' | b'.' => {
                    to_process.push(((position.0, position.1.wrapping_sub(1)), direction));
                }
                b'|' => {
                    to_process.push(((position.0 + 1, position.1), (1, 0)));
                    to_process.push(((position.0.wrapping_sub(1), position.1), (-1, 0)));
                }
                b'\\' => {
                    to_process.push(((position.0.wrapping_sub(1), position.1), (-1, 0)));
                }
                b'/' => {
                    to_process.push(((position.0 + 1, position.1), (1, 0)));
                }
                _ => unreachable!(),
            },
            // down
            (1, 0) => match t {
                b'|' | b'.' => {
                    to_process.push(((position.0 + 1, position.1), direction));
                }
                b'-' => {
                    to_process.push(((position.0, position.1 + 1), (0, 1)));
                    to_process.push(((position.0, position.1.wrapping_sub(1)), (0, -1)));
                }
                b'\\' => {
                    to_process.push(((position.0, position.1 + 1), (0, 1)));
                }
                b'/' => {
                    to_process.push(((position.0, position.1.wrapping_sub(1)), (0, -1)));
                }
                _ => unreachable!(),
            },
            // up
            (-1, 0) => match t {
                b'|' | b'.' => {
                    to_process.push(((position.0.wrapping_sub(1), position.1), direction));
                }
                b'-' => {
                    to_process.push(((position.0, position.1 + 1), (0, 1)));
                    to_process.push(((position.0, position.1.wrapping_sub(1)), (0, -1)));
                }
                b'\\' => {
                    to_process.push(((position.0, position.1.wrapping_sub(1)), (0, -1)));
                }
                b'/' => {
                    to_process.push(((position.0, position.1 + 1), (0, 1)));
                }
                _ => unreachable!(),
            },
            _ => unreachable!(),
        }
    }
    energized
        .iter()
        .map(|line| line.iter().filter(|e| **e).count())
        .sum()
}

pub fn part2(input: &[&[u8]]) -> usize {
    let h = input.len();
    let w = input[0].len();

    let mut max = 0;
    // left
    for p in 0..h {
        let initial_position = (p, 0);
        let initial_direction = (0, 1);
        max = max.max(find_n_energized(initial_position, initial_direction, input));
    }
    // right
    for p in 0..h {
        let initial_position = (p, w - 1);
        let initial_direction = (0, -1);
        max = max.max(find_n_energized(initial_position, initial_direction, input));
    }
    // top
    for p in 0..w {
        let initial_position = (0, p);
        let initial_direction = (1, 0);
        max = max.max(find_n_energized(initial_position, initial_direction, input));
    }
    // bottom
    for p in 0..w {
        let initial_position = (h - 1, p);
        let initial_direction = (-1, 0);
        max = max.max(find_n_energized(initial_position, initial_direction, input));
    }
    max
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(16, 8551, 8754);

    #[test]
    fn test_base() {
        let example = ".|...\\....\n\
                        |.-.\\.....\n\
                        .....|-...\n\
                        ........|.\n\
                        ..........\n\
                        .........\\\n\
                        ..../.\\\\..\n\
                        .-.-/..|..\n\
                        .|....-|.\\\n\
                        ..//.|....\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 46);
        assert_eq!(part2(&generator(example)), 51);
    }
}
