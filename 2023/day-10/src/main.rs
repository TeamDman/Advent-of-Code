use core::panic;
use std::collections::{HashMap, HashSet, VecDeque};
use std::iter;

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

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum Tile {
    Start = 'S' as isize,
    NorthSouth = '|' as isize,
    EastWest = '-' as isize,
    NorthEast = 'L' as isize,
    NorthWest = 'J' as isize,
    SouthEast = 'F' as isize,
    SouthWest = '7' as isize,
    Ground = '.' as isize,
    Flooded = 'X' as isize,
}

impl Tile {
    fn offsets(&self) -> Option<impl Iterator<Item = (isize, isize)>> {
        match self {
            Tile::NorthSouth => Some([(0, -1), (0, 1)].into_iter()),
            Tile::EastWest => Some([(1, 0), (-1, 0)].into_iter()),
            Tile::NorthEast => Some([(0, -1), (1, 0)].into_iter()),
            Tile::NorthWest => Some([(0, -1), (-1, 0)].into_iter()),
            Tile::SouthEast => Some([(0, 1), (1, 0)].into_iter()),
            Tile::SouthWest => Some([(0, 1), (-1, 0)].into_iter()),
            Tile::Ground => None,
            Tile::Start => None,
            Tile::Flooded => None,
        }
    }
    fn iter_pipes() -> impl Iterator<Item = Tile> {
        [
            Tile::NorthSouth,
            Tile::EastWest,
            Tile::NorthEast,
            Tile::NorthWest,
            Tile::SouthEast,
            Tile::SouthWest,
        ]
        .into_iter()
    }
    fn char(&self) -> char {
        *self as u8 as char
    }
}

struct Data {
    grid: HashMap<(isize, isize), Tile>,
    start: (isize, isize),
}

impl Data {
    fn neighbours<'a>(
        &'a self,
        pos: (isize, isize),
    ) -> impl Iterator<Item = ((isize, isize), &'a Tile)> + 'a {
        [(0, 1), (0, -1), (1, 0), (-1, 0)]
            .into_iter()
            .filter_map(move |offset| {
                let neighbour = (pos.0 + offset.0, pos.1 + offset.1);
                self.grid.get(&neighbour).and_then(|tile| {
                    // dbg!(neighbour, tile);
                    if tile != &Tile::Ground {
                        Some((neighbour, tile))
                    } else {
                        None
                    }
                })
            })
    }

    fn bounds(&self) -> (isize, isize, isize, isize) {
        self.grid.keys().fold(
            (isize::MAX, isize::MAX, isize::MIN, isize::MIN),
            |(min_x, min_y, max_x, max_y), (x, y)| {
                (min_x.min(*x), min_y.min(*y), max_x.max(*x), max_y.max(*y))
            },
        )
    }
}

fn parse_data(input: &str) -> Data {
    let mut data = Data {
        grid: HashMap::new(),
        start: (0, 0),
    };
    for (y, line) in input.lines().enumerate() {
        for (x, c) in line.chars().enumerate() {
            let tile = match c {
                'S' => Tile::Start,
                '|' => Tile::NorthSouth,
                '-' => Tile::EastWest,
                'L' => Tile::NorthEast,
                'J' => Tile::NorthWest,
                'F' => Tile::SouthEast,
                '7' => Tile::SouthWest,
                '.' => Tile::Ground,
                _ => panic!("Unknown tile: {}", c),
            };
            if tile == Tile::Start {
                data.start = (x as isize, y as isize);
            }
            data.grid.insert((x as isize, y as isize), tile);
        }
    }

    // the grid is populated, now we need to identify what type of pipe the start is
    // we can do this by looking at the surrounding tiles
    // the start tile should only connect to exactly two other tiles

    // the neighbours that are touching the start tile
    // println!("Calculating starting position {:?} neighbours", &data.start);
    let start_neighbours = data
        .neighbours(data.start)
        .filter(|(n_pos, n_tile)| {
            // dbg!(n_pos, n_tile);
            n_tile
                .offsets()
                .and_then(|mut offsets| {
                    // Check if any offset meets the condition
                    offsets
                        .any(|(dx, dy)| {
                            // dbg!(n_pos.0 + dx, n_pos.1 + dy);
                            n_pos.0 + dx == data.start.0 && n_pos.1 + dy == data.start.1
                        })
                        .then(|| ())
                })
                .is_some()
        })
        .collect_vec();
    // dbg!(start_neighbours.clone());

    // the tiles that can connect to the touching neighbours
    // this should match exactly 1 tile
    let candidates: Option<(Tile,)> = Tile::iter_pipes()
        .filter(|pipe| {
            // all of the pipe offsets should be in the start neighbours
            pipe.offsets().unwrap().all(|(dx, dy)| {
                start_neighbours
                    .iter()
                    .any(|(n_pos, _)| n_pos.0 == data.start.0 + dx && n_pos.1 == data.start.1 + dy)
            })
        })
        .collect_tuple();
    data.grid.insert(data.start, candidates.unwrap().0);
    // dbg!(&data.grid);
    data
}

fn trace_loop(data: &Data) -> Vec<((isize, isize), usize)> {
    let mut visited = HashSet::new();
    let mut tile = data.grid.get(&data.start).unwrap();
    let mut pos = data.start;
    let mut path = vec![];
    let mut step = 0;
    loop {
        visited.insert(pos);
        path.push((pos, step));
        step += 1;
        let next = tile
            .offsets()
            .unwrap()
            .find(|(dx, dy)| {
                let next_pos = (pos.0 + dx, pos.1 + dy);
                // dbg!(next_pos);
                !visited.contains(&next_pos) || next_pos == data.start && path.len() > 2
            })
            .expect("should have a next tile");
        pos = (pos.0 + next.0, pos.1 + next.1);
        tile = data.grid.get(&pos).unwrap();
        if pos == data.start {
            break;
        }
    }
    path
}

fn part1(input: &str) -> usize {
    let data = parse_data(input);
    let path = trace_loop(&data);
    path.len() / 2
}

fn part2(input: &str) -> usize {
    let data = parse_data(input);
    let path: HashMap<(isize, isize), usize> = trace_loop(&data).into_iter().collect();

    // let's make a bigger (3x) version of the map
    // each tile will get expanded
    // L would become
    //
    // . | .
    // . L -
    // . . .

    let (min_x, min_y, max_x, max_y) = data.bounds();
    let mut grid: HashMap<(isize, isize), Tile> = HashMap::new();
    for (pos, tile) in path.keys().map(|k| (k,data.grid.get(k).unwrap())) {
        let (x, y) = *pos;
        let (x, y) = (x * 3, y * 3);
        grid.insert((x, y), *tile);
        tile.offsets().unwrap().for_each(|(dx, dy)| {
            match dx {
                1 => {
                    grid.insert((x + 1, y), Tile::EastWest);
                }
                -1 => {
                    grid.insert((x - 1, y), Tile::EastWest);
                }
                _ => {}
            }
            match dy {
                1 => {
                    grid.insert((x, y + 1), Tile::NorthSouth);
                }
                -1 => {
                    grid.insert((x, y - 1), Tile::NorthSouth);
                }
                _ => {}
            }
        });
    }

    // fill defaults and display the map
    for y in min_y * 3..=max_y * 3 {
        for x in min_x * 3..=max_x * 3 {
            if !grid.contains_key(&(x, y)) {
                grid.insert((x, y), Tile::Ground);
            }
            // print!(
            //     "{}",
            //     grid.get(&(x, y)).unwrap().char()
            // );
        }
        // println!();
    }

    // now we flood fill starting from -1, 0 (outside)
    // update the map with an X for each tile that is visited
    let mut visited = HashSet::new();
    let mut queue = VecDeque::new();
    // we want to start from all positions out of bounds of the original map
    for x in min_x * 3..=max_x * 3 {
        queue.push_back((x, min_y * 3 - 1));
        queue.push_back((x, max_y * 3 + 1));
    }
    for y in min_y * 3..=max_y * 3 {
        queue.push_back((min_x * 3 - 1, y));
        queue.push_back((max_x * 3 + 1, y));
    }
    while let Some(pos) = queue.pop_front() {
        if visited.contains(&pos) {
            continue;
        }
        visited.insert(pos);
        [(-1,0),(1,0),(0,1),(0,-1)].iter().for_each(|(dx, dy)| {
            match grid.get(&(pos.0 + dx, pos.1 + dy)) {
                Some(Tile::Ground) => {
                    grid.insert((pos.0 + dx, pos.1 + dy), Tile::Flooded);
                    queue.push_back((pos.0 + dx, pos.1 + dy));
                }
                _ => {}
            }
        });
    }

    // display the flooded map
    // println!();
    // for y in min_y * 3..=max_y * 3 {
    //     for x in min_x * 3..=max_x * 3 {
    //         print!(
    //             "{}",
    //             grid.get(&(x, y)).unwrap().char()
    //         );
    //     }
    //     println!();
    // }

    // count the non-flooded tiles
    data.grid.iter().filter(|(pos, _tile)| {
        match grid.get(&(pos.0*3, pos.1*3)) {
            Some(Tile::Ground) => true,
            _ => false,
        }
    }).count()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_example() {
        let input = include_str!("./example.txt");
        let data = parse_data(input);
        assert_eq!(data.start, (1, 1));
        assert_eq!(data.grid.get(&data.start).unwrap(), &Tile::SouthEast);
        assert_eq!(part1(input), 4);
    }
    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 6846);
    }

    #[test]
    fn part2_example() {
        let input = include_str!("./example.txt");
        assert_eq!(part2(input), 1);
    }
    #[test]
    fn part2_example_1() {
        let input = include_str!("./example_p2_1.txt");
        assert_eq!(part2(input), 4);
    }
    #[test]
    fn part2_example_2() {
        let input = include_str!("./example_p2_2.txt");
        assert_eq!(part2(input), 8);
    }

    #[test]
    fn test_part2() {
        assert!(part2(include_str!("./input.txt")) == 325);
    }
}
