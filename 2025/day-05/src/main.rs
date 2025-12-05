pub mod part1;
pub mod part2;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../input.txt");

    let part_1_answer = part1::part_1(input)?;
    // Uncomment and set here to run quick assertions for the puzzle.
    assert_eq!(part_1_answer, 726);
    // assert!(part_1_answer > 0, "{} too low", part_1_answer);
    // assert!(part_1_answer < 0, "{} too high", part_1_answer);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part2::part_2(input)?;
    assert_eq!(part_2_answer, 354226555270043);
    // assert!(part_2_answer > 0, "{} too low", part_2_answer);
    // assert!(part_2_answer < 0, "{} too high", part_2_answer);
    println!("Part 2: {}", part_2_answer);
    Ok(())
}
