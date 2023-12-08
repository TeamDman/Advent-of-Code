use std::collections::{HashMap, HashSet};

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

#[derive(Clone, Copy, Debug, Hash, Eq, PartialEq, PartialOrd, Ord)]
enum Direction {
    Left,
    Right,
}

trait Picker {
    fn choose<'a>(self: Self, left: &'a str, right: &'a str) -> &'a str;
}

impl Picker for Direction {
    fn choose<'a>(self: Self, left: &'a str, right: &'a str) -> &'a str {
        match self {
            Self::Left => left,
            Self::Right => right,
        }
    }
}

fn part1(input: &str) -> usize {
    let mut lines = input.lines();
    // println!("first: {}", lines.clone().next().unwrap());
    let pattern = lines.next().unwrap().chars().map(|c| match c {
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => panic!("bad input: {}", c),
    });
    // println!("pattern: {:?}", pattern.clone());
    let lookup = lines
        .skip(1)
        .map(|line| {
            let name = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            // println!("got {}=({},{})", name, left, right);
            (name, (left, right))
        })
        .collect::<HashMap<&str, (&str, &str)>>();
    // println!("Lookup built with {} entries", lookup.keys().count());

    let mut current = "AAA";
    let mut steps = 0;
    let mut visited = HashSet::new();
    loop {
        for (i, dir) in pattern.clone().enumerate() {
            if visited.contains(&(i, current, dir)) {
                panic!("already visited ({}, {},{:?})", i, current, &dir);
            } else {
                visited.insert((i, current, dir));
            }
            if let Some(&(left, right)) = lookup.get(current) {
                let next = dir.choose(left, right);
                // println!(
                //     "{} = ({}, {}), going {:?} ({})",
                //     current,
                //     left,
                //     right,
                //     &dir,
                //     next
                // );
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

fn part2(input: &str) -> usize {
    let mut lines = input.lines();
    let pattern = lines.next().unwrap().chars().map(|c| match c {
        'R' => Direction::Right,
        'L' => Direction::Left,
        _ => panic!("bad input: {}", c),
    }).collect_vec();
    let lookup = lines
        .skip(1)
        .map(|line| {
            let name = &line[0..3];
            let left = &line[7..10];
            let right = &line[12..15];
            (name, (left, right))
        })
        .collect::<HashMap<&str, (&str, &str)>>();

    
        let mut current_nodes: HashSet<&str> = lookup.keys()
        .filter(|&k| k.ends_with("A"))
        .cloned()
        .collect();

    let mut steps = 0;
    while current_nodes.iter().any(|node| !node.ends_with("Z")) {
        let dir = pattern[steps % pattern.len()];
        current_nodes = current_nodes.iter()
            .flat_map(|&node| {
                let (left, right) = lookup.get(node).expect("Node not found in lookup");
                let next_node = dir.choose(left, right);
                Some(next_node)
            })
            .collect();
        steps += 1;
    }

    steps
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
        assert_eq!(part2(include_str!("./input.txt")), 0);
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
