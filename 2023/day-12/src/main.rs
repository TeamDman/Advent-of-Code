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

fn is_valid_arrangement(line: &str, constraint: &Vec<usize>) -> bool {
    let chunks = line.split(".").filter(|x| x.len() > 0);
    // println!("{:?}", chunks.clone().collect_vec());
    if chunks.clone().count() != constraint.len() {
        return false;
    }
    chunks
        .enumerate()
        .par_bridge()
        .all(|(i, x)| x.len() == 0 || x.len() == constraint[i])
}

fn generate_variants(seq: &str) -> Vec<String> {
    let question_marks = seq.matches('?').count(); // Counting the '?' characters
    let combinations = 1 << question_marks; // 2^n

    (0..combinations)
        .par_bridge()
        .map(|i| {
            let mut variant = String::new();
            let mut mask = i;
            for ch in seq.chars() {
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
            variant
        })
        .collect()
}

fn count_valid_arrangements(line: &str) -> usize {
    let (left, right) = line.split_once(" ").unwrap();
    let constraint = right
        .split(",")
        .map(|x| x.parse::<usize>().unwrap())
        .collect_vec();
    let variants = generate_variants(left);

    // for variant in variants.iter() {
    //     println!("{}", variant);
    // }

    variants
        .par_iter()
        .filter(|variant| is_valid_arrangement(variant, &constraint))
        .count()
}

fn part1(input: &str) -> usize {
    input
        .lines()
        .map(|line| count_valid_arrangements(line))
        .sum()
}

fn part2(input: &str) -> usize {
    input
        .lines()
        .map(|line| {
            let (left, right) = line.split_once(" ").unwrap();
            // duplicate left and right 5 times
            // concat the right clones with ","
            // join into new line
            let left = left.repeat(5);
            // let right = right.repeat(5); // how to comma separate?
            let right = (0..5).map(|_| right).join(",");
            let line = format!("{} {}", left, right);
            // println!("new line: {}", line);
            line
        })
        .map(|line| count_valid_arrangements(line.as_str()))
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn gen_variants() {
        let seq = "???.###";
        let variants = generate_variants(seq);
        let expected = vec![
            "....###", "#...###", ".#..###", "##..###", "..#.###", "#.#.###", ".##.###",
            "###.###"
        ];
        // ensure lengths are the same
        // ensure all expected are in variants
        assert_eq!(variants.len(), expected.len());
        for variant in expected.iter() {
            assert!(variants.contains(&variant.to_string()));
        }
    }

    #[test]
    fn test_is_valid_arrangement() {
        assert!(is_valid_arrangement("....###", &vec![3]));
        assert!(!is_valid_arrangement("....###", &vec![3, 4]));
        assert!(!is_valid_arrangement("....###", &vec![3, 3]));
        assert!(!is_valid_arrangement("....###", &vec![3, 3, 3]));
        assert!(is_valid_arrangement("..##..###", &vec![2, 3]));
        assert!(is_valid_arrangement("....##.....###", &vec![2, 3]));
    }

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
