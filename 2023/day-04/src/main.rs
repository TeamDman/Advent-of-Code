use std::collections::HashSet;

use common::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

fn parse_cards<'a>(input: &'a str) -> impl Iterator<Item = (HashSet<usize>, HashSet<usize>)> + 'a {
    input.lines().map(|lines| {
        let nums = lines.split_once(": ").unwrap().1;
        let (winning, revealed) = nums.split_once(" | ").unwrap();
        // println!("winning: {}\nrevealed: {}", winning, revealed);
        let winning = winning
            .split_ascii_whitespace()
            .map(|x| {
                x.parse::<usize>()
                    .expect(format!("Str {} should be numeric", x).as_str())
            })
            .collect::<HashSet<usize>>();
        let revealed = revealed
            .split_ascii_whitespace()
            .map(|x| {
                x.parse::<usize>()
                    .expect(format!("Str {} should be numeric", x).as_str())
            })
            .collect::<HashSet<usize>>();
        (winning, revealed)
    })
}

fn part1(input: &str) -> usize {
    parse_cards(input)
        .map(|(winning, revealed)| {
            let matched = revealed.intersection(&winning).count();
            if matched == 0 {
                0
            } else {
                1 << (matched - 1)
            }
        })
        .sum()
}

fn part2(input: &str) -> usize {
    let cards: Vec<(usize, usize)> = parse_cards(input)
        .map(|(winning, revealed)| revealed.intersection(&winning).count())
        .enumerate()
        .collect();
    let mut counts = vec![1; cards.len()];
    for (i, wins) in cards.iter() {
        let this_count = counts[*i];
        for new in counts[i + 1..i + 1 + wins].iter_mut() {
            *new += this_count;
        }
    }
    counts.iter().sum::<usize>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");
        assert_eq!(part1(input), 15205);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");
        assert_eq!(part2(input), 6189740);
    }

    #[test]
    fn part1_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        assert_eq!(part1(input), 13);
    }

    #[test]
    fn part2_example() {
        let input = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";

        assert_eq!(part2(input), 30);
    }
}
