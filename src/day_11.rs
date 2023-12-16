use std::str::from_utf8;

use arrayvec::ArrayVec;

type Int = u32;
type Input = ArrayVec<(Int, Int), 443>;

pub fn generator(input: &[u8]) -> Input {
    let input = from_utf8(input).unwrap();

    input
        .lines()
        .enumerate()
        .flat_map(|(i, line)| {
            line.bytes()
                .enumerate()
                .filter(|(_, b)| *b == b'#')
                .map(move |(j, _)| (i.to_owned() as Int, j as Int))
        })
        .collect()
}

fn dilate(mut positions: Input, f: Int) -> Input {
    //dilate y
    let mut d = 0;
    let mut last_y = 0;
    for i in 0..positions.len() {
        if positions[i].0 - last_y > 1 {
            d += (positions[i].0 - last_y - 1) * f;
        }
        last_y = positions[i].0;
        positions[i].0 += d;
    }

    // dilate x
    d = 0;
    let mut last_x = 0;
    positions.sort_unstable_by(|a, b| a.1.cmp(&b.1));
    for i in 0..positions.len() {
        if positions[i].1 - last_x > 1 {
            d += (positions[i].1 - last_x - 1) * f;
        }
        last_x = positions[i].1;
        positions[i].1 += d;
    }
    positions
}

pub fn part1(input: &Input) -> u64 {
    let input = dilate(input.to_owned(), 1);
    let mut s = 0;
    for (i, a) in input.iter().enumerate() {
        for b in input[i + 1..].iter() {
            s += ((b.0).abs_diff(a.0) + b.1.abs_diff(a.1)) as u64;
        }
    }
    s
}

pub fn part2(input: &Input) -> u64 {
    let input = dilate(input.to_owned(), 999999);
    let mut s = 0;
    for (i, a) in input.iter().enumerate() {
        for b in input[i + 1..].iter() {
            s += ((b.0).abs_diff(a.0) + b.1.abs_diff(a.1)) as u64;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;
    use itertools::iproduct;

    use aoc_macro::test_parts;
    test_parts!(11, 9684228, 483844716556);

    #[test]
    fn test_base() {
        let example = "...#......\n\
                            .......#..\n\
                            #.........\n\
                            ..........\n\
                            ......#...\n\
                            .#........\n\
                            .........#\n\
                            ..........\n\
                            .......#..\n\
                            #...#....."
            .as_bytes();
        assert_eq!(part1(&generator(example)), 374);

        let input = dilate(generator(example).to_owned(), 9);
        let s: u32 = iproduct!(&input, &input)
            .map(|(a, b)| ((b.0).abs_diff(a.0) + b.1.abs_diff(a.1)))
            .sum();
        let s = s >> 1;

        assert_eq!(s, 1030);
    }
}
