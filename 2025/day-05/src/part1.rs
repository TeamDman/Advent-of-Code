use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct IngredientId(u64);

impl IngredientId {
    fn parse(s: &str) -> eyre::Result<Self> {
        let s = s.trim();
        let v = s.parse::<u64>().map_err(|e| eyre::eyre!(e))?;
        Ok(Self(v))
    }
}

impl fmt::Display for IngredientId {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IdRange {
    start: IngredientId,
    end: IngredientId,
}

impl IdRange {
    pub fn parse(s: &str) -> eyre::Result<Self> {
        let s = s.trim();
        let mut parts = s.split('-');
        let start = parts
            .next()
            .ok_or_else(|| eyre::eyre!("missing start"))?
            .trim();
        let end = parts
            .next()
            .ok_or_else(|| eyre::eyre!("missing end"))?
            .trim();
        if parts.next().is_some() {
            return Err(eyre::eyre!("invalid range format: {}", s));
        }
        let start = IngredientId::parse(start)?;
        let end = IngredientId::parse(end)?;
        Ok(Self { start, end })
    }
}

pub trait Contains<T> {
    fn contains(&self, value: &T) -> bool;
}

impl Contains<IngredientId> for IdRange {
    fn contains(&self, value: &IngredientId) -> bool {
        self.start.0 <= value.0 && value.0 <= self.end.0
    }
}

#[derive(Debug, Clone, Default)]
pub struct FreshRanges {
    ranges: Vec<IdRange>,
}

impl FreshRanges {
    pub fn new(ranges: Vec<IdRange>) -> Self {
        Self { ranges }
    }
    fn is_fresh(&self, id: &IngredientId) -> bool {
        self.ranges.iter().any(|r| r.contains(id))
    }

    pub fn canonicalize(&mut self) {
        if self.ranges.is_empty() {
            return;
        }
        // Convert to tuples and sort
        let mut tuples: Vec<(u64, u64)> = self.ranges.iter().map(|r| r.as_tuple()).collect();
        tuples.sort_unstable_by_key(|t| t.0);

        // Merge overlapping/adjacent ranges
        let mut merged: Vec<(u64, u64)> = Vec::with_capacity(tuples.len());
        let mut cur = tuples[0];
        for &(s, e) in tuples.iter().skip(1) {
            if s <= cur.1.saturating_add(1) {
                if e > cur.1 {
                    cur.1 = e;
                }
            } else {
                merged.push(cur);
                cur = (s, e);
            }
        }
        merged.push(cur);

        // Replace ranges with merged IdRange values
        self.ranges = merged
            .into_iter()
            .map(|(s, e)| IdRange { start: IngredientId(s), end: IngredientId(e) })
            .collect();
    }

    pub fn into_id_iter(mut self) -> FreshIdIter {
        self.canonicalize();
        FreshIdIter::new(self.ranges)
    }

}

pub struct FreshIdIter {
    ranges: Vec<IdRange>,
    idx: usize,
    current: u64,
    remaining: usize,
}

impl FreshIdIter {
    fn new(ranges: Vec<IdRange>) -> Self {
        let remaining = ranges
            .iter()
            .map(|r| ((r.end.0 as u128).saturating_sub(r.start.0 as u128) + 1) as usize)
            .sum();
        let (idx, current) = if ranges.is_empty() {
            (0usize, 0u64)
        } else {
            (0usize, ranges[0].start.0)
        };
        Self { ranges, idx, current, remaining }
    }
}

impl Iterator for FreshIdIter {
    type Item = IngredientId;
    fn next(&mut self) -> Option<Self::Item> {
        if self.idx >= self.ranges.len() {
            return None;
        }
        let e = self.ranges[self.idx].end.0;
        if self.current > e {
            // move to next range
            self.idx += 1;
            if self.idx >= self.ranges.len() {
                return None;
            }
            let ns = self.ranges[self.idx].start.0;
            self.current = ns;
            return self.next();
        }
        let id = IngredientId(self.current);
        self.current = self.current.saturating_add(1);
        self.remaining = self.remaining.saturating_sub(1);
        Some(id)
    }

    fn size_hint(&self) -> (usize, Option<usize>) {
        (self.remaining, Some(self.remaining))
    }
}

impl ExactSizeIterator for FreshIdIter {}

impl IntoIterator for FreshRanges {
    type Item = IngredientId;
    type IntoIter = FreshIdIter;
    fn into_iter(self) -> Self::IntoIter {
        self.into_id_iter()
    }
}

impl IdRange {
    pub fn as_tuple(&self) -> (u64, u64) {
        (self.start.0, self.end.0)
    }
}

pub fn part_1(input: &'static str) -> eyre::Result<usize> {
    let mut ranges = Vec::new();
    let mut ids = Vec::new();
    let mut after_blank = false;

    for line in input.lines() {
        let l = line.trim();
        if l.is_empty() {
            after_blank = true;
            continue;
        }
        if !after_blank {
            ranges.push(IdRange::parse(l)?);
        } else {
            ids.push(IngredientId::parse(l)?);
        }
    }

    let fresh_ranges = FreshRanges { ranges };
    let fresh_count = ids.iter().filter(|id| fresh_ranges.is_fresh(id)).count();
    Ok(fresh_count)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "3-5\n10-14\n16-20\n12-18\n\n1\n5\n8\n11\n17\n32\n";
        assert_eq!(part_1(input)?, 3);
        Ok(())
    }

    #[test]
    pub fn canonicalize_iter_len() -> eyre::Result<()> {
        let ranges = vec![
            IdRange::parse("3-5")?,
            IdRange::parse("10-14")?,
            IdRange::parse("16-20")?,
            IdRange::parse("12-18")?,
        ];
        let mut fr = FreshRanges::new(ranges);
        fr.canonicalize();
        let count_from_iter = fr.clone().into_iter().len();
        assert_eq!(count_from_iter, 14);
        Ok(())
    }
}
