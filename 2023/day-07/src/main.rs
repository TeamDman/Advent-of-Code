use std::cmp::Ordering;

use common::prelude::*;
use itertools::Itertools;
use rayon::iter::ParallelIterator;
use rayon::slice::ParallelSliceMut;
use rayon::str::ParallelString;

fn main() {
    let input = include_str!("./input.txt");
    let result = time_function!(part1, input);
    println!("Part 1: {}", result);
    let result = time_function!(part2, input);
    println!("Part 2: {}", result);
}
#[derive(PartialOrd, Ord, PartialEq, Eq, Clone, Copy, Debug)]
enum HandType {
    HighCard,
    OnePair,
    TwoPair,
    ThreeOfAKind,
    FullHouse,
    FourOfAKind,
    FiveOfAKind,
}

fn get_hand_type(hand: &str) -> HandType {
    let mut cards = hand.chars().collect::<Vec<char>>();
    cards.sort_unstable();
    let groups = cards
        .iter()
        .group_by(|&&card| card)
        .into_iter()
        .map(|(card, group)| (card, group.count()))
        .sorted_by(|a, b| b.1.cmp(&a.1).then_with(|| a.0.cmp(&b.0)))
        .map(|(card, count)| (card, count))
        .collect::<Vec<_>>();

    match groups[0].1 {
        5 => HandType::FiveOfAKind,
        4 => HandType::FourOfAKind,
        3 => {
            if groups[1].1 == 2 {
                HandType::FullHouse
            } else {
                HandType::ThreeOfAKind
            }
        }
        2 => {
            if groups[1].1 == 2 {
                HandType::TwoPair
            } else {
                HandType::OnePair
            }
        }
        _ => HandType::HighCard,
    }
}

fn get_joker_hand_type(hand: &str) -> HandType {
    // Check if the hand contains any 'J' (now 'Z') cards
    if hand.contains('Z') {
        let other_cards = [
            'A', 'K', 'Q', 'T', '9', '8', '7', '6', '5', '4', '3', '2', 'Z',
        ];

        // Generate all possible hands by replacing 'Z' with each of the other cards
        let possible_hands = other_cards.iter().map(|&card| {
            hand.chars()
                .map(|h| if h == 'Z' { card } else { h })
                .collect::<String>()
        });

        // Evaluate each possible hand and keep the strongest one
        possible_hands.map(|h| get_hand_type(&h)).max().unwrap()
    } else {
        // If there are no jokers, evaluate the hand as it is
        get_hand_type(&hand)
    }
}

fn card_value(card: char) -> u8 {
    match card {
        'A' => 14,
        'K' => 13,
        'Q' => 12,
        'J' => 11,
        'T' => 10,
        '9' => 9,
        '8' => 8,
        '7' => 7,
        '6' => 6,
        '5' => 5,
        '4' => 4,
        '3' => 3,
        '2' => 2,
        'Z' => 1, // Joker
        _ => 0,   // default case, though it should never be reached with valid input
    }
}

fn card_compare(a: &char, b: &char) -> Ordering {
    card_value(*a).cmp(&card_value(*b))
}
fn hand_order(
    a_hand: &str,
    a_bet: usize,
    a_type: HandType,
    b_hand: &str,
    b_bet: usize,
    b_type: HandType,
) -> Ordering {
    let x = a_type.cmp(&b_type);
    if x != Ordering::Equal {
        // better hand wins
        return x;
    } else {
        // tiebreak logic
        let a_j_count = a_hand.chars().filter(|c| *c == 'Z').count();
        let b_j_count = b_hand.chars().filter(|c| *c == 'Z').count();
        let x = a_j_count.cmp(&b_j_count);
        if x != Ordering::Equal {
            // hand with fewer jokers wins
            return x;
        } else {
            // same number of jokers
            // hand with highest first different card wins
            for (c_a, c_b) in a_hand.chars().zip(b_hand.chars()) {
                let x = card_compare(&c_a, &c_b);
                if x == Ordering::Equal {
                    continue;
                }
                return x;
            }
            return Ordering::Equal;
        }
    }
}
fn part1(input: &str) -> usize {
    let hands = input.lines().map(|line| {
        let (hand, bet) = line.split_once(" ").unwrap();
        (
            hand,
            bet.parse::<usize>()
                .expect(format!("bet '{}' should be a number", bet).as_str()),
            get_hand_type(hand),
        )
    });

    let hands_ranked = hands.sorted_by(|(a_hand, a_bet, a_type), (b_hand, b_bet, b_type)| {
        hand_order(a_hand, *a_bet, *a_type, b_hand, *b_bet, *b_type)
    });
    // println!("hands: {:?}", hands_ranked.clone().enumerate().map(|(i,v)| (i+1,v,(i+1) * v.1)).collect_vec());

    hands_ranked
        .enumerate()
        .map(|(i, (_hand, bet, _strength))| bet * (i + 1))
        .sum()
}

fn part2_hand_sorter(input: &str) -> Vec<(String, usize, HandType)> {
    let mut rtn = input
        .replace('J', "Z")
        .par_lines()
        .map(|line| {
            let (hand, bet) = line.split_once(" ").unwrap();
            (
                hand.to_owned(),
                bet.parse::<usize>()
                    .expect(format!("bet '{}' should be a number", bet).as_str()),
                get_joker_hand_type(hand),
            )
        })
        .collect::<Vec<_>>(); // Collect into a vector to sort later
    rtn.par_sort_by(|(a_hand, a_bet, a_type), (b_hand, b_bet, b_type)| {
        hand_order(a_hand, *a_bet, *a_type, b_hand, *b_bet, *b_type)
    });
    rtn
}

fn part2(input: &str) -> usize {
    part2_hand_sorter(input)
        .iter()
        .enumerate()
        .map(|(i, (_hand, bet, _strength))| bet * (i + 1))
        .sum()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part1() {
        assert_eq!(part1(include_str!("./input.txt")), 249390788);
    }

    #[test]
    fn test_part2() {
        assert_eq!(part2(include_str!("./input.txt")), 0);
    }

    #[test]
    fn part1_example() {
        assert_eq!(part1(include_str!("./example.txt")), 6440);
    }

    #[test]
    fn part2_example() {
        assert_eq!(part2(include_str!("./example.txt")), 5905);
    }

    #[test]
    fn ordering() {
        assert_eq!(card_compare(&'A', &'A'), Ordering::Equal);
        assert_eq!(card_compare(&'A', &'K'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'Q'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'J'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'T'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'9'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'8'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'7'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'6'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'5'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'4'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'3'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'2'), Ordering::Greater);
        assert_eq!(card_compare(&'A', &'1'), Ordering::Greater);
    }

    #[test]
    fn hand_strength_test() {
        assert_eq!(get_hand_type("32T3K"), HandType::OnePair);
        assert_eq!(get_hand_type("KK677"), HandType::TwoPair);
        assert_eq!(get_hand_type("KTJJT"), HandType::TwoPair);
        assert_eq!(get_hand_type("T55J5"), HandType::ThreeOfAKind);
        assert_eq!(get_hand_type("QQQJA"), HandType::ThreeOfAKind);
    }

    #[test]
    fn joker_hand_strength_test() {
        // Standard hands without Jokers
        assert_eq!(get_joker_hand_type("QQQQA"), HandType::FourOfAKind);
        assert_eq!(get_joker_hand_type("KKKQ2"), HandType::ThreeOfAKind);
        assert_eq!(get_joker_hand_type("AA234"), HandType::OnePair);
        assert_eq!(get_joker_hand_type("AKQ2T"), HandType::HighCard);

        // Hands with one Joker
        assert_eq!(get_joker_hand_type("QQQZ2"), HandType::FourOfAKind); // Joker acts as Q
        assert_eq!(get_joker_hand_type("ZZ234"), HandType::ThreeOfAKind); // Joker acts as 3
        assert_eq!(get_joker_hand_type("AZKQ2"), HandType::OnePair); // Joker acts as 2

        // Hands with multiple Jokers
        assert_eq!(get_joker_hand_type("QQZZ2"), HandType::FourOfAKind); // Jokers act as Qs
        assert_eq!(get_joker_hand_type("ZZZQ2"), HandType::FourOfAKind); // Jokers act as Qs
        assert_eq!(get_joker_hand_type("ZZ234"), HandType::ThreeOfAKind); // Two Jokers act as 2s or 3s

        // Complex cases with multiple Jokers
        assert_eq!(get_joker_hand_type("ZZKQQ"), HandType::FourOfAKind); // Two Jokers act as Queens
        assert_eq!(get_joker_hand_type("K2ZZZ"), HandType::FourOfAKind); // Three Jokers act as Ks
    }

    #[test]
    fn bad_answers() {
        assert_eq!(
            part2(include_str!("./input.txt")).cmp(&248618050),
            Ordering::Greater
        );
        assert_eq!(
            part2(include_str!("./input.txt")).cmp(&249028421),
            Ordering::Less
        );
    }

    #[test]
    fn reddit_example() {
        let input = "2345A 1
Q2KJJ 13
Q2Q2Q 19
T3T3J 17
T3Q33 11
2345J 3
J345A 2
32T3K 5
T55J5 29
KK677 7
KTJJT 34
QQQJA 31
JJJJJ 37
JAAAA 43
AAAAJ 59
AAAAA 61
2AAAA 23
2JJJJ 53
JJJJ2 41";
        assert_eq!(part1(input), 6592);

        let part2_ordering = "2345A
J345A
2345J
32T3K
KK677
T3Q33
Q2KJJ
T3T3J
Q2Q2Q
2AAAA
T55J5
QQQJA
KTJJT
JJJJJ
JJJJ2
JAAAA
2JJJJ
AAAAJ
AAAAA";
        let ours = part2_hand_sorter(input)
            .iter()
            .map(|x| &x.0)
            .join("\n")
            .replace("Z", "J");
        assert_eq!(ours, part2_ordering);
        assert_eq!(part2(input), 6839);
    }
}
