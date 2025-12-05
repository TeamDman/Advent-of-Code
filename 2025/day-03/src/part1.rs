
pub struct BatteryBank(&'static str);

impl BatteryBank {
    pub fn largest_joltage(&self) -> usize {
        let s = self.0.as_bytes();
        let mut max = 0usize;
        for i in 0..s.len() {
            for j in (i + 1)..s.len() {
                let a = (s[i] - b'0') as usize;
                let b = (s[j] - b'0') as usize;
                let val = 10 * a + b;
                if val > max {
                    max = val;
                }
            }
        }
        max
    }
}

pub fn part_1(input: &'static str) -> eyre::Result<usize> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let bank = BatteryBank(line);
        sum += bank.largest_joltage();
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
        let result = part_1(input)?;
        assert_eq!(result, 357);
        Ok(())
    }
}