type Int = u16;

fn str_to_int(s: &[u8]) -> Option<u8> {
    let b = s[0];
    if b.is_ascii_digit() {
        return Some(b - b'0');
    }
    if s.len() >= 3 {
        if &s[..3] == b"six" {
            return Some(6);
        }
        if &s[..3] == b"two" {
            return Some(2);
        }
        if &s[..3] == b"one" {
            return Some(1);
        }
    }
    if s.len() >= 4 {
        if &s[..4] == b"zero" {
            return Some(0);
        }
        if &s[..4] == b"four" {
            return Some(4);
        }
        if &s[..4] == b"five" {
            return Some(5);
        }
        if &s[..4] == b"nine" {
            return Some(9);
        }
    }
    if s.len() >= 5 {
        if &s[..5] == b"three" {
            return Some(3);
        }
        if &s[..5] == b"seven" {
            return Some(7);
        }
        if &s[..5] == b"eight" {
            return Some(8);
        }
    }
    None
}

pub fn generator(input: &[u8]) -> &[u8] {
    input
}

pub fn part1(input: &[u8]) -> Int {
    input
        .split(|b| b == &b'\n')
        .map(|line| {
            let first = line.iter().find(|v| v.is_ascii_digit());
            let last = line.iter().rev().find(|v| v.is_ascii_digit());

            if let (Some(first), Some(last)) = (first, last) {
                ((first - b'0') as Int * 10 + (last - b'0') as Int) as Int
            } else {
                0
            }
        })
        .sum()
}

pub fn part2(input: &[u8]) -> Int {
    input
        .split(|b| b == &b'\n')
        .map(|line| {
            let mut vec = Vec::with_capacity(line.len());
            for i in 0..line.len() {
                if let Some(v) = str_to_int(&line[i..]) {
                    vec.push(v);
                }
            }
            if let (Some(first), Some(last)) = (vec.first(), vec.last()) {
                (first * 10 + last) as Int
            } else {
                0
            }
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
            treb7uchet"
            .as_bytes();
        assert_eq!(part1(generator(example)), 142);

        let example2 = "two1nine\n\
            eightwothree\n\
            abcone2threexyz\n\
            xtwone3four\n\
            4nineeightseven2\n\
            zoneight234\n\
            7pqrstsixteen"
            .as_bytes();
        assert_eq!(part2(generator(example2)), 281);
    }
}
