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

### [Day 2](https://adventofcode.com/2024/day/2)
I had some trouble writing a function to validate whether the levels in the report were safe. Part 2 relied on this function even more and forced me to refactor my Part 1 solution before I could move on to solving Part 2.

### [Day 3](https://adventofcode.com/2024/day/3)
Initially, I tried to solve this puzzle without using regex, but parsing the input manually at 6 AM proved more challenging than I anticipated. Later, I refactored the code, dropping regex in favor of a custom parsing function, which improved performance.

### [Day 4](https://adventofcode.com/2024/day/4)
I experimented with different approaches to solve this puzzle. Initially, I split the input data into four separate vectors containing the x and y positions for each letter, and used `.contains()` method to check if a calculated neighbor position was present in the appropriate vector. While this approach was convenient, it turned out to be quite costly in terms of performance.

Ultimately, I switched to storing the input data as a `Vec<Vec<char>>` and validated neighbor positions by checking if x and y were within the required range. For Part 1, I prioritized readability by using the `.all()` method, even though it came with a slight performance trade-off.

### [Day 5](https://adventofcode.com/2024/day/5)
My solutions required some refinement, particularly for Part 2. I eventually decided to use a `HashMap` to store page ordering rules, which helped reduce the number of comparisons needed to check if the pages were in the correct order.

Part 2 took me by surprise, and I ended up unnecessarily implementing a sorting function by removing pages that were out of order and inserting them into the correct position. At one point, I even considered implementing a topological sort to optimize my sorting logic. Fortunately, after taking a short break, I realized that the `.sort_by()` method was the correct solution. This not only simplified my code but also improved its performance.

### [Day 6](https://adventofcode.com/2024/day/6)
My initial solution modified the map (`Vec<Vec<char>`) to mark visited cells. During refactoring, I switched to using a `HashSet` to record visited cells, which simplified the code at a slight performance cost.

For Part 2, my initial solution was a brute-force approach. I attempted to insert an additional obstacle at every cell. This required creating a copy of the map (`Vec<Vec<u8>>`) because I was modifying it during traversal. I couldn't devise a proper way to detect infinite loops, so I assumed that if the guard's step count exceeded a certain threshold, they were stuck in an infinite loop. Despite these flaws, the solution ran in 0.9 seconds.
The refactored version avoids modifying the map entirely. It only inserts obstacles on the guard's route and properly handles infinite loop detection.

### [Day 7](https://adventofcode.com/2024/day/7)
Initially, I struggled to solve this puzzle using recursion, and my first solution generated all possible combinations of operations. Part 2 was particularly challenging. At first, I misread the instructions and implemented an incorrect solution. Later, I created a solution that worked on the test data but failed with my actual input. Eventually, I identified the root issue: equations that produce the correct result before all terms are used should be considered invalid. Adding this condition resolved the problem. The refactored version, which uses recursion, is a significant improvement over my initial approach.
