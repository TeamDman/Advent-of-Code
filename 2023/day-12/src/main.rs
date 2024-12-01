use std::collections::HashMap;

use common::prelude::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelRefIterator, ParallelBridge, ParallelIterator};
use rayon::str::ParallelString;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}


struct Problem<'a> {
    pattern: &'a str,
    constraints: Vec<usize>,
    multiplier: usize,
}

/// Given a problem, count the number of valid arrangements from resolving wildcards.
fn count_solutions(problem: Problem) -> usize {
    count_solutions_memoized(problem.pattern, &problem.constraints)
}

fn is_valid(arrangement: &str, constraints: &[usize]) -> bool {
    let mut groups = Vec::new();
    let mut count = 0;
    for c in arrangement.chars() {
        if c == '#' {
            count += 1;
        } else if count > 0 {
            groups.push(count);
            count = 0;
        }
    }
    if count > 0 {
        groups.push(count);
    }
    groups == constraints
}

fn backtrack_memoized(pattern: &str, constraints: &[usize], index: usize, current: &str, memo: &mut HashMap<(usize, String), usize>) -> usize {
    if let Some(&count) = memo.get(&(index, current.to_string())) {
        return count;
    }

    if index == pattern.len() {
        let count = if is_valid(current, constraints) { 1 } else { 0 };
        memo.insert((index, current.to_string()), count);
        return count;
    }

    if pattern.chars().nth(index).unwrap() != '?' {
        let next = format!("{}{}", current, pattern.chars().nth(index).unwrap());
        return backtrack_memoized(pattern, constraints, index + 1, &next, memo);
    }

    let count = backtrack_memoized(pattern, constraints, index + 1, &format!("{}{}", current, '.'), memo)
        + backtrack_memoized(pattern, constraints, index + 1, &format!("{}{}", current, '#'), memo);
    
    memo.insert((index, current.to_string()), count);
    count
}

fn count_solutions_memoized(pattern: &str, constraints: &[usize]) -> usize {
    let mut memo = HashMap::new();
    backtrack_memoized(pattern, constraints, 0, "", &mut memo)
}




enum Part {
    Part1,
    Part2,
    Test(usize),
}
fn parse_line<'a>(input: &'a str, part: &Part) -> Problem<'a> {
    let (left, right) = input.split_once(" ").unwrap();
    let pattern = left;
    let constraints = right
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let mut multiplier = 1;
    match part {
        Part::Part2 => {
            multiplier = 5;
        }
        Part::Test(n) => {
            multiplier = *n;
        }
        _ => {}
    }
    Problem {
        pattern,
        constraints,
        multiplier,
    }
}

fn parse(input: &str, part: Part) -> Vec<Problem> {
    input
        .par_lines()
        .map(|line| parse_line(line, &part))
        .collect()
}

fn part1(input: &str) -> usize {
    parse(input, Part::Part1)
        .into_iter()
        .map(count_solutions)
        .sum()
}

fn part2(input: &str) -> usize {
    parse(input, Part::Part2)
        .into_iter()
        .map(count_solutions)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("./example.txt")), 21);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 6981);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(".??..??...?##. 1,1,3"), 16384);
        assert_eq!(part2(include_str!("./example.txt")), 525152);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./input.txt")), 0);
    }
}
