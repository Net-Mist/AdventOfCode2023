const N_GROUPS: usize = 10 * 5;
const N_BITS: usize = 20 * 5 + 5;
const CACHE_SIZE: usize = N_GROUPS * N_BITS;
type Input = Vec<(u128, u128, Vec<u8>, u8)>;

pub fn generator(input: &str) -> Input {
    input
        .lines()
        .map(|l| {
            let (map, groups) = l.split_once(' ').unwrap();
            let mut damaged = 0u128;
            let mut unknown = 0u128;

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

pub fn part1(input: &Input) -> usize {
    input
        .iter()
        .map(|(damaged, unknown, groups, n_bits)| {
            let mut groups = groups.to_owned();
            groups.reverse();
            let n_damaged = groups.iter().sum();
            let mut cache = [usize::MAX; CACHE_SIZE];
            count_valid(*damaged, *unknown, &groups, &mut cache, *n_bits, n_damaged)
        })
        .sum()
}

pub fn part2(input: &Input) -> usize {
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
            let n_damaged = new_count.iter().sum();
            let mut cache = [usize::MAX; 10400];

            count_valid(
                new_damaged,
                new_unknown,
                &count.repeat(5),
                &mut cache,
                *n_bits * 5 + 4,
                n_damaged,
            )
        })
        .sum()
}

fn count_valid(
    damaged: u128,
    unknown: u128,
    count: &[u8],
    cache_matrix: &mut [usize],
    n_bits: u8,
    n_damaged: u8,
) -> usize {
    let cache_key = n_damaged as usize * N_BITS + n_bits as usize;
    if cache_matrix[cache_key] != usize::MAX {
        return cache_matrix[cache_key];
    }
    let mut s = 0;
    if count.is_empty() {
        if damaged != 0 {
            return 0;
        }
        return 1;
    }
    if damaged == 0 && unknown == 0 {
        if count.iter().any(|v| *v != 0) {
            return 0;
        }
        return 1;
    }

    if damaged & 1 == 0 {
        s += count_valid(
            damaged >> 1,
            unknown >> 1,
            count,
            cache_matrix,
            n_bits - 1,
            n_damaged,
        );
    }

    let i = count[0];
    if ((damaged + unknown + 1) & ((1 << i) - 1) == 0) && (damaged & (1 << (i)) == 0) {
        s += count_valid(
            damaged >> (i + 1),
            unknown >> (i + 1),
            &count[1..],
            cache_matrix,
            n_bits.saturating_sub(i + 1),
            n_damaged - i,
        );
    }
    cache_matrix[cache_key] = s;
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(12, 7260, 1909291258644);

    #[test]
    fn test_base() {
        let example = "???.### 1,1,3\n\
                                .??..??...?##. 1,1,3\n\
                                ?#?#?#?#?#?#?#? 1,3,1,6\n\
                                ????.#...#... 4,1,1\n\
                                ????.######..#####. 1,6,5\n\
                                ?###???????? 3,2,1";
        assert_eq!(part1(&generator(example)), 21);
        assert_eq!(part2(&generator(example)), 525152);
    }
}
