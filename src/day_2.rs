use std::str::from_utf8;

type Int = u32;

#[derive(Default)]
struct Colors {
    red: u8,
    green: u8,
    blue: u8,
}

pub struct Game {
    id: u8,
    colors: Vec<Colors>,
}

pub fn generator(input: &[u8]) -> Vec<Game> {
    from_utf8(input)
        .unwrap()
        .lines()
        .map(|line| {
            let (game, sets) = line.split_once(": ").unwrap();
            let id = game[5..].parse().unwrap();
            let colors = sets
                .split("; ")
                .map(|set| {
                    let mut colors = Colors::default();
                    for ind_color in set.split(", ") {
                        let (n, color_str) = ind_color.split_once(' ').unwrap();
                        if color_str == "blue" {
                            colors.blue = n.parse().unwrap();
                        }
                        if color_str == "red" {
                            colors.red = n.parse().unwrap();
                        }
                        if color_str == "green" {
                            colors.green = n.parse().unwrap();
                        }
                    }
                    colors
                })
                .collect();
            Game { id, colors }
        })
        .collect()
}

pub fn part1(input: &[Game]) -> Int {
    input
        .iter()
        .filter_map(|game| {
            if (game.colors.iter().map(|c| c.red).max().unwrap_or_default() <= 12)
                && (game
                    .colors
                    .iter()
                    .map(|c| c.green)
                    .max()
                    .unwrap_or_default()
                    <= 13)
                && (game.colors.iter().map(|c| c.blue).max().unwrap_or_default() <= 14)
            {
                Some(game.id as Int)
            } else {
                None
            }
        })
        .sum()
}

pub fn part2(input: &[Game]) -> Int {
    input
        .iter()
        .map(|game| {
            game.colors.iter().map(|c| c.red).max().unwrap_or_default() as Int
                * (game
                    .colors
                    .iter()
                    .map(|c| c.green)
                    .max()
                    .unwrap_or_default() as Int)
                * (game.colors.iter().map(|c| c.blue).max().unwrap_or_default() as Int)
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;
    use aoc_macro::test_parts;

    test_parts!(2, 1853, 72706);

    #[test]
    fn test_base() {
        let example = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green\n\
            Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue\n\
            Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red\n\
            Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red\n\
            Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green\n"
            .as_bytes();
        assert_eq!(part1(&generator(example)), 8);

        assert_eq!(part2(&generator(example)), 2286);
    }
}
