use common::prelude::*;

fn main() {
    let input = include_str!("./input.txt");
    time_function!(part1, input);
    time_function!(part2, input);
}

struct Round {
    blue: usize,
    green: usize,
    red: usize,
}

fn part1(input: &str) -> usize {
    let constraint = Round {
        red: 12,
        green: 13,
        blue: 14,
    };

    input
        .lines()
        .map(|line| {
            let (game, rounds) = line
                .split_once(": ")
                .expect("Input lines begin with `Game $n$: `");
            let game_id = game.split(" ").nth(1).unwrap().parse::<usize>().unwrap();

            let mut rounds = rounds.split("; ").map(|round| {
                let (round, _) = round.split_once(" ").unwrap();
                let mut round = round.split(", ");
                let blue = round.next().unwrap().parse::<usize>().unwrap();
                let green = round.next().unwrap().parse::<usize>().unwrap();
                let red = round.next().unwrap().parse::<usize>().unwrap();
                Round { blue, green, red }
            });
            (game_id, rounds)
        })
        .filter(|(game_id, rounds)| {
            rounds.all(|round| {
                round.blue <= constraint.blue
                    && round.green <= constraint.green
                    && round.red <= constraint.red
            })
        })
        .map(|(game_id, rounds)| game_id)
        .sum()
}
fn part2(input: &str) -> usize {
    todo!();
}

mod tests {
    use super::*;

    // #[test]
    // fn test_part1() {
    //     let input = include_str!("./input.txt");
    //     assert_eq!(part1(input), 0);
    // }

    // #[test]
    // fn test_part2() {
    //     let input = include_str!("./input.txt");
    //     assert_eq!(part2(input), 0);
    // }

    #[test]
    fn part1_example() {
        let input = "Game 1: 3 blue, 4 red; 1 red, 2 green, 6 blue; 2 green
Game 2: 1 blue, 2 green; 3 green, 4 blue, 1 red; 1 green, 1 blue
Game 3: 8 green, 6 blue, 20 red; 5 blue, 4 red, 13 green; 5 green, 1 red
Game 4: 1 green, 3 red, 6 blue; 3 green, 6 red; 3 green, 15 blue, 14 red
Game 5: 6 red, 1 blue, 3 green; 2 blue, 1 red, 2 green";
        assert_eq!(part1(input), 8);
    }
}
