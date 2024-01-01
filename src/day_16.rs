use std::str::from_utf8;

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

fn direction_to_index(direction: &(i32, i32)) -> usize {
    match direction {
        (-1, 0) => 0,
        (1, 0) => 1,
        (0, -1) => 2,
        (0, 1) => 3,
        _ => unreachable!(),
    }
}

fn find_n_energized(
    initial_position: (usize, usize),
    initial_direction: (i32, i32),
    input: &[&[u8]],
) -> usize {
    // let mut to_process = vec![(initial_position, initial_direction)];
    let mut to_process = Vec::with_capacity(50);
    to_process.push((initial_position, initial_direction));
    let h = input.len(); // 110
    let w = input[0].len(); // 110

    // keep track of energized cells
    let mut energized = vec![0u64; (w * h + 63) / 64];

    // keep track of seen direction for each energized cells
    let mut seen_direction = vec![0u64; (4 * w * h + 63) / 64];
    'main: while let Some((mut position, direction)) = to_process.pop() {
        if position.0 >= h || position.1 >= w {
            continue;
        }
        let seen_direction_id = (position.0 * w + position.1) * 4 + direction_to_index(&direction);
        let energized_id = position.0 * w + position.1;
        if seen_direction[seen_direction_id / 64] >> (seen_direction_id % 64) & 1 == 1 {
            continue;
        }
        seen_direction[seen_direction_id / 64] |= 1 << (seen_direction_id % 64);
        energized[energized_id / 64] |= 1 << (energized_id % 64);
        let mut t = input.get(position.0).unwrap().get(position.1).unwrap();

        let udirection = (direction.0 as usize, direction.1 as usize);
        while t == &b'.' {
            position.0 = position.0.wrapping_add(udirection.0);
            position.1 = position.1.wrapping_add(udirection.1);
            if position.0 >= h || position.1 >= w {
                continue 'main;
            }
            let energized_id = position.0 * w + position.1;
            energized[energized_id / 64] |= 1 << (energized_id % 64);
            t = input.get(position.0).unwrap().get(position.1).unwrap();
        }
        match direction {
            // right
            (0, 1) => match t {
                b'-' => {
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
                b'-' => {
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
                b'|' => {
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
                b'|' => {
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
    energized.into_iter().map(|b| b.count_ones() as usize).sum()
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
