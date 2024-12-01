## Advent of Code 2024 Solutions

This repository contains my solutions for the [Advent of Code 2024](https://adventofcode.com/2024) challenge, implemented in Rust.

The purpose of this project is to:
- improve my Rust programming skills,
- showcase my problem-solving approach,
- practice writing clear and readable code,
- practice basic Git workflows.

### Directory Structure

- `src/day_xx_puzzle_name/`: Contains Rust scripts for each day's challenge, where `xx` is the day number and `puzzle_name` is the name of the puzzle.
- `input/`: Directory where input files are expected to be placed, formatted as `xx.txt`. **Note**: Input files are not included in this repository.
- `test_input/`: Directory where test input files are stored, formatted as `xx.txt` (or `xx_y.txt` if necessary for multiple test cases).

### [Day 1](https://adventofcode.com/2024/day/1)

This was an enjoyable puzzle. Part 2 provided an opportunity to optimize the solution from O(n<sup>2</sup>) to O(n) using a `HashMap`.
