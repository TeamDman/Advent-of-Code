# Day Template

This folder is a ready-to-copy template for creating a new `day-XX` workspace.

Quick steps (PowerShell):

1. Copy the template to a new folder (replace `05` with desired day number):

   Copy-Item -Recurse day-template day-05

2. Update the package name inside the new folder's `Cargo.toml`:

   (Get-Content day-05\Cargo.toml) -replace 'name = "day-template"', 'name = "day-05"' | Set-Content day-05\Cargo.toml

3. Edit `input.txt` with the puzzle input.
4. Implement puzzle logic in `src/part1.rs` and `src/part2.rs`.
5. Optionally run:

   cd day-05
   cargo check
   cargo test

Quick steps (Unix):

1. Copy template and update package name (replace `05`):

   cp -r day-template day-05
   sed -i 's/name = "day-template"/name = "day-05"/' day-05/Cargo.toml

2. Edit `day-05/input.txt`, implement logic, and run `cargo check`/`cargo test`.

Notes:
- `main.rs` already includes commented-out sample asserts for answers and ranges.
- Update the asserts to match known answers for quick validation.
