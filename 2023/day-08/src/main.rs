use std::collections::{HashMap, HashSet};

use common::prelude::*;
use itertools::Itertools;
use num_integer::lcm;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> usize {
    let pattern = parse_pattern(input);
    let lookup = parse_lookup(input);

    let mut current = "AAA";
    let mut steps = 0;
    let mut visited = HashSet::new();
    loop {
        for (i, dir) in pattern.iter().enumerate() {
            if visited.contains(&(i, current, dir)) {
                panic!("already visited ({}, {},{:?})", i, current, &dir);
            } else {
                visited.insert((i, current, dir));
            }
            if let Some(&(left, right)) = lookup.get(current) {
                let next = match dir {
                    Direction::Left => left,
                    Direction::Right => right,
                };
                current = next;
            } else {
                panic!("lookup key missing: {}", current);
            }
            steps += 1;
            if current == "ZZZ" {
                return steps;
            }
        }
    }
}

fn part2(input: &str) -> i128 {
    let pattern = parse_pattern(input);
    let lookup = parse_lookup(input);
    println!("parse complete");

    let path_infos: Vec<_> = lookup.keys()
        .filter(|k| k.ends_with("A"))
        .map(|start_node| track_path(&lookup, &pattern, start_node))
        .collect();

    println!("got path infos: {:?}", path_infos);

    calculate_alignment_step(path_infos)
}

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

fn parse_pattern(input: &str) -> Vec<Direction> {
    input
        .lines()
        .next()
        .unwrap()
        .chars()
        .map(|c| match c {
            'R' => Direction::Right,
            'L' => Direction::Left,
            _ => panic!("bad input: {}", c),
        })
        .collect_vec()
}
fn parse_lookup(input: &str) -> HashMap<&str, (&str, &str)> {
    input
        .lines()
        .skip(2)
        .map(|line| {
            let name = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (name, (left, right))
        })
        .collect()
}

/// Identify the cycle length and the steps to get to each z-ending
fn track_path(
    lookup: &HashMap<&str, (&str, &str)>,
    pattern: &[Direction],
    start_node: &str,
) -> (usize, HashSet<usize>) {
    let mut current = start_node;
    let mut step = 0;
    let mut visited = HashSet::new();
    let mut endings = HashSet::new();
    'outer: loop {
        for (i, dir) in pattern.iter().enumerate() {
            if visited.contains(&(i, current, dir)) {
                break 'outer;
            } else {
                visited.insert((i, current, dir));
            }
            if let Some(&(left, right)) = lookup.get(current) {
                let next = match pattern[step % pattern.len()] {
                    Direction::Left => left,
                    Direction::Right => right,
                };
                current = next;
            } else {
                panic!("lookup key missing: {}", current);
            }
            step += 1;
            if current.ends_with("Z") {
                endings.insert(step);
            }
        }
    }
    (step, endings)
}

fn calculate_alignment_step(path_infos: Vec<(usize, HashSet<usize>)>) -> i128 {
    path_infos.iter().flat_map(|x| x.1.iter()).map(|x| *x as i128).reduce(lcm).unwrap()
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 20221);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./input.txt")), 14616363770447);
    }

    #[test]
    fn part1_example1() {
        assert_eq!(part1(include_str!("./example1.txt")), 2);
    }
    #[test]
    fn part1_example2() {
        assert_eq!(part1(include_str!("./example2.txt")), 6);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("./example3.txt")), 6);
    }
}
