type Int = u16;

const ZERO: u8 = 48;
const NINE: u8 = 57;

fn str_to_int(s: &str) -> Option<u8> {
    let b = s[..1].as_bytes()[0];
    if (ZERO..=NINE).contains(&b) {
        return Some(b - ZERO);
    }
    if s.len() >= 3 {
        if &s[..3] == "six" {
            return Some(6);
        }
        if &s[..3] == "two" {
            return Some(2);
        }
        if &s[..3] == "one" {
            return Some(1);
        }
    }
    if s.len() >= 4 {
        if &s[..4] == "zero" {
            return Some(0);
        }
        if &s[..4] == "four" {
            return Some(4);
        }
        if &s[..4] == "five" {
            return Some(5);
        }
        if &s[..4] == "nine" {
            return Some(9);
        }
    }
    if s.len() >= 5 {
        if &s[..5] == "three" {
            return Some(3);
        }
        if &s[..5] == "seven" {
            return Some(7);
        }
        if &s[..5] == "eight" {
            return Some(8);
        }
    }
    None
}

pub fn generator(input: &str) -> &str {
    input
}
pub fn part1(input: &str) -> Int {
    input
        .lines()
        .map(|line| {
            let iter = line.bytes().filter_map(|v| {
                if (ZERO..=NINE).contains(&v) {
                    Some(v - ZERO)
                } else {
                    None
                }
            });
            let first = iter.to_owned().next().unwrap();
            let last = iter.last().unwrap();
            (first * 10 + last) as Int
        })
        .sum()
}

pub fn part2(input: &str) -> Int {
    input
        .lines()
        .map(|line| {
            let mut vec = Vec::with_capacity(line.len());
            for i in 0..line.len() {
                if let Some(v) = str_to_int(&line[i..]) {
                    vec.push(v);
                }
            }
            let first = vec.first().unwrap();
            let last = vec.last().unwrap();
            (first * 10 + last) as Int
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_macro::test_parts;

    test_parts!(1, 55172, 54925);

    #[test]
    fn test_base() {
        let example = "1abc2\n\
            pqr3stu8vwx\n\
            a1b2c3d4e5f\n\
            treb7uchet";
        assert_eq!(part1(generator(example)), 142);

        let example2 = "two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen";
        assert_eq!(part2(generator(example2)), 281);
    }
}
