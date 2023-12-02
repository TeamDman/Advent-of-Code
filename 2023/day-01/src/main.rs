mod profiler;

fn main() {
    let input = include_str!("./input.txt");
    time_function!(part1,input);
    time_function!(part2,input);
}

fn part1(input: &str) -> usize {
    // for each line, which contains many digits and non-digits
    // find the first and last digit and concat to make a two digit number
    // sum the numbers for the result
    input
        .lines()
        .map(|line| {
            // println!("got line '{}'", line);
            let numbers = line.chars().filter(|c| c.is_digit(10));
            let first = numbers.clone().next().unwrap();
            let last = numbers.last().unwrap();
            first.to_digit(10).unwrap() * 10 + last.to_digit(10).unwrap()
        })
        .fold(0, |acc, x| acc + x as usize)
}

fn part2(input: &str) -> usize {
    // for each line, which contains digits, digit-words, and other characters
    // find the first and last digit and concat to make a two digit number
    // sum the line results for the final result

    let words = vec![
        "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
    ];
    input
        .lines()
        .map(|line| {
            // Find the first digit or digit-word
            let first = find_digit(&line, &words, true);

            // Find the last digit or digit-word
            let last = find_digit(&line, &words, false);

            first * 10 + last
        })
        .fold(0, |acc, x| acc + x as usize)
}

// Function to find a digit or digit-word in a line
fn find_digit(line: &str, words: &Vec<&str>, is_first: bool) -> usize {
    let max_word_length = 5; // Length of the longest word ("seven")
    let line_length = line.len();

    if is_first {
        // Searching from left to right
        for i in 0..line_length {
            let end = std::cmp::min(i + max_word_length, line_length);
            let window = &line[i..end];

            if let Some(digit) = window.chars().next().filter(|c| c.is_digit(10)) {
                return digit.to_digit(10).unwrap() as usize;
            }

            for (j, word) in words.iter().enumerate() {
                if window.starts_with(word) {
                    return j;
                }
            }
        }
    } else {
        // Searching from right to left
        for i in (0..line_length).rev() {
            let start = if i >= max_word_length - 1 {
                i - (max_word_length - 1)
            } else {
                0
            };
            let window = &line[start..=i];

            if let Some(digit) = window.chars().last().filter(|c| c.is_digit(10)) {
                return digit.to_digit(10).unwrap() as usize;
            }

            for (j, word) in words.iter().enumerate() {
                if window.ends_with(word) {
                    return j;
                }
            }
        }
    }

    0 // Return 0 if no digit or digit-word is found
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part1_answer() {
        assert_eq!(part1(include_str!("./input.txt")), 54159);
    }
    #[test]
    fn part2_answer() {
        assert_eq!(part2(include_str!("./input.txt")), 53866);
    }

    #[test]
    fn part1_0() {
        assert_eq!(
            part1(
                "1abc2
pqr3stu8vwx
a1b2c3d4e5f
treb7uchet"
            ),
            142
        );
    }

    #[test]
    fn part2_0() {
        assert_eq!(
            part2(
                "two1nine
eightwothree
abcone2threexyz
xtwone3four
4nineeightseven2
zoneight234
7pqrstsixteen"
            ),
            281
        )
    }

    #[test]
    fn part2_1() {
        assert_eq!(part2("fivetkhfnnx22"), 52);
        assert_eq!(part2("tbvdcsjsvmxtshv3fourseven4kmxvvfour9"), 39);
        assert_eq!(part2("1bnndtnsfjdsevenfivetwo3k85"), 15);
        assert_eq!(part2("69sixnine"), 69);
        assert_eq!(part2("nineninesevenztfggvfkgkzfcm2"), 92);
        assert_eq!(part2("six"), 66);
        assert_eq!(part2("5six"), 56);
        assert_eq!(part2("5"), 55);
        assert_eq!(part2("twone"), 21);
        assert_eq!(
            part2(
                "fivetkhfnnx22
tbvdcsjsvmxtshv3fourseven4kmxvvfour9
1bnndtnsfjdsevenfivetwo3k85"
            ),
            52 + 39 + 15
        );
    }

    fn generate_test_case() -> (usize, usize, String) {
        let mut rng = thread_rng();

        // Step 1: Pick a Pair of Digits
        let digit1 = rng.gen_range(1..=9);
        let digit2 = rng.gen_range(1..=9);

        // Step 2: Replace Digits with Words
        let words = [
            "zero", "one", "two", "three", "four", "five", "six", "seven", "eight", "nine",
        ];
        let pair = vec![
            if rng.gen_bool(0.5) {
                words[digit1].to_string()
            } else {
                digit1.to_string()
            },
            if rng.gen_bool(0.5) {
                words[digit2].to_string()
            } else {
                digit2.to_string()
            },
        ];

        // Step 3 & 5: Generate Random Prefix and Suffix
        let prefix = format!("{}z", generate_random_string(5, &words));
        let suffix = format!("z{}",generate_random_string(5, &words));

        // Step 4: Generate Filler
        let filler = format!("z{}z", generate_random_string(3, &words));

        // Combine all parts
        (
            digit1,
            digit2,
            format!("{}{}{}{}", prefix, pair.join(""), filler, suffix),
        )
    }

    fn generate_random_string(len: usize, exclude: &[&str]) -> String {
        let mut rng = thread_rng();
        let mut result = String::new();
        while result.len() < len {
            let char = rng.sample(Alphanumeric) as char;
            
            // Check for any excluded word formation
            let mut is_excluded = false;
            for excl in exclude {
                if format!("{}{}", result, char).contains(excl) {
                    is_excluded = true;
                    break;
                }
            }
    
            if !char.is_digit(10) && !is_excluded {
                result.push(char);
            }
        }
        result
    }
    
    #[test]
    fn part2_auto_single() {
        println!("BEANS!~");
        let (left, right, test_case) = generate_test_case();
        println!("{} + {} = {} from {}", left, right, left + right, test_case);
    }

    #[test]
    fn part2_auto_many() {
        let mut total_expected_result = 0;
        let mut combined_test_cases = String::new();

        for _ in 0..100000 {
            // Generate 1 to 100 test cases
            let (left, right, test_case) = generate_test_case();
            let expected_result = left * 10 + right;
            total_expected_result += expected_result;

            assert_eq!(
                part2(&test_case),
                expected_result,
                "Failed on test case: {} + {} = {} from {}",
                left,
                right,
                expected_result,
                test_case
            );
            combined_test_cases.push_str(&test_case);
            combined_test_cases.push('\n');
        }

        // Removing the last newline character
        combined_test_cases.pop();

        // Validate part2 output
        let part2_result = part2(&combined_test_cases);
        assert_eq!(
            part2_result, total_expected_result,
            "Failed on combined test cases"
        );
    }

    #[test]
    fn part2_fuzz() {
        assert_eq!(part2("GiMdcsix3htwOuduM"), 63);
    }
}
