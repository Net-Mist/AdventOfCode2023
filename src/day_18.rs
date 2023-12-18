use std::str::from_utf8;

use ahash::{HashSet, HashSetExt};

type Input<'a> = Vec<(&'a u8, u8, &'a [u8])>;

pub fn generator(input: &[u8]) -> Input {
    input[0..input.len() - 1]
        .split(|b| b == &b'\n')
        .map(|line| {
            let direction = line.get(0).unwrap();
            let mut iter = line[2..].splitn(2, |b| b == &b' ');
            let n = from_utf8(iter.next().unwrap()).unwrap().parse().unwrap();
            let color = iter.next().unwrap();
            (direction, n, color)
        })
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut min_x = 0;
    let mut min_y = 0;
    let mut max_x = 0;
    let mut max_y = 0;

    let mut map = HashSet::new();
    map.insert((0, 0));
    let mut current_position = (0, 0);

    for (direction, n, color) in input {
        let direction = match direction {
            b'U' => (0, -1),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            _ => unreachable!(),
        };
        for _ in 0..*n {
            current_position.0 += direction.0;
            current_position.1 += direction.1;
            map.insert(current_position);
            min_x = min_x.min(current_position.0);
            max_x = max_x.max(current_position.0);
            min_y = min_y.min(current_position.1);
            max_y = max_y.max(current_position.1);
        }
    }
    dbg!(min_x);
    dbg!(min_y);
    dbg!(max_x);
    dbg!(max_y);
    // count
    // wall are never touching themselves
    let mut c = 0;
    let mut from_bellow = false;
    let mut from_above = false;
    let mut on_wall = false;
    for y in min_y..=max_y {
        let mut inside = false;
        from_above = false;
        from_bellow = false;
        on_wall = false;
        dbg!(y);
        dbg!(c);
        for x in min_x..=max_x {
            if map.contains(&(x, y)) {
                c += 1;
                if !on_wall {
                    if map.contains(&(x, y - 1)) {
                        from_above = true;
                    }
                    if map.contains(&(x, y + 1)) {
                        from_bellow = true;
                    }
                }
                on_wall = true;
            } else if on_wall {
                // we are leaving a wall section
                on_wall = false;
                if from_above && from_bellow {
                    inside = !inside;
                } else if from_above && map.contains(&(x - 1, y + 1)) {
                    inside = !inside;
                } else if from_bellow && map.contains(&(x - 1, y - 1)) {
                    inside = !inside;
                }
                from_above = false;
                from_bellow = false;
                if inside {
                    c += 1
                }
            } else if inside {
                c += 1;
            }
        }
    }
    c
}

pub fn part2(input: &Input) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

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
        assert_eq!(part1(&generator(example)), 1);
        assert_eq!(part2(&generator(example)), 1);
    }
}
