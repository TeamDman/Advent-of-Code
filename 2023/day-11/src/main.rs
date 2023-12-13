use common::prelude::*;
use itertools::Itertools;
use rayon::iter::{ParallelIterator, IntoParallelIterator};
use rayon::str::ParallelString;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

fn parse_map(input: &str) -> Vec<Vec<char>> {
    let a = input
        .par_lines()
        .flat_map(|line| {
            let mut empty = false;
            let rtn = line.chars().filter(|c| {
                if *c != '.' {
                    empty = false;
                }
                true
            }).collect_vec();
            if empty {
                [rtn,rtn].into_par_iter();
            } else {
                [rtn].into_par_iter();
            }
        })
        .collect_vec();

    todo!();
}

fn part1(input: &str) -> usize {
    todo!()
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
