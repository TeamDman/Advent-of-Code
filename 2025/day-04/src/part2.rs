use crate::part1::{Grid, Paper, Neighbors};

pub fn part_2(input: &'static str) -> eyre::Result<usize> {
    let mut grid: Grid = input.parse()?;
    let mut total_removed = 0usize;
    loop {
        // clone to snapshot the current configuration for neighbor counting
        let snapshot = Grid {
            width: grid.width,
            height: grid.height,
            cells: grid.cells.clone(),
        };
        let mut to_remove = Vec::new();
        for y in 0..grid.height {
            for x in 0..grid.width {
                let idx = y * grid.width + x;
                if grid.cells[idx].is_roll() {
                    let n = snapshot.neighbors_count(x, y);
                    if n < 4 {
                        to_remove.push(idx);
                    }
                }
            }
        }
        if to_remove.is_empty() {
            break;
        }
        total_removed += to_remove.len();
        for idx in to_remove {
            grid.cells[idx] = Paper::empty();
        }
    }
    Ok(total_removed)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "..@@.@@@@.\n@@@.@.@.@@\n@@@@@.@.@@\n@.@@@@..@.\n@@.@@@@.@@\n.@@@@@@@.@\n.@.@.@.@@@\n@.@@@.@@@@\n.@@@@@@@@.\n@.@.@@@.@.";
        assert_eq!(part_2(input)?, 43);
        Ok(())
    }
}
