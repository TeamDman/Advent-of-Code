pub fn part_1(input: &'static str) -> eyre::Result<usize> {
    let _ = input;
    Ok(0)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "";
        assert_eq!(part_1(input)?, 0);
        Ok(())
    }
}
