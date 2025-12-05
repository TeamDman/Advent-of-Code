use std::ops::Add;
use std::str::FromStr;

pub fn part_1(content: &str) -> eyre::Result<usize> {
    struct Dial {
        // 0 to 99, wrapping
        position: usize,
    }
    enum Direction {
        Left,
        Right,
    }
    struct Movement {
        direction: Direction,
        steps: usize,
    }
    impl Dial {
        fn new() -> Self {
            Dial { position: 50 }
        }
    }
    impl Add<Movement> for Dial {
        type Output = Dial;

        fn add(self, movement: Movement) -> Dial {
            let new_position = match movement.direction {
                Direction::Left => (self.position + 100 - (movement.steps % 100)) % 100,
                Direction::Right => (self.position + movement.steps) % 100,
            };
            Dial {
                position: new_position,
            }
        }
    }
    impl FromStr for Movement {
        type Err = eyre::Error;
        fn from_str(s: &str) -> Result<Self, Self::Err> {
            let direction = match &s[0..1] {
                "L" => Direction::Left,
                "R" => Direction::Right,
                _ => return Err(eyre::eyre!("Invalid direction")),
            };
            let steps = s[1..].parse::<usize>()?;
            Ok(Movement { direction, steps })
        }
    }

    let mut dial = Dial::new();
    let mut count_of_times_dial_at_zero = 0;
    for movement in content
        .lines()
        .into_iter()
        .map(|line| line.parse::<Movement>())
        .collect::<eyre::Result<Vec<Movement>>>()?
    {
        dial = dial + movement;
        if dial.position == 0 {
            count_of_times_dial_at_zero += 1;
        }
    }
    Ok(count_of_times_dial_at_zero)
}
