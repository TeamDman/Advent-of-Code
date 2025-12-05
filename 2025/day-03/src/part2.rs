// No unused imports here; keep signatures compatible with main.rs style

pub struct BatteryBank(&'static str);

impl BatteryBank {
    pub fn largest_joltage_k(&self, k: usize) -> usize {
        let s = self.0.as_bytes();
        let n = s.len();
        if k == 0 || n == 0 {
            return 0;
        }
        let mut idx = 0usize;
        let mut digits = Vec::with_capacity(k);
        for picked in 0..k {
            let remaining_to_pick = k - picked - 1;
            // we must pick at most position end-1 (inclusive)
            let end = n - remaining_to_pick;
            let mut best = b'0';
            let mut best_pos = idx;
            for p in idx..end {
                let d = s[p];
                if d > best {
                    best = d;
                    best_pos = p;
                    if best == b'9' {
                        break;
                    }
                }
            }
            digits.push((best - b'0') as usize);
            idx = best_pos + 1;
        }
        let mut val: usize = 0;
        for d in digits {
            val = val * 10 + d;
        }
        val
    }
}

pub fn part_2(input: &'static str) -> eyre::Result<usize> {
    let mut sum: usize = 0;
    for line in input.lines() {
        let line = line.trim();
        if line.is_empty() {
            continue;
        }
        let bank = BatteryBank(line);
        sum += bank.largest_joltage_k(12);
    }
    Ok(sum)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "987654321111111\n811111111111119\n234234234234278\n818181911112111\n";
        let result = part_2(input)?;
        assert_eq!(result, 3121910778619usize);
        Ok(())
    }

}