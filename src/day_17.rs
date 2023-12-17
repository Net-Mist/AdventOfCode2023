use std::{cmp::Ordering, collections::BinaryHeap};

use ahash::{HashSet, HashSetExt};

type Input<'a> = Vec<&'a [u8]>;

#[derive(PartialEq, Eq, PartialOrd, Ord, Debug, Hash, Clone, Copy)]
enum Direction {
    North,
    South,
    East,
    West,
    None,
}

#[derive(PartialEq, Eq, Debug, Ord)]
struct State {
    heat_loss: u64,
    line: usize,
    col: usize,
    direction: Direction,
    n_direction: u8,
}

impl PartialOrd for State {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.heat_loss.cmp(&other.heat_loss).reverse())
    }
}

pub fn generator(input: &str) -> Input {
    input.lines().map(|l| l.as_bytes()).collect()
}

pub fn part1(input: &Input) -> u64 {
    let h = input.len();
    let w = input[0].len();
    let mut queue = BinaryHeap::new();
    let mut seen = HashSet::new();
    queue.push(State {
        heat_loss: 0,
        line: 0,
        col: 0,
        direction: Direction::South,
        n_direction: 0,
    });
    while let Some(e) = queue.pop() {
        if seen.contains(&(e.line, e.col, e.direction, e.n_direction)) {
            continue;
        }
        if e.n_direction > 3 {
            continue;
        }
        if e.line == h - 1 && e.col == w - 1 {
            return e.heat_loss;
        }
        seen.insert((e.line, e.col, e.direction, e.n_direction));
        if e.col <= w - 2 && e.direction != Direction::East {
            queue.push(State {
                heat_loss: e.heat_loss + (input[e.line][e.col + 1] - b'0') as u64,
                line: e.line,
                col: e.col + 1,
                direction: Direction::West,
                n_direction: if e.direction == Direction::West {
                    e.n_direction + 1
                } else {
                    1
                },
            });
        }
        if e.col > 0 && e.direction != Direction::West {
            queue.push(State {
                heat_loss: e.heat_loss + (input[e.line][e.col.wrapping_sub(1)] - b'0') as u64,
                line: e.line,
                col: e.col.wrapping_sub(1),
                direction: Direction::East,
                n_direction: if e.direction == Direction::East {
                    e.n_direction + 1
                } else {
                    1
                },
            });
        }
        if e.line <= h - 2 && e.direction != Direction::North {
            queue.push(State {
                heat_loss: e.heat_loss + (input[e.line + 1][e.col] - b'0') as u64,
                line: e.line + 1,
                col: e.col,
                direction: Direction::South,
                n_direction: if e.direction == Direction::South {
                    e.n_direction + 1
                } else {
                    1
                },
            });
        }
        if e.line > 0 && e.direction != Direction::South {
            queue.push(State {
                heat_loss: e.heat_loss + (input[e.line.wrapping_sub(1)][e.col] - b'0') as u64,
                line: e.line.wrapping_sub(1),
                col: e.col,
                direction: Direction::North,
                n_direction: if e.direction == Direction::North {
                    e.n_direction + 1
                } else {
                    1
                },
            });
        }
    }

    return 0;
}

pub fn part2(input: &Input) -> u64 {
    let h = input.len();
    let w = input[0].len();
    let mut queue = BinaryHeap::with_capacity(141 * 141 * 4);
    let mut seen = HashSet::with_capacity(141 * 141 * 4);
    queue.push(State {
        heat_loss: 0,
        line: 0,
        col: 0,
        direction: Direction::None,
        n_direction: 0,
    });
    while let Some(e) = queue.pop() {
        if seen.contains(&(e.line, e.col, e.direction)) {
            continue;
        }
        if e.line == h - 1 && e.col == w - 1 {
            return e.heat_loss;
        }
        seen.insert((e.line, e.col, e.direction));
        if e.col <= w - 5 && e.direction != Direction::East && e.direction != Direction::West {
            let mut add_heat: u64 = (1..4)
                .map(|i| (input[e.line][e.col + i] - b'0') as u64)
                .sum();

            for (i, c) in ((e.col + 4)..(e.col + 11).min(w)).enumerate() {
                add_heat += (input[e.line][c] - b'0') as u64;
                queue.push(State {
                    heat_loss: e.heat_loss + add_heat,
                    line: e.line,
                    col: c,
                    direction: Direction::East,
                    n_direction: i as u8 + 4,
                });
            }
        }
        if e.col > 3 && e.direction != Direction::East && e.direction != Direction::West {
            let mut add_heat: u64 = (1..4)
                .map(|i| (input[e.line][e.col - i] - b'0') as u64)
                .sum();

            for (i, c) in (e.col.saturating_sub(10)..=(e.col - 4)).rev().enumerate() {
                add_heat += (input[e.line][c] - b'0') as u64;
                queue.push(State {
                    heat_loss: e.heat_loss + add_heat,
                    line: e.line,
                    col: c,
                    direction: Direction::West,
                    n_direction: i as u8 + 4,
                });
            }
        }
        if e.line <= h - 5 && e.direction != Direction::North && e.direction != Direction::South {
            let mut add_heat: u64 = (1..4)
                .map(|i| (input[e.line + i][e.col] - b'0') as u64)
                .sum();

            for (i, c) in ((e.line + 4)..(e.line + 11).min(h)).enumerate() {
                add_heat += (input[c][e.col] - b'0') as u64;
                queue.push(State {
                    heat_loss: e.heat_loss + add_heat,
                    line: c,
                    col: e.col,
                    direction: Direction::South,
                    n_direction: i as u8 + 4,
                });
            }
        }
        if e.line > 3 && e.direction != Direction::North && e.direction != Direction::South {
            let mut add_heat: u64 = (1..4)
                .map(|i| (input[e.line - i][e.col] - b'0') as u64)
                .sum();

            for (i, c) in (e.line.saturating_sub(10)..=(e.line - 4)).rev().enumerate() {
                add_heat += (input[c][e.col] - b'0') as u64;
                queue.push(State {
                    heat_loss: e.heat_loss + add_heat,
                    line: c,
                    col: e.col,
                    direction: Direction::North,
                    n_direction: i as u8 + 4,
                });
            }
        }
    }

    return 0;
}
#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(17, 817, 925);

    #[test]
    fn test_base() {
        let example = "2413432311323\n\
                             3215453535623\n\
                             3255245654254\n\
                             3446585845452\n\
                             4546657867536\n\
                             1438598798454\n\
                             4457876987766\n\
                             3637877979653\n\
                             4654967986887\n\
                             4564679986453\n\
                             1224686865563\n\
                             2546548887735\n\
                             4322674655533\n";
        assert_eq!(part1(&generator(example)), 102);
        assert_eq!(part2(&generator(example)), 94);
    }
}
