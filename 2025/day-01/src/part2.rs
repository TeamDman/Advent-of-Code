use std::ops::Add;
use std::str::FromStr;

struct Dial {
    // 0 to 99, wrapping
    position: usize,
    // number of times the dial has crossed position 0
    clicks: usize,
}
#[derive(Copy, Clone)]
enum Direction {
    Left,
    Right,
}
#[derive(Copy, Clone)]
struct Movement {
    direction: Direction,
    steps: usize,
}
impl Dial {
    fn new() -> Self {
        Dial { position: 50, clicks: 0 }
    }
}
impl Add<Movement> for Dial {
    type Output = Dial;

    fn add(self, movement: Movement) -> Dial {
        let old_position = self.position;
        let new_position = match movement.direction {
            Direction::Left => (self.position + 100 - (movement.steps % 100)) % 100,
            Direction::Right => (self.position + movement.steps) % 100,
        };
        // compute wraps (clicks) for this movement using modular counting
        // A wrap occurs for step k (1..=steps) when (old_position +/- k) % 100 == 0.
        let mut wraps = 0usize;
        match movement.direction {
            Direction::Right => {
                let r = (100usize - (old_position % 100)) % 100; // step congruence needed
                if r == 0 {
                    wraps = movement.steps / 100;
                } else if movement.steps >= r {
                    wraps = ((movement.steps - r) / 100) + 1;
                }
            }
            Direction::Left => {
                let r = old_position % 100; // step congruence needed for left
                if r == 0 {
                    wraps = movement.steps / 100;
                } else if movement.steps >= r {
                    wraps = ((movement.steps - r) / 100) + 1;
                }
            }
        }
        Dial {
            position: new_position,
            clicks: self.clicks + wraps,
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

pub fn part_2(content: &str) -> eyre::Result<usize> {
    let mut dial = Dial::new();
    for movement in content
        .lines()
        .into_iter()
        .map(|line| line.parse::<Movement>())
        .collect::<eyre::Result<Vec<Movement>>>()?
    {
        dial = dial + movement;
    }

    Ok(dial.clicks)
}

#[cfg(test)]
mod test {
    use crate::part2::part_2;

    #[test]
    pub fn it_works() -> eyre::Result<()> {
        let answer = part_2("R1000")?;
        assert_eq!(answer, 10);
        Ok(())
    }

    #[test]
    pub fn right_small_wraps() -> eyre::Result<()> {
        assert_eq!(part_2("R50")?, 1);
        assert_eq!(part_2("R49")?, 0);
        assert_eq!(part_2("R100")?, 1);
        assert_eq!(part_2("R150")?, 2);
        Ok(())
    }

    #[test]
    pub fn left_small_wraps() -> eyre::Result<()> {
        assert_eq!(part_2("L50")?, 1);
        assert_eq!(part_2("L49")?, 0);
        assert_eq!(part_2("L100")?, 1);
        assert_eq!(part_2("L150")?, 2);
        Ok(())
    }

    #[test]
    pub fn mixed_sequences() -> eyre::Result<()> {
        assert_eq!(part_2("R99\nR1")?, 1);
        assert_eq!(part_2("L99\nL1")?, 1);
        assert_eq!(part_2("R100\nR100")?, 2);
        assert_eq!(part_2("R50\nL50")?, 1);
        Ok(())
    }

    #[test]
    pub fn zero_steps() -> eyre::Result<()> {
        assert_eq!(part_2("R0")?, 0);
        assert_eq!(part_2("L0")?, 0);
        Ok(())
    }
}
