pub mod part1;
pub mod part2;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let input = include_str!("../input.txt");

    let part_1_answer = part1::part_1(input)?;
    assert_eq!(part_1_answer, 17766);
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part2::part_2(input)?;
    assert_eq!(part_2_answer, 176582889354075);
    println!("Part 2: {}", part_2_answer);
    Ok(())
}
