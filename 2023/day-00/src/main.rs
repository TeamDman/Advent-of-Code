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
    todo!()
}

fn part2(input: &str) -> usize {
    todo!()
}

#[cfg(test)]
mod tests {
    use super::*;

    // #[test]
    // fn test_part1() {
    //     let input = include_str!("./input.txt");
    //     assert_eq!(part1(input), 12345);
    // }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("./input.txt");
    //     assert_eq!(part2(input), 12345);
    // }

    const EXAMPLE : &str = r#"
"#;

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 8);
    }

    // #[test]
    // fn part2_example() {
    //     assert_eq!(part2(EXAMPLE), 2286);
    // }

    // #[test]
    // fn part1_badanswer() {
    //     assert!(
    //         part2(include_str!("./input.txt")) < 12345,
    //         "Answer too high"
    //     );
    // }
    // #[test]
    // fn part2_badanswer() {
    //     assert!(
    //         part2(include_str!("./input.txt")) < 12345,
    //         "Answer too high"
    //     );
    // }
}
