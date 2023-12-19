use std::str::from_utf8;

pub fn generator(input: &[u8]) -> &[u8] {
    input
}

pub fn part1(input: &[u8]) -> u64 {
    let iter = input[0..input.len() - 1]
        .split(|b| b == &b'\n')
        .map(|line| {
            let direction = *line.first().unwrap();
            let n = line[2..].splitn(2, |b| b == &b' ').next().unwrap();
            let n: i64 = from_utf8(n).unwrap().parse().unwrap();
            (direction, n)
        });
    area(iter)
}

pub fn part2(input: &[u8]) -> u64 {
    let iter = input[0..input.len() - 1]
        .split(|b| b == &b'\n')
        .map(|line| {
            let hex = line[0..line.len() - 1]
                .splitn(2, |b| b == &b'#')
                .nth(1)
                .unwrap();
            let (direction, n) = hex.split_last().unwrap();
            let mut num = 0;
            for e in n {
                num *= 16;
                if e.is_ascii_digit() {
                    num += (e - b'0') as i64;
                } else {
                    num += (e - b'a' + 10) as i64;
                }
            }
            (*direction - b'0', num)
        });

    area(iter)
}

fn area(iter: impl Iterator<Item = (u8, i64)>) -> u64 {
    let mut current_position = (0, 0);
    let mut n_points_on_side = 0;
    let mut partial_area = 0;
    iter.for_each(|(direction, n)| {
        let direction = match direction {
            b'U' => (0, -1),
            b'D' => (0, 1),
            b'L' => (-1, 0),
            b'R' => (1, 0),
            3 => (0, -1),
            1 => (0, 1),
            2 => (-1, 0),
            0 => (1, 0),
            _ => unreachable!(),
        };
        if direction.1 == 0 {
            // Shoelace formula
            partial_area += current_position.1 * direction.0 * n;
        }
        current_position.0 += direction.0 * n;
        current_position.1 += direction.1 * n;
        n_points_on_side += n;
    });
    // Pick's theorem
    partial_area.unsigned_abs() + n_points_on_side as u64 / 2 + 1
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_macro::test_parts;
    test_parts!(18, 58550, 47452118468566);

    #[test]
    fn test_base() {
        let example = "R 6 (#70c710)\n\
                            D 5 (#0dc571)\n\
                            L 2 (#5713f0)\n\
                            D 2 (#d2c081)\n\
                            R 2 (#59c680)\n\
                            D 2 (#411b91)\n\
                            L 5 (#8ceee2)\n\
                            U 2 (#caa173)\n\
                            L 1 (#1b58a2)\n\
                            U 2 (#caa171)\n\
                            R 2 (#7807d2)\n\
                            U 3 (#a77fa3)\n\
                            L 2 (#015232)\n\
                            U 2 (#7a21e3)\n"
            .as_bytes();
        assert_eq!(part1(generator(example)), 62);
        assert_eq!(part2(generator(example)), 952408144115);
    }
}
