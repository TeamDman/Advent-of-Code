mkdir day-%02d
cd
cargo init .
cargo add eyre color-eyre
copy gitignore
copy rustfmt
create part1.rs skeleton
create part2.rs skeleton
create empty input.txt
update main to call skeletons
add commented out asserts for correct value, value too low, value too high
