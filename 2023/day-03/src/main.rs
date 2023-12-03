use std::collections::{HashMap, HashSet};

use common::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

struct Input {
    // (x, y) -> (x_start, y, number)
    numbers: HashMap<(isize, isize), (isize, isize, usize)>,
    symbols: HashMap<(isize, isize), char>,
}

fn parse(input: &str) -> Input {
    let mut rtn = Input {
        numbers: HashMap::new(),
        symbols: HashMap::new(),
    };
    input.lines().enumerate().for_each(|(y, line)| {
        let mut x = 0;
        while x < line.len() {
            let char = line.chars().nth(x).unwrap();
            if char == '.' {
                x += 1;
                continue;
            }
            if char.is_numeric() {
                let mut number = char.to_digit(10).unwrap();
                let mut number_string = char.to_string();
                let x1 = x;
                let mut x2 = x;
                for char in line.chars().skip(x + 1) {
                    if char.is_numeric() {
                        number *= 10;
                        number += char.to_digit(10).unwrap();
                        number_string.push(char);
                        x2 += 1;
                    } else {
                        break;
                    }
                }
                for x in x1..=x2 {
                    rtn.numbers.insert(
                        (x as isize, y as isize),
                        (x1 as isize, y as isize, number as usize),
                    );
                }
                x = x2 + 1; // Skip to the character after the number
            } else {
                rtn.symbols.insert((x as isize, y as isize), char);
                x += 1;
            }
        }
    });
    rtn
}

fn get_adjacent_numbers(input: &Input, x: isize, y: isize) -> HashSet<(isize, isize, usize)> {
    let mut rtn = HashSet::new();
    for dx in -1isize..=1 {
        for dy in -1isize..=1 {
            if dx == 0 && dy == 0 {
                continue;
            }
            if let Some((kx, ky, number)) = input.numbers.get(&(x + dx, y + dy)) {
                rtn.insert((*kx, *ky, *number));
            }
        }
    }
    rtn
}

fn part1(input: &str) -> usize {
    let input = parse(input);
    // find all numbers that are 8-ways adjacent to a symbol and sum them
    // use the get_adjacent to filter the numbers to those that are adjacent to a symbol
    // use a set to avoid double counting
    let mut set = HashSet::new();
    for (x, y) in input.symbols.keys() {
        for (kx, ky, number) in get_adjacent_numbers(&input, *x, *y) {
            set.insert((kx, ky, number));
        }
    }
    set.iter().map(|(_, _, number)| number).sum()
}

fn part2(input: &str) -> usize {
    // a gear is any '*' symbol adjacent to exactly two numbers
    // find the sum of the product of the two numbers adjacent to each gear
    let input = parse(input);
    let mut total = 0;
    for (x, y) in input.symbols.keys() {
        let adjacent = get_adjacent_numbers(&input, *x, *y);
        if adjacent.len() == 2 {
            total += adjacent.iter().map(|(_, _, number)| number).product::<usize>();
        }
    }
    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");
        assert_eq!(part1(input), 540025);
    }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("./input.txt");
    //     assert_eq!(part2(input), 12345);
    // }

    #[test]
    fn parser_1() {
        let input = "123#456";
        let input = parse(input);
        assert_eq!(input.numbers.len(), 6);
        assert_eq!(input.symbols.len(), 1);
        assert_eq!(input.numbers.get(&(0, 0)), Some(&(0, 0, 123)));
        assert_eq!(input.numbers.get(&(1, 0)), Some(&(0, 0, 123)));
        assert_eq!(input.numbers.get(&(2, 0)), Some(&(0, 0, 123)));
        assert_eq!(input.numbers.get(&(4, 0)), Some(&(4, 0, 456)));
        assert_eq!(input.numbers.get(&(5, 0)), Some(&(4, 0, 456)));
        assert_eq!(input.numbers.get(&(6, 0)), Some(&(4, 0, 456)));
        assert!(input.symbols.get(&(3, 0)).is_some());
    }
    #[test]
    fn parser_2() {
        let input = "..123...\n345#....";
        let input = parse(input);
        assert_eq!(input.numbers.len(), 6);
        assert_eq!(input.symbols.len(), 1);
        assert_eq!(input.numbers.get(&(2, 0)), Some(&(2, 0, 123)));
        assert_eq!(input.numbers.get(&(3, 0)), Some(&(2, 0, 123)));
        assert_eq!(input.numbers.get(&(4, 0)), Some(&(2, 0, 123)));
        assert_eq!(input.numbers.get(&(0, 1)), Some(&(0, 1, 345)));
        assert_eq!(input.numbers.get(&(1, 1)), Some(&(0, 1, 345)));
        assert_eq!(input.numbers.get(&(2, 1)), Some(&(0, 1, 345)));
        assert!(input.symbols.get(&(3, 1)).is_some());
    }

    #[test]
    fn part1_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert!(parse(input).numbers.values().map(|x| x.2).sum::<usize>() > 4361);
        assert!(parse(input).symbols.iter().count() > 0);
        assert_eq!(part1(input), 4361);
    }

    #[test]
    fn part2_example() {
        let input = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        assert_eq!(part2(input), 467835);
    }

    #[test]
    fn part1_badanswer() {
        assert!(
            part1(include_str!("./input.txt")) > 344838,
            "Answer too low"
        );
    }
    #[test]
    fn part2_badanswer() {
        assert!(
            part2(include_str!("./input.txt")) > 74409246,
            "Answer too low"
        );
    }
}
