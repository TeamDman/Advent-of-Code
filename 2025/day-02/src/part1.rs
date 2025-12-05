use eyre::bail;

struct Pair {
    left: Id,
    right: Id,
}
impl IntoIterator for Pair {
    type Item = Id;
    type IntoIter = std::array::IntoIter<Id, 2>;
    fn into_iter(self) -> Self::IntoIter {
        [self.left, self.right].into_iter()
    }
}
struct Id(&'static str);
impl Id {
    fn validate_str(s: &str) -> eyre::Result<()> {
        // if starts with '0', return valid
        if s.starts_with('0') {
            return Ok(());
        }
        // If the ID is composed of some sequence repeated twice, it's invalid.
        let len = s.len();
        if len % 2 == 0 && len > 0 {
            let mid = len / 2;
            if &s[..mid] == &s[mid..] {
                bail!("Invalid ID: repeated sequence {}", &s[..mid]);
            }
        }
        Ok(())
    }
}
impl TryFrom<Id> for usize {
    type Error = eyre::Error;
    fn try_from(value: Id) -> Result<Self, Self::Error> {
        value.0.parse::<usize>().map_err(Into::into)
    }
}

pub fn part_1(input: &'static str) -> eyre::Result<usize> {
    let pairs = input.split(',').map(|chunk| {
        let mut parts = chunk.split('-');
        let left = parts
            .next()
            .map(|b| Id(b))
            .ok_or_else(|| eyre::eyre!("Missing left ID"))?;
        let right = parts
            .next()
            .map(|b| Id(b))
            .ok_or_else(|| eyre::eyre!("Missing right ID"))?;
        eyre::Ok(Pair { left, right })
    });
    let mut answer = 0usize;
    for pair in pairs {
        let Pair { left, right } = pair?;
        let left_num = usize::try_from(left)?;
        let right_num = usize::try_from(right)?;
        for n in left_num..=right_num {
            let s = n.to_string();
            if Id::validate_str(&s).is_err() {
                answer += n;
            }
        }
    }
    Ok(answer)
}

#[cfg(test)]
mod test {
    use super::*;

    #[test]
    pub fn sample_example() -> eyre::Result<()> {
        let input = "11-22,95-115,998-1012,1188511880-1188511890,222220-222224,1698522-1698528,446443-446449,38593856-38593862,565653-565659,824824821-824824827,2121212118-2121212124";
        assert_eq!(part_1(input)?, 1227775554);
        Ok(())
    }

    #[test]
    pub fn detects_simple_and_complex_repeats() -> eyre::Result<()> {
        // 11 and 6464 and 1010 are invalid IDs for the puzzle definition.
        let input = "1-11,6464-6464,1000-1010";
        assert_eq!(part_1(input)?, 11 + 6464 + 1010);
        Ok(())
    }
}