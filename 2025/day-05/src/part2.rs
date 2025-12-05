use crate::part1::{FreshRanges, IdRange};

pub fn part_2(input: &'static str) -> eyre::Result<usize> {
    let mut ranges = Vec::new();
    for line in input.lines() {
        let l = line.trim();
        if l.is_empty() {
            break;
        }
        ranges.push(IdRange::parse(l)?);
    }
    let fr = FreshRanges::new(ranges);
    // Use iterator len to compute union size without enumerating all IDs
    let count = fr.into_iter().len();
    Ok(count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";
        assert_eq!(part_2(input)?, 14);
        Ok(())
    }
}
