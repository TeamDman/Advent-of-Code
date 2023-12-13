use std::collections::{HashMap, HashSet};
use std::fmt::{Display, Write};

use common::prelude::*;
use itertools::Itertools;
use rayon::iter::{IntoParallelIterator, ParallelIterator};
use rayon::str::ParallelString;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

fn parse_board(input: &str) -> Board {
    let map = input
        .lines()
        .enumerate()
        .flat_map(|(y, line)| line.chars().enumerate().map(move |(x, c)| ((x, y), c)));

    let positions = map.filter(|((x, y), c)| *c == '#').map(|a| a.0).collect();

    let dimensions = (
        input.lines().next().unwrap().chars().count(),
        input.lines().count(),
    );

    Board {
        positions,
        dimensions,
    }
}

struct Board {
    positions: HashSet<(usize, usize)>,
    dimensions: (usize, usize),
}
impl Display for Board {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        for y in 0..self.dimensions.1 {
            for x in 0..self.dimensions.0 {
                f.write_char(self.positions.get(&(x, y)).map(|_| '#').unwrap_or('.'))?;
            }
            f.write_char('\n')?;
        }
        Ok(())
    }
}

fn expand_board(board: Board, factor: usize) -> Board {
    let xs = board
        .positions
        .iter()
        .map(|k| k.0)
        .collect::<HashSet<usize>>();
    // println!("xs {:?}", xs.iter().sorted());
    let ys = board
        .positions
        .iter()
        .map(|k| k.1)
        .collect::<HashSet<usize>>();
    // println!("ys {:?}", ys.iter().sorted());
    let (max_x, max_y) = board.dimensions;
    let ex = (0..max_x)
        .map(|x| !xs.contains(&x) as usize)
        .scan(0, |state, a| {
            *state += a;
            Some(*state)
        })
        .collect_vec();
    // println!("ex {:?}", ex.iter());
    let ey = (0..max_y)
        .map(|y| !ys.contains(&y) as usize)
        .scan(0, |state, a| {
            *state += a;
            Some(*state)
        })
        .collect_vec();
    // println!("ey {:?}", ey.iter());

    let positions = board
        .positions
        .iter()
        .map(|(x, y)| (x + ex[*x] * factor, y + ey[*y] * factor))
        .collect::<HashSet<_>>();
    // assert_eq!(board.positions.len(), positions.len());
    let max_x = max_x + ex.last().unwrap() * factor;
    let max_y = max_y + ey.last().unwrap() * factor;

    Board {
        positions,
        dimensions: (max_x, max_y),
    }
}

fn calc_distance_sums(board: &Board) -> usize {
    board
    .positions
    .iter()
    .map(|a| {
        board
            .positions
            .iter()
            .filter(|b| *b != a)
            .map(|b| {
                ((a.0 as isize - b.0 as isize).abs() + (a.1 as isize - b.1 as isize).abs())
                    as usize
            })
            .sum::<usize>()
    })
    .sum::<usize>() / 2
}

fn part1(input: &str) -> usize {
    let board = expand_board(parse_board(input), 1);
    calc_distance_sums(&board)
}


fn part2(input: &str) -> usize {
    let board = expand_board(parse_board(input), 1_000_000 - 1);
    calc_distance_sums(&board)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("./example.txt")), 374);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 10313550);
    }

    #[test]
    fn expansion() {
        let board = expand_board(parse_board(include_str!("./example.txt")), 1);
        println!("{}\n", board);
        let board = expand_board(parse_board(include_str!("./example.txt")), 3);
        println!("{}\n", board);
        let board = expand_board(parse_board(include_str!("./example.txt")), 10);
        println!("{}", board);
    }

    #[test]
    fn part2_example() {
        let board = expand_board(parse_board(include_str!("./example.txt")), 10-1);
        assert_eq!(calc_distance_sums(&board), 1030);
        let board = expand_board(parse_board(include_str!("./example.txt")), 100-1);
        assert_eq!(calc_distance_sums(&board), 8410);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./input.txt")), 611998089572);
    }
}
