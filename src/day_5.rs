use rayon::prelude::*;

pub fn generator(input: &str) -> (Vec<u64>, Vec<Vec<Vec<u64>>>) {
    let mut blocks = input.split("\n\n");
    let seeds = blocks
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect::<Vec<u64>>();

    let maps = blocks
        .map(|block| {
            block
                .lines()
                .skip(1)
                .map(|line| line.split(' ').map(|v| v.parse().unwrap()).collect())
                .collect()
        })
        .collect();
    (seeds, maps)
}

pub fn part1(input: &(Vec<u64>, Vec<Vec<Vec<u64>>>)) -> u64 {
    let (seeds, maps) = input;

    let mut l = u64::MAX;
    for seed in seeds {
        let mut v = *seed;

        'outer: for map in maps {
            for range in map.iter() {
                if v >= range[1] && v < range[1] + range[2] {
                    v = range[0] + (v - range[1]);
                    continue 'outer;
                }
            }
        }
        if v < l {
            l = v;
        }
    }
    l
}

pub fn part2(input: &(Vec<u64>, Vec<Vec<Vec<u64>>>)) -> u64 {
    let (seeds, maps) = input;

    seeds
        .par_chunks(2)
        .map(|seed| {
            let mut l = u64::MAX;
            for mut v in seed[0]..seed[0] + seed[1] {
                'outer: for map in maps {
                    for range in map.iter() {
                        if v >= range[1] && v < range[1] + range[2] {
                            v = range[0] + (v - range[1]);
                            continue 'outer;
                        }
                    }
                }
                if v < l {
                    l = v;
                }
            }
            l
        })
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    // use aoc_macro::test_parts;
    // test_parts!(5, 836040384, 10834440);

    #[test]
    fn test_base() {
        let example = "seeds: 79 14 55 13\n\
            \n\
            seed-to-soil map:\n\
            50 98 2\n\
            52 50 48\n\
            \n\
            soil-to-fertilizer map:\n\
            0 15 37\n\
            37 52 2\n\
            39 0 15\n\
            \n\
            fertilizer-to-water map:\n\
            49 53 8\n\
            0 11 42\n\
            42 0 7\n\
            57 7 4\n\
            \n\
            water-to-light map:\n\
            88 18 7\n\
            18 25 70\n\
            \n\
            light-to-temperature map:\n\
            45 77 23\n\
            81 45 19\n\
            68 64 13\n\
            \n\
            temperature-to-humidity map:\n\
            0 69 1\n\
            1 0 69\n\
            \n\
            humidity-to-location map:\n\
            60 56 37\n\
            56 93 4";
        assert_eq!(part1(&generator(example)), 35);
        assert_eq!(part2(&generator(example)), 46);
    }
}
