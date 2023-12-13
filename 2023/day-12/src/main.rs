use common::prelude::*;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::str::ParallelString;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

fn is_valid_arrangement(line: &str, constraint: Vec<usize>) -> bool {}

fn count_valid_arrangements(line: &str) -> usize {
    let (left, right) = line.split_once(" ").unwrap();
    let constraint = right.split(",").map(|x| x.parse::<usize>()).collect_vec();
    
    let question_marks = line.matches('?').count(); // Counting the '?' characters
    let combinations = 1 << question_marks; // 2^n

    let mut variants = Vec::new();

    for i in 0..combinations {
        let mut variant = String::new();
        let mut mask = i;
        for ch in line.chars() {
            match ch {
                '?' => {
                    if mask & 1 == 0 {
                        variant.push('.');
                    } else {
                        variant.push('#');
                    }
                    mask >>= 1;
                }
                _ => variant.push(ch),
            }
        }
        variants.push(variant);
    }

    for variant in variants {
        println!("{}", variant);
    }
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| count_valid_arrangements(line))
        .sum()
}

fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("./example.txt")), 0);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 0);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("./example.txt")), 0);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./input.txt")), 0);
    }
}
