use std::str::from_utf8;

use arrayvec::ArrayVec;

type Input<'a> = ArrayVec<ArrayVec<&'a [u8], 17>, 100>;

pub fn generator(input: &[u8]) -> Input {
    let input = from_utf8(input).unwrap();

    let block = input.split("\n\n");
    block
        .map(|b| b.lines().map(|line| line.as_bytes()).collect())
        .collect()
}

pub fn part1(input: &Input) -> usize {
    let mut s = 0;
    'block: for block in input {
        let block_h = block.len();
        let block_w = block[0].len();
        // test line reflection
        for i in 0..block_h - 1 {
            if (0..=i.min(block_h - i - 2))
                .all(|j| (0..block_w).all(|k| block[i - j][k] == block[i + 1 + j][k]))
            {
                s += (i + 1) * 100;
                continue 'block;
            }
        }

        // test column reflection
        for i in 0..block_w - 1 {
            if (0..=i.min(block_w - i - 2))
                .all(|j| (0..block_h).all(|k| block[k][i - j] == block[k][i + 1 + j]))
            {
                s += i + 1;
                continue 'block;
            }
        }
    }
    s
}

pub fn part2(input: &Input) -> usize {
    let mut s = 0;
    'block: for block in input {
        let block_h = block.len();
        let block_w = block[0].len();
        // test line reflection
        for i in 0..block_h - 1 {
            if (0..=i.min(block_h - i - 2))
                .map(|j| {
                    (0..block_w)
                        .map(|k| (block[i - j][k] != block[i + 1 + j][k]) as usize)
                        .sum::<usize>()
                })
                .sum::<usize>()
                == 1
            {
                s += (i + 1) * 100;
                continue 'block;
            }
        }

        // test column reflection
        for i in 0..block_w - 1 {
            if (0..=i.min(block_w - i - 2))
                .map(|j| {
                    (0..block_h)
                        .map(|k| (block[k][i - j] != block[k][i + 1 + j]) as usize)
                        .sum::<usize>()
                })
                .sum::<usize>()
                == 1
            {
                s += i + 1;
                continue 'block;
            }
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(13, 33122, 32312);

    #[test]
    fn test_base() {
        let example = "#.##..##.\n\
                            ..#.##.#.\n\
                            ##......#\n\
                            ##......#\n\
                            ..#.##.#.\n\
                            ..##..##.\n\
                            #.#.##.#.\n\
                            \n\
                            #...##..#\n\
                            #....#..#\n\
                            ..##..###\n\
                            #####.##.\n\
                            #####.##.\n\
                            ..##..###\n\
                            #....#..#"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 405);
        assert_eq!(part2(&generator(example)), 400);
    }
}
