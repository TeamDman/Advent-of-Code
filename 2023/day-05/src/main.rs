use std::ops::Range;

use common::prelude::*;
use itertools::Itertools;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

#[derive(Debug, PartialEq)]
struct RangeMapping {
    destination_start: usize,
    source_start: usize,
    range_length: usize,
}

impl RangeMapping {
    fn new(destination_start: usize, source_start: usize, range_length: usize) -> Self {
        RangeMapping {
            destination_start,
            source_start,
            range_length,
        }
    }

    fn map_number(&self, number: usize) -> Option<usize> {
        if number >= self.source_start && number < self.source_start + self.range_length {
            Some(self.destination_start + (number - self.source_start))
        } else {
            None
        }
    }

    fn map_range(&self, range: Range<usize>) -> (Vec<Range<usize>>, Vec<Range<usize>>) {
        let source_end = self.source_start + self.range_length;
        if range.end <= self.source_start {
            // Entire range is before the source range
            return (vec![range], vec![]);
        } else if range.start >= source_end {
            // Entire range is after the source range
            return (vec![range], vec![]);
        }
        let mut unused_ranges = Vec::new();
        let mut result_ranges = Vec::new();
        // Add unmapped range before the overlap (if any)
        if range.start < self.source_start {
            unused_ranges.push(range.start..self.source_start);
        }

        // Calculate and map the overlap range
        let overlap_start = std::cmp::max(range.start, self.source_start);
        let overlap_end = std::cmp::min(range.end, source_end);
        let mapped_start = self.destination_start + (overlap_start - self.source_start);
        let mapped_end = mapped_start + (overlap_end - overlap_start);
        result_ranges.push(mapped_start..mapped_end);

        // Add unmapped range after the overlap (if any)
        if range.end > source_end {
            unused_ranges.push(source_end..range.end);
        }

        (unused_ranges, result_ranges)
    }
}

fn process_mappings<'a>(
    chunks: impl Iterator<Item = &'a str>,
) -> Vec<(&'a str, Vec<RangeMapping>)> {
    chunks
        .map(|chunk| {
            let mut lines = chunk.lines();
            let name = lines.next().unwrap().rsplit_once(" ").unwrap().0;
            let mappings = lines.map(|line| {
                let mut numbers = line.split_ascii_whitespace().map(|n| {
                    n.parse::<usize>()
                        .expect(format!("{} should be a number", n).as_str())
                });
                let range = RangeMapping::new(
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                    numbers.next().unwrap(),
                );
                range
            });
            // println!("{}: {} mappings", name, mappings.clone().count());
            (name, mappings.collect::<Vec<RangeMapping>>())
        })
        .collect_vec()
}
fn apply_range_mappings(
    named_mapping_lists: &Vec<(&str, Vec<RangeMapping>)>,
    range: Range<usize>,
) -> Vec<Range<usize>> {
    let mut inputs = vec![range];
    for (_chunk_name, chunk_list) in named_mapping_lists.iter() {
        println!("Processing chunk: {}", _chunk_name);
        let mut chunk_inputs = inputs.clone();
        let mut chunk_unused: Vec<Range<usize>> = vec![];
        let mut chunk_results: Vec<Range<usize>> = vec![];
        for entry in chunk_list.iter() {
            for x in chunk_inputs.iter() {
                let (unmapped, mapped) = entry.map_range(x.clone());
                println!("chunk input {:?} became {:?}, {:?}", x.clone(), unmapped.clone(), mapped.clone());
                chunk_unused.extend(unmapped);
                chunk_results.extend(mapped);
            }
            chunk_inputs = chunk_unused.clone();
            chunk_unused.clear();
        }
        inputs = [chunk_inputs, chunk_results].concat();
    }
    inputs
    // maps.iter().fold(vec![range], |acc, (_, range_mapping_vec)| {
    //     let mut result = Vec::new();

    //     for range in acc {
    //         if let Some((first, mapped)) = range_mapping_vec.iter()
    //             .find_map(|range_mapping| {
    //                 let mapped_ranges = range_mapping.map_range(range.clone());
    //                 if mapped_ranges.len() > 1 || (mapped_ranges.len() == 1 && mapped_ranges[0] != range) {
    //                     Some((range.clone(), mapped_ranges))
    //                 } else {
    //                     None
    //                 }
    //             }) {
    //                 result.extend(mapped);
    //                 break;
    //             } else {
    //                 result.push(range);
    //             }
    //     }

    //     result
    // })
}

fn part1(input: &str) -> usize {
    let mut chunks = input.split("\n\n");
    let seeds = chunks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|n| {
            n.parse::<usize>()
                .expect(format!("{} should be a number", n).as_str())
        })
        .collect::<Vec<usize>>();
    let maps = process_mappings(chunks);
    let min_location = seeds
        .iter()
        .map(|seed| {
            maps.iter().fold(*seed, |x, (_, ranges)| {
                ranges
                    .iter()
                    .map(|range| range.map_number(x))
                    .filter(Option::is_some)
                    .map(Option::unwrap)
                    .next()
                    .unwrap_or(x)
            })
        })
        .min()
        .unwrap();
    min_location
}

fn part2(input: &str) -> usize {
    let mut chunks = input.split("\n\n");

    // Parse the initial seed ranges
    let seed_numbers = chunks
        .next()
        .unwrap()
        .split_once(": ")
        .unwrap()
        .1
        .split_ascii_whitespace()
        .map(|n| {
            n.parse::<usize>()
                .expect(format!("{} should be a number", n).as_str())
        })
        .collect::<Vec<_>>();

    // Process the mappings
    let maps = process_mappings(chunks);

    // Generate all seed numbers from the ranges and find the minimum location
    seed_numbers
        .chunks(2)
        .map(|chunk| {
            let start = chunk[0];
            let length = chunk[1];
            start..start + length
        })
        .flat_map(|range| apply_range_mappings(&maps, range))
        .map(|range| range.start)
        .min()
        .unwrap()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");
        assert_eq!(part1(input), 1181555926);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");
        assert_eq!(part2(input), 37806486);
    }

    const EXAMPLE: &str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";

    #[test]
    fn part1_example() {
        assert_eq!(part1(EXAMPLE), 35);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(EXAMPLE), 46);
    }

    #[test]
    fn part2_diagnose() {
        assert_eq!(
            part2(
                "seeds: 1 5 200 5

first map:
100 2 5

second map:
900 500 100"
            ),
            1
        );
        assert_eq!(
            part2(
                "seeds: 1 5 200 5

first map:
100 1 5"
            ),
            100
        );
        assert_eq!(
            part2(
                "seeds: 1 5 200 5

first map:
100 2 5

second map:
900 1 5"
            ),
            100
        );
    }

    #[test]
    fn part2_diagnosis_1() {
        let input = "first map:
100 2 5
500 2 5

second map:
900 1 5";
        let maps = process_mappings(input.split("\n\n"));
        assert_eq!(
            maps[0],
            (
                "first",
                vec![RangeMapping::new(100, 2, 5), RangeMapping::new(500, 2, 5)]
            )
        );
        assert_eq!(maps[1], ("second", vec![RangeMapping::new(900, 1, 5)]));
    }

    #[test]
    fn part2_diagnosis_2() {
        let input = "first map:
100 2 5
500 2 5";
        let maps = process_mappings(input.split("\n\n"));
        assert_eq!(apply_range_mappings(&maps, 1..6), vec![1..2, 100..104])
    }

    #[test]
    fn part2_diagnosis_3() {
        assert_eq!(
            RangeMapping::new(100, 2, 5).map_range(1..6),
            (vec![1..2], vec![100..104]),
        );
    }

    #[test]
    fn test_map_range_fully_within() {
        let mapping = RangeMapping::new(10, 5, 5); // Maps 5-9 to 10-14
        assert_eq!(mapping.map_range(6..8), (vec![], vec![11..13]));
    }

    #[test]
    fn test_map_range_partial_overlap() {
        let mapping = RangeMapping::new(10, 5, 5); // Maps 5-9 to 10-14
        assert_eq!(mapping.map_range(4..7), (vec![4..5], vec![10..12]));
    }

    #[test]
    fn test_map_range_no_overlap() {
        let mapping = RangeMapping::new(10, 5, 5); // Maps 5-9 to 10-14
        assert_eq!(mapping.map_range(1..4), (vec![1..4], vec![]));
    }

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
