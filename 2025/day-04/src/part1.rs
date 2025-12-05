use std::str::FromStr;

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(crate) struct Paper(bool);
impl From<char> for Paper {
    fn from(c: char) -> Self {
        match c {
            '@' => Paper(true),
            _ => Paper(false),
        }
    }
}
impl Paper {
    pub(crate) fn is_roll(&self) -> bool {
        self.0
    }
    pub(crate) fn empty() -> Self {
        Paper(false)
    }
    pub(crate) fn roll() -> Self {
        Paper(true)
    }
}

#[derive(Debug)]
pub(crate) struct Grid {
    pub(crate) width: usize,
    pub(crate) height: usize,
    pub(crate) cells: Vec<Paper>,
}

// Simple errors returned via eyre, no extra crates required

impl FromStr for Grid {
    type Err = eyre::Error;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut lines = s.lines().filter(|l| !l.is_empty()).peekable();
        if lines.peek().is_none() {
            return Err(eyre::eyre!("empty input"));
        }
        let first = lines.peek().unwrap();
        let width = first.len();
        let mut cells = Vec::new();
        let mut height = 0usize;
        for line in lines {
            if line.len() != width {
                return Err(eyre::eyre!("inconsistent row widths"));
            }
            height += 1;
            for c in line.chars() {
                cells.push(Paper::from(c));
            }
        }
        Ok(Grid {
            width,
            height,
            cells,
        })
    }
}

pub(crate) trait Neighbors {
    fn neighbors_count(&self, x: usize, y: usize) -> usize;
}

impl Neighbors for Grid {
    fn neighbors_count(&self, x: usize, y: usize) -> usize {
        let mut count = 0usize;
        let w = self.width as isize;
        let h = self.height as isize;
        let x0 = x as isize;
        let y0 = y as isize;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x0 + dx;
                let ny = y0 + dy;
                if nx >= 0 && nx < w && ny >= 0 && ny < h {
                    let idx = (ny as usize) * self.width + (nx as usize);
                    if self.cells[idx].is_roll() {
                        count += 1;
                    }
                }
            }
        }
        count
    }
}

pub fn part_1(input: &'static str) -> eyre::Result<usize> {
    let grid: Grid = input.parse()?;
    let mut accessible = 0usize;
    for y in 0..grid.height {
        for x in 0..grid.width {
            let idx = y * grid.width + x;
            if grid.cells[idx].is_roll() {
                let n = grid.neighbors_count(x, y);
                if n < 4 {
                    accessible += 1;
                }
            }
        }
    }
    Ok(accessible)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(part_1(input)?, 13);
        Ok(())
    }
}
