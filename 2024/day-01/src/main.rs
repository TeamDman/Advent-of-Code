use std::collections::HashMap;

use common::prelude::*;
use itertools::Itertools;
use rayon::iter::{IndexedParallelIterator, IntoParallelRefIterator, ParallelIterator};
use rayon::str::ParallelString;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

fn get_columns(input: &str) -> (Vec<usize>, Vec<usize>) {
    input
        .par_lines()
        .map(|line| {
            let numbers = line
                .split_whitespace()
                .map(|s| s.parse::<usize>().unwrap())
                .collect_vec();
            (numbers[0], numbers[1])
        })
        .unzip()
}

fn part1(input: &str) -> usize {
    let (mut left, mut right) = get_columns(input);
    left.sort();
    right.sort();
    left.par_iter()
        .zip(right.par_iter())
        .map(|(a, b)| (*a as isize - *b as isize).abs() as usize)
        .sum()
}

fn part2(input: &str) -> usize {
    let (left, right) = get_columns(input);
    let left_counts: HashMap<usize, usize> = left.into_iter().counts();
    let right_counts: HashMap<usize, usize> = right.into_iter().counts();
    left_counts
        .par_iter()
        .map(|(number, left_count)| left_count * number * right_counts.get(&number).unwrap_or(&0))
        .sum()
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
