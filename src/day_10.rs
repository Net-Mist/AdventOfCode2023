use std::{fmt::Display, str::from_utf8};

use ahash::HashSet;
use arrayvec::ArrayVec;

const MAP_BORDER: usize = 140;

// | is a vertical pipe connecting north and south.
// - is a horizontal pipe connecting east and west.
// L is a 90-degree bend connecting north and east.
// J is a 90-degree bend connecting north and west.
// 7 is a 90-degree bend connecting south and west.
// F is a 90-degree bend connecting south and east.
#[derive(Clone, Copy, Debug, PartialEq)]
pub enum Pipe {
    Vertical,
    Horizontal,
    NorthEast,
    NorthWest,
    SouthWest,
    SouthEast,
    None,
}

impl Display for Pipe {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let e = match self {
            Pipe::Vertical => '│',
            Pipe::Horizontal => '─',
            Pipe::NorthEast => '└',
            Pipe::NorthWest => '┘',
            Pipe::SouthWest => '┐',
            Pipe::SouthEast => '┌',
            Pipe::None => '*',
        };
        write!(f, "{}", e)
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Origin {
    North,
    South,
    East,
    West,
}

impl From<u8> for Pipe {
    fn from(value: u8) -> Self {
        match value {
            b'|' => Pipe::Vertical,
            b'-' => Pipe::Horizontal,
            b'L' => Pipe::NorthEast,
            b'J' => Pipe::NorthWest,
            b'7' => Pipe::SouthWest,
            b'F' => Pipe::SouthEast,
            b'S' => Pipe::Vertical,
            _ => Pipe::None,
        }
    }
}

type Map = ArrayVec<ArrayVec<Pipe, MAP_BORDER>, MAP_BORDER>;

type Type = ((u8, u8), Map);

pub fn generator(input: &[u8]) -> Type {
    let input = from_utf8(input).unwrap();

    let map = input
        .lines()
        .map(|l| l.bytes().map(Pipe::from).collect())
        .collect();
    let (i, start_line) = input
        .lines()
        .enumerate()
        .find(|(_, l)| l.contains('S'))
        .unwrap();
    let j = start_line
        .bytes()
        .enumerate()
        .find(|(_, e)| *e == b'S')
        .unwrap()
        .0;
    ((i as u8, j as u8), map)
}

pub fn part1(input: &Type) -> usize {
    let (position_start, map) = input;
    let l = compute_loop(position_start, map);
    l.len() / 2
}

fn compute_loop(position_start: &(u8, u8), map: &Map) -> ArrayVec<(u8, u8), 15000> {
    let mut l = ArrayVec::new();
    let mut position = position_start.to_owned();
    let mut origin = Origin::North;
    loop {
        match (map[position.0 as usize][position.1 as usize], origin) {
            (Pipe::Vertical, Origin::North) => position.0 += 1,
            (Pipe::Vertical, Origin::South) => position.0 -= 1,
            (Pipe::Horizontal, Origin::East) => position.1 -= 1,
            (Pipe::Horizontal, Origin::West) => position.1 += 1,
            (Pipe::NorthEast, Origin::North) => {
                position.1 += 1;
                origin = Origin::West
            }
            (Pipe::NorthEast, Origin::East) => {
                position.0 -= 1;
                origin = Origin::South
            }
            (Pipe::NorthWest, Origin::North) => {
                position.1 -= 1;
                origin = Origin::East;
            }
            (Pipe::NorthWest, Origin::West) => {
                position.0 -= 1;
                origin = Origin::South
            }
            (Pipe::SouthWest, Origin::South) => {
                position.1 -= 1;
                origin = Origin::East
            }
            (Pipe::SouthWest, Origin::West) => {
                position.0 += 1;
                origin = Origin::North
            }
            (Pipe::SouthEast, Origin::South) => {
                position.1 += 1;
                origin = Origin::West
            }
            (Pipe::SouthEast, Origin::East) => {
                position.0 += 1;
                origin = Origin::North
            }
            _ => unreachable!(),
        }
        l.push(position.to_owned());

        if position == *position_start {
            break;
        }
    }
    l
}

pub fn part2(input: &Type) -> usize {
    let (position_start, map) = input;
    let loop_pipe = compute_loop(position_start, map)
        .into_iter()
        .collect::<HashSet<_>>();

    // create a new map with only the interesting pipe
    let mut new_map = map.to_owned();
    for (i, l) in map.iter().enumerate() {
        for (j, _) in l.iter().enumerate() {
            if !loop_pipe.contains(&(i as u8, j as u8)) {
                new_map[i][j] = Pipe::None
            }
        }
    }

    let mut n_inner = 0;
    let mut is_inner = false;
    let mut from_north = false;
    let mut from_south = false;
    for l in new_map.iter() {
        for e in l.iter() {
            match e {
                Pipe::Vertical => is_inner = !is_inner,
                Pipe::Horizontal => {}
                Pipe::NorthEast => from_north = true,
                Pipe::NorthWest => {
                    if from_south {
                        is_inner = !is_inner
                    }
                    from_south = false;
                    from_north = false;
                }
                Pipe::SouthWest => {
                    if from_north {
                        is_inner = !is_inner
                    }
                    from_south = false;
                    from_north = false;
                }
                Pipe::SouthEast => from_south = true,
                Pipe::None => {
                    if is_inner {
                        n_inner += 1;
                    }
                }
            }
        }
    }

    n_inner
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(10, 6897, 367);

    #[test]
    fn test_base() {
        let example = ".....\n\
                             .F-7.\n\
                             .S.|.\n\
                             .L-J.\n\
                             ....."
            .as_bytes();
        assert_eq!(part1(&generator(example)), 4);

        let example = "..........\n\
                            .F------7.\n\
                            .|F----7|.\n\
                            .S|....||.\n\
                            .||....||.\n\
                            .|L-7F-J|.\n\
                            .|..||..|.\n\
                            .L--JL--J.\n\
                            .........."
            .as_bytes();
        assert_eq!(part2(&generator(example)), 4);
    }
}
