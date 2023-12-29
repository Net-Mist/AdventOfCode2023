use ahash::{HashMap, HashMapExt};

use rustworkx_core::petgraph::graphmap::UnGraphMap;

use rustworkx_core::connectivity::stoer_wagner_min_cut;

type Input = Vec<(usize, usize)>;
// type Input<'a> = Vec<(&'a str, &'a str)>;

pub fn generator(input: &[u8]) -> Input {
    let mut next_id = 0usize;
    let mut name_to_id = HashMap::new();
    let mut edges = vec![];
    for line in input[0..input.len() - 1].split(|b| b == &b'\n') {
        let names = line
            .split(|b| !b.is_ascii_lowercase())
            .filter(|b| !b.is_empty())
            .map(|b| {
                name_to_id
                    .entry(b)
                    .or_insert_with(|| {
                        next_id += 1;
                        next_id - 1
                    })
                    .to_owned()
            })
            .collect::<Vec<_>>();
        let first = names.first().unwrap();
        for other in names[1..].iter() {
            edges.push((*first, *other));
        }
    }
    edges
}

pub fn part1(input: &Input) -> usize {
    let g = UnGraphMap::<usize, ()>::from_edges(input);
    let parts = stoer_wagner_min_cut(&g, |_| Ok::<_, ()>(1))
        .unwrap()
        .unwrap()
        .1
        .len();
    parts * (g.node_count() - parts)
}

pub fn part2(_input: &Input) -> usize {
    1
}

#[cfg(test)]
mod tests {
    use super::*;

    // use aoc_macro::test_parts;
    // test_parts!(2, 1698735, 1594785890);

    #[test]
    fn test_base() {
        let example = "jqt: rhn xhk nvd\n\
                                rsh: frs pzl lsr\n\
                                xhk: hfx\n\
                                cmg: qnr nvd lhk bvb\n\
                                rhn: xhk bvb hfx\n\
                                bvb: xhk hfx\n\
                                pzl: lsr hfx nvd\n\
                                qnr: nvd\n\
                                ntq: jqt hfx bvb xhk\n\
                                nvd: lhk\n\
                                lsr: lhk\n\
                                rzs: qnr cmg lsr rsh\n\
                                frs: qnr lhk lsr\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 54);
    }
}
