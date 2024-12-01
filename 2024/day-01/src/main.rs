use std::collections::HashMap;

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

fn part1(input: &str) -> usize {
    let mut columns: (Vec<usize>, Vec<usize>) = input
        .par_lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            (numbers[0], numbers[1])
        })
        .unzip();
    columns.0.sort();
    columns.1.sort();
    columns
        .0
        .iter()
        .zip(columns.1.iter())
        .map(|(a, b)| (*a as isize - *b as isize).abs() as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    let mut columns: (Vec<usize>, Vec<usize>) = input
        .par_lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            (numbers[0], numbers[1])
        })
        .unzip();
    let left_counts: HashMap<usize, usize> = columns.0.into_iter().counts();
    let right_counts: HashMap<usize, usize> = columns.1.into_iter().counts();
    let mut result = 0;
    for (number, left_count) in left_counts.iter() {
        result += left_count * number * right_counts.get(number).unwrap_or(&0);
    }
    result
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("./example.txt")), 11);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 2378066);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("./example.txt")), 31);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./input.txt")), 18934359);
    }
}
