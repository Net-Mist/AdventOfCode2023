use std::{collections::VecDeque, str::from_utf8};

use ahash::{HashSet, HashSetExt};

type Input = Vec<Node>;

#[derive(Debug)]
pub struct Node {
    supported_by: HashSet<u16>,
    support: HashSet<u16>,
}

pub fn generator(input: &[u8]) -> Input {
    // order bricks by increasing Z
    // Fill an array of size 10*10*360 (36K u16)
    // extract the graph
    let mut map = vec![vec![vec![0; 370]; 10]; 10];
    let mut blocks: Vec<Vec<u16>> = input[0..input.len() - 1]
        .split(|b| b == &b'\n')
        .map(|line| {
            line.split(|b| b == &b',' || b == &b'~')
                .map(|n| from_utf8(n).unwrap().parse::<u16>().unwrap())
                .collect()
        })
        .collect();
    blocks.sort_unstable_by(|a, b| a[2].partial_cmp(&b[2]).unwrap());

    let mut graph: Vec<Node> = Vec::with_capacity(blocks.len());

    for (block_id, block) in blocks.into_iter().enumerate() {
        let mut delta = 1usize;
        let mut supported_by = HashSet::with_capacity(10);
        while block[2] as usize > delta {
            for i in block[0]..=block[3] {
                for j in block[1]..=block[4] {
                    for k in block[2]..=block[5] {
                        let v = map[i as usize][j as usize][k as usize - delta];
                        if v != 0 {
                            supported_by.insert((v - 1) as u16);
                        }
                    }
                }
            }
            if !supported_by.is_empty() {
                break;
            }
            delta += 1;
        }

        for i in block[0]..=block[3] {
            for j in block[1]..=block[4] {
                for k in block[2]..=block[5] {
                    map[i as usize][j as usize][k as usize + 1 - delta] = block_id + 1;
                }
            }
        }
        for id in supported_by.iter() {
            graph[*id as usize].support.insert(block_id as u16);
        }
        graph.push(Node {
            supported_by,
            support: HashSet::new(),
        });
    }
    graph
}

pub fn part1(graph: &Input) -> usize {
    graph
        .iter()
        .filter(|node| {
            node.support
                .iter()
                .all(|node| graph[*node as usize].supported_by.len() != 1)
        })
        .count()
}

pub fn part2(input: &Input) -> u32 {
    let mut s = 0;
    let mut removed = VecDeque::with_capacity(30);
    for j in 0..input.len() {
        removed.clear();
        let mut removed_ids = [0u64; 23];

        removed.push_back(j as u16);
        removed_ids[j / 64] |= 1 << j % 64;
        while let Some(i) = removed.pop_front() {
            input[i as usize].support.iter().for_each(|node_id| {
                if input[*node_id as usize]
                    .supported_by
                    .iter()
                    .all(|i| (removed_ids[*i as usize / 64] >> i) & 1 == 1)
                {
                    removed_ids[(*node_id as usize) / 64] |= 1 << (*node_id) % 64;
                    removed.push_back(*node_id as u16);
                }
            });
        }

        s += removed_ids
            .iter()
            .map(|i| i.count_ones() as u32)
            .sum::<u32>()
            - 1;
    }
    s
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(22, 492, 86556);

    #[test]
    fn test_base() {
        let example = "1,0,1~1,2,1\n\
                                0,0,2~2,0,2\n\
                                0,2,3~2,2,3\n\
                                0,0,4~0,2,4\n\
                                2,0,5~2,2,5\n\
                                0,1,6~2,1,6\n\
                               1,1,8~1,1,9\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 5);
        assert_eq!(part2(&generator(example)), 7);
    }
}
