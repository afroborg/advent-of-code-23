# Advent of code 2023

Solutions for [Advent of code 2023](https://adventofcode.com/2023).

## Project structure

- `template` - template for new day
- `day-xx` - solution for day `xx`
  - `input1.txt` - input data for day `xx`
  - `input2.txt` - input data for day `xx`
  - `src` - source code for day `xx`
    - `bin` - binaries to get the solution
      - `part1.rs` - runs the solution for part 1 using `input1.txt`
      - `part2.rs` - runs the solution for part 2 using `input2.txt`
    - `lib.rs` - library with common code for part 1 and 2
    - `part1.rs` - solution for part 1
    - `part2.rs` - solution for part 2

### Testing

```bash
cargo test part1
cargo test part2
```

This runs the tests in `part1.rs` and `part2.rs` respectively.

## Generating a new day

Uses [cargo-generate](https://github.com/cargo-generate/cargo-generate) to generate a new day based on the [`template`](./template) directory.

```bash
cargo generate --path ./template --name day-xx
```
