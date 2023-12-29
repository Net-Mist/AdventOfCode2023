use std::str::from_utf8;

use ahash::HashMap;

#[derive(Clone, Copy, Debug)]
pub enum Category {
    X,
    M,
    A,
    S,
}

impl Category {
    fn from_str(s: &str) -> Self {
        match s {
            "x" => Category::X,
            "m" => Category::M,
            "a" => Category::A,
            "s" => Category::S,
            _ => unreachable!(),
        }
    }
}

#[derive(Debug)]
pub enum Rule {
    Lower((Category, u16)),
    Greater((Category, u16)),
    None,
}

type Input<'a> = (HashMap<&'a str, Vec<(&'a str, Rule)>>, Vec<Vec<u16>>);

pub fn generator(input: &[u8]) -> Input {
    let input = from_utf8(input).unwrap();
    let (workflow, parts) = input.split_once("\n\n").unwrap();

    let workflow = workflow
        .lines()
        .map(|line| {
            let (name, other) = line.split_once('{').unwrap();
            let rules = other[0..other.len() - 1]
                .split(',')
                .map(|rule| {
                    if rule.contains(':') {
                        let (rule, label) = rule.split_once(':').unwrap();

                        if rule.contains('<') {
                            let (category, value) = rule.split_once('<').unwrap();

                            (
                                label,
                                Rule::Lower((Category::from_str(category), value.parse().unwrap())),
                            )
                        } else {
                            let (category, value) = rule.split_once('>').unwrap();
                            (
                                label,
                                Rule::Greater((
                                    Category::from_str(category),
                                    value.parse().unwrap(),
                                )),
                            )
                        }
                    } else {
                        (rule, Rule::None)
                    }
                })
                .collect::<Vec<_>>();
            (name, rules)
        })
        .collect::<HashMap<_, _>>();

    let parts = parts
        .lines()
        .map(|line| {
            let line = &line[1..line.len() - 1];
            line.split(',')
                .map(|e| e.split_once('=').unwrap().1.parse().unwrap())
                .collect()
        })
        .collect::<Vec<Vec<u16>>>();

    (workflow, parts)
}

pub fn part1(input: &Input) -> u32 {
    let (workflow, parts) = input;

    parts
        .iter()
        .map(|part| {
            let mut current_rule = "in";

            'out: loop {
                if current_rule == "A" {
                    return part.iter().sum::<u16>() as u32;
                }
                if current_rule == "R" {
                    return 0;
                }
                let w = workflow.get(current_rule).unwrap();
                for (dest, rule) in w {
                    match rule {
                        Rule::Lower((cat, n)) => {
                            if part[*cat as usize] < *n {
                                current_rule = dest;
                                continue 'out;
                            }
                        }
                        Rule::Greater((cat, n)) => {
                            if part[*cat as usize] > *n {
                                current_rule = dest;
                                continue 'out;
                            }
                        }
                        Rule::None => {
                            current_rule = dest;
                            continue 'out;
                        }
                    }
                }
            }
        })
        .sum()
}

pub fn part2(input: &Input) -> u64 {
    let (workflow, _) = input;

    count("in", vec![(1, 4000); 4], workflow)
}

fn count(
    key: &str,
    mut part: Vec<(u16, u16)>,
    workflow: &std::collections::HashMap<&str, Vec<(&str, Rule)>, ahash::RandomState>,
) -> u64 {
    if key == "A" {
        return part.iter().map(|v| (v.1 - v.0 + 1) as u64).product();
    }
    if key == "R" {
        return 0;
    }
    let w = workflow.get(key).unwrap();
    let mut s = 0;
    for (dest, rule) in w {
        match rule {
            Rule::Lower((cat, n)) => {
                if part[*cat as usize].1 < *n {
                    return s + count(dest, part, workflow);
                }
                if part[*cat as usize].0 >= *n {
                    continue;
                }
                let mut part2 = part.clone();
                part2[*cat as usize].1 = *n - 1;
                part[*cat as usize].0 = *n;

                s += count(dest, part2, workflow);
            }
            Rule::Greater((cat, n)) => {
                if part[*cat as usize].0 > *n {
                    return s + count(dest, part, workflow);
                }
                if part[*cat as usize].1 <= *n {
                    continue;
                }
                let mut part2 = part.clone();
                part2[*cat as usize].0 = *n + 1;
                part[*cat as usize].1 = *n;

                s += count(dest, part2, workflow);
            }
            Rule::None => {
                return s + count(dest, part, workflow);
            }
        }
    }
    0
}

#[cfg(test)]
mod tests {
    use super::*;

    use aoc_macro::test_parts;
    test_parts!(19, 418498, 123331556462603);

    #[test]
    fn test_base() {
        let example = "px{a<2006:qkq,m>2090:A,rfg}\n\
                                pv{a>1716:R,A}\n\
                                lnx{m>1548:A,A}\n\
                                rfg{s<537:gd,x>2440:R,A}\n\
                                qs{s>3448:A,lnx}\n\
                                qkq{x<1416:A,crn}\n\
                                crn{x>2662:A,R}\n\
                                in{s<1351:px,qqz}\n\
                                qqz{s>2770:qs,m<1801:hdj,R}\n\
                                gd{a>3333:R,R}\n\
                                hdj{m>838:A,pv}\n\n\
                                {x=787,m=2655,a=1222,s=2876}\n\
                                {x=1679,m=44,a=2067,s=496}\n\
                                {x=2036,m=264,a=79,s=2244}\n\
                                {x=2461,m=1339,a=466,s=291}\n\
                                {x=2127,m=1623,a=2188,s=1013}\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 19114);
        assert_eq!(part2(&generator(example)), 167409079868000);
    }
}
