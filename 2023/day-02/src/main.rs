use common::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}

struct Round {
    blue: usize,
    green: usize,
    red: usize,
}

fn get_games(input: &str) -> Vec<(usize, Vec<Round>)> {
    input
        .lines()
        .map(|line| {
            let (game_header, game_body) = line
                .split_once(": ")
                .expect("Input lines begin with `Game $n$: `");
            let game_id = game_header
                .split(" ")
                .nth(1)
                .unwrap()
                .parse::<usize>()
                .unwrap();
            // println!("processing body: {}", game_body);
            let rounds: Vec<Round> = game_body
                .split("; ")
                .map(|round| {
                    let mut acc = Round {
                        red: 0,
                        green: 0,
                        blue: 0,
                    };
                    // println!("Processing round {}", round);
                    round.split(", ").for_each(|entry| {
                        let (count, color) = entry.split_once(" ").expect(
                            format!("Entry \"{}\" should be a number and a colour", entry).as_str(),
                        );
                        let count = count.parse::<usize>().unwrap();
                        match color {
                            "red" => acc.red += count,
                            "green" => acc.green += count,
                            "blue" => acc.blue += count,
                            _ => panic!("Invalid color"),
                        }
                    });
                    acc
                })
                .collect();
            (game_id, rounds)
        })
        .collect()
}

fn part1(input: &str) -> usize {
    let constraint = Round {
        red: 12,
        green: 13,
        blue: 14,
    };
    get_games(input)
        .iter()
        .filter(|(_, rounds)| {
            rounds.iter().all(|round| {
                round.blue <= constraint.blue
                    && round.green <= constraint.green
                    && round.red <= constraint.red
            })
        })
        .map(|(game_id, _)| game_id)
        .sum()
}

fn part2(input: &str) -> usize {
    get_games(input)
        .iter()
        .map(|(game_id, rounds)| {
            // fold rounds into the highest number for red, green, blue
            let mut acc = Round {
                red: 0,
                green: 0,
                blue: 0,
            };
            rounds.iter().for_each(|round| {
                acc.red = acc.red.max(round.red);
                acc.green = acc.green.max(round.green);
                acc.blue = acc.blue.max(round.blue);
            });
            (game_id, acc)
        })
        .map(|(_, round)| {
            // multiply the highest number for red, green, blue
            round.red * round.green * round.blue
        })
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        let input = include_str!("./input.txt");
        assert_eq!(part1(input), 2416);
    }

    #[test]
    fn test_part2() {
        let input = include_str!("./input.txt");
        assert_eq!(part2(input), 63307);
    }

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }

    #[test]
    fn part2_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";

        let line_answers = [48, 12, 1560, 630, 36];
        input.lines().zip(line_answers.iter()).for_each(|(line, answer)| {
            assert_eq!(part2(line), *answer, "Line: {}", line);
        });

        assert_eq!(part2(input), 2286);
    }

    #[test]
    fn part2_badanswer() {
        assert!(
            part2(include_str!("./input.txt")) < 582239,
            "Answer too high"
        );
    }
}
