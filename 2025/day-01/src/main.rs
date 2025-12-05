pub mod part1;
pub mod part2;

fn main() -> eyre::Result<()> {
    color_eyre::install()?;
    let content = include_str!("../input.txt");
    let part_1_answer = part1::part_1(content)?;
    println!("Part 1: {}", part_1_answer);

    let part_2_answer = part2::part_2(content)?;
    assert!(part_2_answer > 5246, "{} too low", part_2_answer); // known too low
    assert!(part_2_answer < 6294, "{} too high", part_2_answer); // known too high
    println!("Part 2: {}", part_2_answer);

    Ok(())
}
