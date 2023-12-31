type Input = (Vec<Vec<bool>>, (u8, u8));

const N_STEPS: usize = 26501365;

pub fn generator(input: &[u8]) -> Input {
    let mut init_position = (0, 0);
    (
        input[0..input.len() - 1]
            .split(|b| b == &b'\n')
            .enumerate()
            .map(|(i, line)| {
                line.iter()
                    .enumerate()
                    .map(|(j, b)| match b {
                        b'#' => false,
                        b'.' => true,
                        b'S' => {
                            init_position = (i as u8, j as u8);
                            true
                        }
                        _ => unreachable!(),
                    })
                    .collect()
            })
            .collect(),
        init_position,
    )
}

pub fn part1(input: &Input) -> usize {
    let (map, position) = input;
    let mut map = map.clone();
    let h = map.len() as u8;
    let w = map[0].len() as u8;
    let mut previous_positions = Vec::with_capacity(h as usize * w as usize);
    let mut new_positions = Vec::with_capacity(h as usize * w as usize);

    previous_positions.push(position.to_owned());
    let directions = [(1, 0), (0, 1), (255, 0), (0, 255)];
    let mut previous_n = 0;
    let mut previous_previous_n;
    let mut n = 0;

    for _ in 0..64 {
        for position in previous_positions.iter() {
            for direction in directions {
                let new_position = (
                    position.0.wrapping_add(direction.0),
                    position.1.wrapping_add(direction.1),
                );
                if new_position.0 >= h || new_position.1 >= w {
                    continue;
                }
                if map[new_position.0 as usize][new_position.1 as usize] {
                    new_positions.push(new_position);
                    map[new_position.0 as usize][new_position.1 as usize] = false;
                }
            }
        }
        (previous_positions, new_positions) = (new_positions, previous_positions);
        new_positions.clear();
        previous_previous_n = previous_n;
        previous_n = n;
        n = previous_positions.len() + previous_previous_n;
    }
    n
}

pub fn part2(input: &Input) -> usize {
    let (map, _position) = input;
    let h = map.len() as u8;
    let w = map[0].len() as u8;
    let mut n = 0;

    // count center
    let l = compute_n_position_until_cycle(map, &((h - 1) / 2, (w - 1) / 2));
    if N_STEPS % 2 != l.len() % 2 {
        n += l[l.len() - 1];
    } else {
        n += l[l.len() - 2];
    }

    // add maps alligned with axis
    n += [
        (h - 1, (w - 1) / 2),
        (0, (w - 1) / 2),
        ((h - 1) / 2, 0),
        ((h - 1) / 2, (w - 1)),
    ]
    .iter()
    .map(|position| {
        let mut n = 0;
        let l = compute_n_position_until_cycle(map, position);
        for i in 0..=((N_STEPS - h as usize / 2 - 1) / h as usize) {
            let n_steps = N_STEPS - (h as usize / 2 + 1 + i * h as usize);
            if n_steps < l.len() {
                n += l[n_steps];
            } else if n_steps % 2 != l.len() % 2 {
                n += l[l.len() - 1];
            } else {
                n += l[l.len() - 2]
            }
        }
        n
    })
    .sum::<usize>();

    // add diagonals
    let init_positions = [(0, 0), (h - 1, 0), (0, w - 1), (h - 1, w - 1)];
    n += init_positions
        .iter()
        .map(|position| {
            let mut n = 0;
            let l = compute_n_position_until_cycle(map, position);
            for i in 0..=((N_STEPS - h as usize / 2 - w as usize / 2 - 2) / h as usize) {
                let n_steps = h as usize / 2 + w as usize / 2 + 2 + i * h as usize;
                let n_steps = N_STEPS - n_steps;
                if n_steps < l.len() {
                    n += l[n_steps] * (i + 1);
                } else if n_steps % 2 != l.len() % 2 {
                    n += l[l.len() - 1] * (i + 1);
                } else {
                    n += l[l.len() - 2] * (i + 1)
                }
            }
            n
        })
        .sum::<usize>();
    n
}

fn compute_n_position_until_cycle(map: &Vec<Vec<bool>>, init_position: &(u8, u8)) -> Vec<usize> {
    let mut map = map.clone();
    let h = map.len() as u8;
    let w = map[0].len() as u8;
    let mut previous_positions = Vec::with_capacity(h as usize * w as usize);
    let mut new_positions = Vec::with_capacity(h as usize * w as usize);

    previous_positions.push(init_position.to_owned());
    let directions = [(1, 0), (0, 1), (255, 0), (0, 255)];

    let mut previous_n = 0;
    let mut previous_previous_n;
    let mut n = 0;
    let mut ns = vec![1];
    for _ in 0..N_STEPS {
        for position in previous_positions.iter() {
            for direction in directions {
                let new_position = (
                    position.0.wrapping_add(direction.0),
                    position.1.wrapping_add(direction.1),
                );
                if new_position.0 >= h || new_position.1 >= w {
                    continue;
                }
                if map[new_position.0 as usize][new_position.1 as usize] {
                    map[new_position.0 as usize][new_position.1 as usize] = false;
                    new_positions.push(new_position);
                }
            }
        }
        (previous_positions, new_positions) = (new_positions, previous_positions);
        new_positions.clear();
        previous_previous_n = previous_n;
        previous_n = n;
        n = previous_positions.len() + previous_previous_n;
        ns.push(n);
        if previous_previous_n == n {
            break;
        }
    }
    ns
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(21, 3699, 613391294577878);

    #[test]
    fn test_base() {
        let example = "...........\n\
                            .....###.#.\n\
                            .###.##..#.\n\
                            ..#.#...#..\n\
                            ....#.#....\n\
                            .##..S####.\n\
                            .##..#...#.\n\
                            .......##..\n\
                            .##.#.####.\n\
                            .##..##.##.\n\
                            ...........\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 42);
    }
}
