use std::str::from_utf8;

type Int = u64;

#[derive(PartialEq, Eq, PartialOrd, Ord)]
pub struct Map {
    source: Int,
    dest: Int,
    len: Int,
}

impl FromIterator<Int> for Map {
    fn from_iter<T: IntoIterator<Item = Int>>(iter: T) -> Self {
        let mut iter = iter.into_iter();
        Map {
            dest: iter.next().unwrap(),
            source: iter.next().unwrap(),
            len: iter.next().unwrap(),
        }
    }
}

type Input = (Vec<Int>, Vec<Vec<Map>>);

pub fn generator(input: &[u8]) -> Input {
    let input = from_utf8(input).unwrap();
    let mut blocks = input.split("\n\n");
    let seeds = blocks
        .next()
        .unwrap()
        .split(' ')
        .skip(1)
        .map(|n| n.parse().unwrap())
        .collect();

    let maps = blocks
        .map(|block| {
            let mut maps = block
                .lines()
                .skip(1)
                .map(|line| line.split(' ').map(|v| v.parse().unwrap()).collect())
                .collect::<Vec<Map>>();
            maps.sort();
            maps
        })
        .collect();
    (seeds, maps)
}

pub fn part1(input: &Input) -> Int {
    let (seeds, maps) = input;

    let mut l = Int::MAX;
    for seed in seeds {
        let mut v = *seed;

        'outer: for map in maps {
            for range in map.iter() {
                if v >= range.source && v < range.source + range.len {
                    v = range.dest + (v - range.source);
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

pub fn part2(input: &Input) -> Int {
    let (seeds, maps) = input;

    let mut min = Int::MAX;
    for seed in seeds.chunks(2) {
        let seed_min = seed[0];
        let seed_max = seed[0] + seed[1];
        let mut l = Int::MAX;
        let mut next_seed = seed_min;

        while next_seed < seed_max {
            let mut v = next_seed;
            let mut jump = Int::MAX;
            'outer: for map in maps {
                // range are sorted
                for range in map.iter() {
                    if v >= range.source && v < range.source + range.len {
                        jump = jump.min(range.source + range.len - v);
                        v = range.dest + (v - range.source);
                        continue 'outer;
                    }
                    if v < range.source {
                        // then v is outside of all ranges
                        jump = jump.min(range.source - v);
                        continue 'outer;
                    }
                }
            }
            next_seed += jump;
            l = l.min(v);
        }
        min = min.min(l);
    }
    min
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(5, 836040384, 10834440);

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
            56 93 4"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 35);
        assert_eq!(part2(&generator(example)), 46);
    }
}
