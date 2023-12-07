use common::prelude::*;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

fn part1(input: &str) -> usize {
    let (times, distances) = input
        .split("\n")
        .map(|line| {
            line.split_once(":")
                .unwrap()
                .1
                .split_ascii_whitespace()
                .map(|x| x.parse::<usize>().unwrap())
                .collect_vec()
        })
        .take(2)
        .collect_tuple()
        .unwrap();
    let ways_to_beat = times.iter().zip(distances.iter()).map(|(t, d)| {
        // distance = charge_time * (time - charge_time)
        // each time unit spent charging increases velocity by 1
        // d is an attempt at the race
        // we must calculate the number of ways we can beat each attempt
        let mut ways = 0;
        for i in 0..*t {
            let charge_time = i + 1;
            let velocity = charge_time;
            let distance = velocity * (t - charge_time);
            if distance > *d {
                ways += 1;
            }
        }
        ways
    });
    ways_to_beat.product()
}

fn part2(input: &str) -> usize {
    // this time, ignore all whitespace such that all numbers in each line are concatenated to produce a single time,distance pair to beat
    let (time, distance) = input.split("\n").map(|line| {
        line.split_once(":")
            .unwrap()
            .1
            .chars()
            .map(|x| x.to_digit(10))
            .filter(Option::is_some)
            .map(Option::unwrap)
            .fold(0, |acc, x| acc * 10 + x as u64)
    }).take(2).collect_tuple().unwrap();
    let mut ways = 0;
    for i in 0..time {
        let charge_time = i + 1;
        let velocity = charge_time;
        let new_distance = velocity * (time - charge_time);
        if new_distance > distance {
            ways += 1;
        }
    }
    ways
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");
        assert_eq!(part1(input), 1624896);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");
        assert_eq!(part2(input), 32583852);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("./example.txt")), 288);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("./example.txt")), 71503);
    }
}
