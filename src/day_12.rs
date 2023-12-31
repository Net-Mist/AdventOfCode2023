// use std::{ops::AddAssign, simd::u64x2};

use std::str::from_utf8;

type Input = Vec<(u128, u128, Vec<u8>, u8)>;

pub fn generator(input: &[u8]) -> Input {
    let input = from_utf8(input).unwrap();

    input
        .lines()
        .map(|l| {
            let (map, groups) = l.split_once(' ').unwrap();
            let mut damaged = 0;
            let mut unknown = 0;

            for b in map.bytes() {
                damaged <<= 1;
                unknown <<= 1;
                match b {
                    b'.' => {}
                    b'?' => unknown |= 1,
                    b'#' => damaged |= 1,
                    _ => unreachable!(),
                }
            }
            let groups = groups.split(',').map(|v| v.parse().unwrap()).collect();
            (damaged, unknown, groups, map.len() as u8)
        })
        .collect()
}

pub fn part1(input: &Input) -> u64 {
    input
        .iter()
        .map(|(damaged, unknown, groups, n_bits)| {
            let mut groups = groups.to_owned();
            groups.reverse();
            let n_damaged = groups.len() as u8;
            let _cache = vec![u64::MAX; *n_bits as usize * n_damaged as usize];
            count_valid(*damaged, *unknown, &groups, *n_bits)
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    input
        .iter()
        .map(|(damaged, unknown, count, n_bits)| {
            let mut count = count.to_owned();
            count.reverse();
            let mut new_damaged = *damaged;
            let mut new_unknown = *unknown;
            for _ in 0..4 {
                new_unknown <<= 1;
                new_unknown += 1;
                new_unknown <<= n_bits;
                new_unknown += unknown;
                new_damaged <<= 1 + n_bits;
                new_damaged += damaged;
            }
            let new_count = count.repeat(5);

            count_valid(new_damaged, new_unknown, &new_count, n_bits * 5 + 4)
        })
        .sum()
}

fn count_valid(damaged: u128, unknown: u128, count: &[u8], max_n_bits: u8) -> u64 {
    let damaged_or_unknown = damaged | unknown;
    let mut s = 0u64;
    let mut previous_s;

    let mut cache_previous = vec![0u64; max_n_bits as usize + 1];
    cache_previous[0] = 1; // finding 0 damaged on a empty map is a success
    for i in 0..max_n_bits {
        if (damaged >> i) & 1 != 1 {
            cache_previous[i as usize + 1] = 1;
        } else {
            break;
        }
    }

    let mut cache_current = vec![0; max_n_bits as usize + 1];
    let mut min_bits: u8 = 0;
    let mut additionnal_jump = 0;
    unsafe {
        for groupe_size in count {
            previous_s = 0;
            for i in 0..groupe_size + additionnal_jump {
                *cache_current.get_unchecked_mut(i as usize + min_bits as usize) = 0;
            }

            min_bits += additionnal_jump + *groupe_size;
            for n_bits in min_bits..=max_n_bits {
                s = 0;

                // if not damaged, skip to the next one
                // let z = u64x2::from(damaged);

                let a = (damaged >> (n_bits - 1)) as u8;
                let s1 = previous_s * (a & 1 == 0) as u64;
                // s.add_assign()
                // s += previous_s * (damaged >> (n_bits - 1) & 1 == 0) as u64;
                // if damaged >> (n_bits - 1) & 1 == 0 {
                // s += previous_s;
                // }

                let group = (damaged_or_unknown >> (n_bits - groupe_size)) as u16;
                let group = group.wrapping_add(1);
                let validator = 0b1111111111111111 >> (16 - groupe_size);
                let is_full_1 = group & validator == 0;
                let is_not_followed_by_damaged =
                    (damaged << 1) & (1 << (n_bits - groupe_size)) == 0;
                // if is_full_1 && is_not_followed_by_damaged {
                s += cache_previous
                    .get_unchecked((n_bits - groupe_size - additionnal_jump) as usize)
                    * (is_full_1 && is_not_followed_by_damaged) as u64
                    + s1;
                // }

                *cache_current.get_unchecked_mut(n_bits as usize) = s;
                previous_s = s;
            }
            (cache_previous, cache_current) = (cache_current, cache_previous);
            additionnal_jump = 1;
        }
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(12, 7260, 1909291258644);

    #[test]
    fn test_base() {
        let example = "???.### 1,1,3".as_bytes();
        assert_eq!(part1(&generator(example)), 1);

        let example = ".??..??...?##. 1,1,3".as_bytes();
        assert_eq!(part1(&generator(example)), 4);

        let example = "?#?#?#?#?#?#?#? 1,3,1,6".as_bytes();
        assert_eq!(part1(&generator(example)), 1);

        let example = "????.#...#... 4,1,1".as_bytes();
        assert_eq!(part1(&generator(example)), 1);

        let example = "????.######..#####. 1,6,5".as_bytes();
        assert_eq!(part1(&generator(example)), 4);

        let example = "?###???????? 3,2,1".as_bytes();
        assert_eq!(part1(&generator(example)), 10);

        let example = "???.### 1,1,3\n\
                                .??..??...?##. 1,1,3\n\
                                ?#?#?#?#?#?#?#? 1,3,1,6\n\
                                ????.#...#... 4,1,1\n\
                                ????.######..#####. 1,6,5\n\
                                ?###???????? 3,2,1"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 21);
        assert_eq!(part2(&generator(example)), 525152);
    }
}
