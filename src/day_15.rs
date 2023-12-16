use std::str::from_utf8;

type Type<'a> = Vec<&'a str>;

pub fn generator(input: &[u8]) -> Type {
    let input = from_utf8(input).unwrap();

    input.lines().next().unwrap().split(',').collect()
}

fn compute_hash(w: &str) -> u8 {
    let mut hash = 0u8;
    for b in w.bytes() {
        hash = hash.wrapping_add(b);
        hash = hash.wrapping_mul(17);
    }
    hash
}

pub fn part1(input: &Type) -> u32 {
    input.iter().map(|w| compute_hash(w) as u32).sum()
}

#[derive(Clone, Copy, Debug)]
struct Element<'a> {
    label: &'a str,
    value: u16,
}

pub fn part2(input: &Type) -> u32 {
    let mut hashmap: Vec<Vec<Element>> = vec![vec![]; 256];
    'outer: for w in input {
        if let Some((label, value)) = w.split_once('=') {
            let hash = compute_hash(label);
            let value = value.parse().unwrap();

            let v = hashmap.get_mut(hash as usize).unwrap();
            for element in v.iter_mut() {
                if element.label == label {
                    element.value = value;
                    continue 'outer;
                }
            }
            v.push(Element { label, value });
        } else {
            let label = &w[0..w.len() - 1];
            let hash = compute_hash(label);
            let mut id = None;
            for (i, e) in hashmap[hash as usize].iter().enumerate() {
                if e.label == label {
                    id = Some(i);
                    break;
                }
            }
            if let Some(i) = id {
                hashmap.get_mut(hash as usize).unwrap().remove(i);
            }
        }
    }
    hashmap
        .iter()
        .enumerate()
        .map(|(i, b)| {
            b.iter()
                .enumerate()
                .map(|(j, e)| (1 + i as u32) * (1 + j as u32) * e.value as u32)
                .sum::<u32>()
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(15, 511215, 236057);

    #[test]
    fn test_base() {
        let example = "rn=1,cm-,qp=3,cm=2,qp-,pc=4,ot=9,ab=5,pc-,pc=6,ot=7".as_bytes();
        assert_eq!(part1(&generator(example)), 1320);
        assert_eq!(part2(&generator(example)), 145);
    }
}
