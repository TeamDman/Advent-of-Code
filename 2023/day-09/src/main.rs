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


fn predict_next(sequence: &[isize]) -> isize {
    let mut layers = vec![sequence.to_vec()];

    while !layers.last().unwrap().windows(2).all(|w| w[0] == w[1]) {
        let next_layer = layers.last().unwrap()
                                .windows(2)
                                .map(|w| w[1] - w[0])
                                .collect();
        layers.push(next_layer);
    }

    // dbg!(&layers);

    let mut next_value = 0; //*sequence.last().unwrap();
    for layer in layers.iter().rev() {
        next_value += layer.last().unwrap();
    }

    next_value
}

fn part1(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect_vec()
        })
        .map(|sequence| predict_next(sequence.as_slice()))
        .map(|x| {
            // println!("{}", x);
            x
        })
        .sum()
}

fn part2(input: &str) -> isize {
    input
        .lines()
        .map(|line| {
            line.split_whitespace()
                .map(|s| s.parse::<isize>().unwrap())
                .collect_vec()
        })
        .map(|sequence| sequence.iter().map(|x| -x).rev().collect_vec())
        .map(|sequence| predict_next(sequence.as_slice()))
        .map(|x| {
            // println!("{}", x);
            -x
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("./example.txt")), 114);
    }

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 1887980197);
    }


    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("./example.txt")), 2);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./input.txt")), 990);
    }

}
