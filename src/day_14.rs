use ahash::{HashMap, HashMapExt};

use arrayvec::ArrayVec;

type Input = ArrayVec<ArrayVec<u8, 100>, 100>;

pub fn generator(input: &str) -> Input {
    input.lines().map(|line| line.bytes().collect()).collect()
}

pub fn part1(input: &Input) -> usize {
    let h = input.len();
    let w = input[0].len();
    let mut input = input.to_owned();

    for c in 0..w {
        let mut next_position = 0;
        for l in 0..h {
            if input[l][c] == b'O' {
                input[l][c] = b'.';
                input[next_position][c] = b'O';
                next_position += 1;
            } else if input[l][c] == b'#' {
                next_position = l + 1;
            }
        }
    }

    total_load(input)
}

pub fn part2(input: &Input) -> usize {
    let h = input.len();
    let w = input[0].len();
    let mut input = input.to_owned();

    let mut map_to_step = HashMap::new();
    for i in 0..30000 {
        step(h, w, &mut input);
        if map_to_step.contains_key(&input) {
            let previous_i = map_to_step.get(&input).unwrap();
            let period = (1000000000 - 1 - i) / (i - previous_i);
            let i = i + period * (i - previous_i);
            let r = 999999999 - i;
            for _ in 0..r {
                step(h, w, &mut input);
            }
            return total_load(input);
        }

        map_to_step.insert(input.to_owned(), i);
    }
    0
}

fn total_load(input: Input) -> usize {
    input
        .iter()
        .rev()
        .enumerate()
        .map(|(u, l)| l.iter().filter(|e| **e == b'O').count() * (u + 1))
        .sum()
}

fn step(h: usize, w: usize, input: &mut Input) {
    let mut next_position;
    for c in 0..w {
        next_position = 0;
        for l in 0..h {
            if input[l][c] == b'O' {
                input[l][c] = b'.';
                input[next_position][c] = b'O';
                next_position += 1;
            } else if input[l][c] == b'#' {
                next_position = l + 1;
            }
        }
    }

    for l in 0..h {
        next_position = 0;
        for c in 0..w {
            if input[l][c] == b'O' {
                input[l][c] = b'.';
                input[l][next_position] = b'O';
                next_position += 1;
            } else if input[l][c] == b'#' {
                next_position = c + 1;
            }
        }
    }

    for c in 0..w {
        next_position = h - 1;
        for l in (0..h).rev() {
            if input[l][c] == b'O' {
                input[l][c] = b'.';
                input[next_position][c] = b'O';
                next_position = next_position.wrapping_sub(1);
            } else if input[l][c] == b'#' {
                next_position = l.wrapping_sub(1);
            }
        }
    }

    for l in 0..h {
        next_position = h - 1;
        for c in (0..w).rev() {
            if input[l][c] == b'O' {
                input[l][c] = b'.';
                input[l][next_position] = b'O';
                next_position = next_position.wrapping_sub(1);
            } else if input[l][c] == b'#' {
                next_position = c.wrapping_sub(1);
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(14, 106517, 79723);

    #[test]
    fn test_base() {
        let example = "O....#....\n\
                             O.OO#....#\n\
                             .....##...\n\
                             OO.#O....O\n\
                             .O.....O#.\n\
                             O.#..O.#.#\n\
                             ..O..#O..O\n\
                             .......O..\n\
                             #....###..\n\
                             #OO..#....";
        assert_eq!(part1(&generator(example)), 136);
        assert_eq!(part2(&generator(example)), 64);
    }
}
