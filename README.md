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

### [Day 8](https://adventofcode.com/2024/day/8)
This puzzle was fairly straightforward for me. My refactored version is not significantly different from the original solution. The only notable mistake I made was generating `n * (n - 1)` combinations of possible pairs instead of the `n * (n - 1) / 2` unique pairs. However, this allowed me to simplify the logic by only searching in one direction for antinodes after calculating the delta between two antennas.

### [Day 9](https://adventofcode.com/2024/day/9)
I found several ways to improve my Part 1 solution, and the final version ended up being quite concise. Initially, I used the `.position()` method to find `free_space_index`, but this approach left a lot of room for performance improvements. The next iteration stored all indices of free space in a `Vec`, and a subsequent version also added a `Vec` for occupied space indices. In the final version, I replaced these vectors with two `while` loops to efficiently update `free_space_index` and `occupied_space_index`.

Part 2 required a different approach to processing the input data. I separated file and free space information, storing them in two `Vec<(u32, u32)>`, where the first value indicated the starting index, and the second value represented the length. Unfortunately, I couldn't improve on my initial solution—every attempt at optimization resulted in a performance loss, leaving me slightly unsatisfied.

### [Day 10](https://adventofcode.com/2024/day/10)
I took my time solving this puzzle because I had too many ideas and couldn’t commit to a single approach. I considered several methods, such as going from each possible starting point to each possible endpoint or vice versa. Along the way, I scrapped two partial implementations and started over, realizing I could make the solution even better.

Eventually, I implemented a solution that stored all unique possible next neighbor locations for each step. For example, if a starting point had two valid neighbors, I would store both `(x, y)` positions. I then repeated this process, looking for all unique possible next neighbor locations until the endpoint was reached. In the refactored version, I replaced the `HashSet` with a `Vec` to store all possible next neighbor locations, sorting it and removing duplicates. This approach was significantly faster. I continued refining the solution before moving on to Part 2, which gave me enough time to explore different ways of traversing the area.

For Part 2, I decided to use recursion, which turned out to be surprisingly easy to implement. With the `get_valid_neighbours()` function already in place, the final solution came together quickly.

### [Day 11](https://adventofcode.com/2024/day/11)
I solved Part 1 and thought it was the easiest Part 1 so far. However, that was only true because the naive solution was able to handle the required number of loops.

Part 2 was the most straightforward so far — it simply required tripling the number of loops. My naive solution was no longer sufficient. I started by inspecting how different numbers changed with each loop, hoping to find a pattern. However, I quickly realized it would be difficult to predict how a given number would evolve after `n` iterations. Then, the obvious solution struck me: I needed to use a `HashMap` to store numbers as keys and their respective counts as values. This significantly reduced the number of operations required per loop iteration.

My `part_1.rs` file contains the naive solution, while `part_2.rs` implements the optimized solution using a `HashMap`.

### [Day 12](https://adventofcode.com/2024/day/12)
This puzzle was quite complex for me. I began by processing the input and grouping cells into plots based on their plant type using a `HashMap`. The keys were discarded, and only the values representing the `x` and `y` positions of the cells were kept. This resulted in a structure of type `Vec<Vec<(u32, u32)>>`, where each element of the outer vector contained a vector of cells with the same plant type. Next, each group of cells with the same plant type was further divided into separate plots. For each individual plot, the perimeter was calculated by checking the neighbours of each cell. If a neighbour was not part of the same plot, the perimeter value was increased by 1. Finally, the perimeter value for each plot was multiplied by the number of cells in the plot to produce the final result.

I spent a considerable amount of time on Part 2 trying to devise a method to count edges or corners, as both numbers are equal. In the end, the solution was surprisingly straightforward.

I created four sets of neighbours, each containing three directions:
- **N, E, NE**
- **S, E, SE**
- **S, W, SW**
- **N, W, NW**

For each of these sets, I checked and increased the corner count by 1 if either of the following conditions was met:
- Neighbours 1 and 2 were both present, but neighbour 3 was not part of the plot.
- Neighbours 1 and 2 were both not part of the plot.  

### [Day 13](https://adventofcode.com/2024/day/13)
This was a mathematical puzzle. Processing the input file was the harder part for me. Part 1 was solved without transforming the two equations that could be built with each input set of six numbers. There were two unknown variables: `a` (representing how many times button A was pressed) and `b` (representing how many times button B was pressed). For `a` in the range 0 to 100 inclusive, the value of `b` was calculated based on the first equation, and then both values were used in the second equation to validate whether they matched the required results. Since the number of button presses had to be an unsigned integer (as pressing a button 85.12 times is not possible), a condition was enforced during the calculation of `b` to ensure that the division produced no remainder before it could be completed.

For Part 2, the two equations were transformed, allowing the values of `a` and `b` to be directly calculated. However, solving the transformed equations occasionally produced invalid values for `a` and `b`. To address this, the `abs_diff` method was used during subtraction to prevent `u64` values from overflowing, and a condition `numerator % denominator == 0` was checked before division. This process ensured that any invalid results were properly discarded.

Equation transformations:
- a * x<sub>1</sub> + b * x<sub>2</sub> = x<sub>result</sub>
- a = (x<sub>result</sub> - b * x<sub>2</sub>) / x<sub>1</sub>
- a * y<sub>1</sub> + b * y<sub>2</sub> = y<sub>result</sub>
- ((x<sub>result</sub> - b * x<sub>2</sub>) / x<sub>1</sub>) * y<sub>1</sub> + b * y<sub>2</sub> = y<sub>result</sub>
- b = (y<sub>result</sub> * x<sub>1</sub> - x<sub>result</sub> * y<sub>1</sub>) / (x<sub>2</sub> * y<sub>1</sub> - x<sub>1</sub> * y<sub>2</sub>)

Where:
- x<sub>1</sub> - x position change when button A is pressed
- x<sub>2</sub> - x position change when button B is pressed
- y<sub>1</sub> - y position change when button A is pressed
- y<sub>2</sub> - y position change when button B is pressed
- x<sub>result</sub> - x position that had to be reached
- y<sub>result</sub> - y position that had to be reached

Before calculating, it was necessary to ensure that x<sub>2</sub> * y<sub>1</sub> ≠ x<sub>1</sub> * y<sub>2</sub>, as dividing by zero is undefined.
This condition means that the two sets of coefficients (x<sub>1</sub>, y<sub>1</sub>) and (x<sub>2</sub>, y<sub>2</sub>) must not be proportional to each other. This also allowed me to remove unnecessary code in Part 1 that was expecting more than one valid set of `a` and `b`.

### [Day 14](https://adventofcode.com/2024/day/14)
Part 1 was straightforward, as the exact guard locations after `n` iterations could be calculated using modulo arithmetic.

Part 2, however, was far more enigmatic, stating that the guards would form a Christmas tree after `n` iterations. I explored various approaches but initially failed to determine the correct iteration number. The final solution is slow because it checks all possible unique iterations, of which there are 10,403 (103 × 101). After each iteration, the guards are divided into groups, and the iteration with the smallest number of groups is assumed to provide the correct solution.

#### Updates
The calculation of guard positions based on their starting position and velocity has been simplified by using the `.rem_euclid()` method.

Part 2 has been refactored to improve the speed of solving the puzzle:
- A `HashSet` is now used instead of a `Vec` to speed up lookups using the `.contains()` method and to efficiently remove duplicate guard positions.
- The method for determining the iteration that forms the tree has been revised. Instead of splitting guards into groups, a "closeness factor" is calculated. This factor increases each time a guard has a neighboring guard, providing a faster measure for determining the desired iteration.

### [Day 15](https://adventofcode.com/2024/day/15)
The puzzle was about moving boxes in a warehouse. The robot tried to move in a predefined way.  
Part 1 was relatively easy to implement. It was possible to move multiple boxes at once if they were aligned (one after another) in the direction of the robot's movement. Boxes could be moved if there was an empty space behind them.

Part 2 was more challenging as each box occupied 2 horizontal spaces. When the robot moved horizontally, the box or boxes moved by half of their length. The interesting part was when the robot moved vertically and encountered a box. This box could push up to two other boxes, and those two boxes could push up to three new boxes, and so on. I decided not to keep track of the robot anymore, and its position was reacquired from the warehouse state before each move. This allowed me to add the robot to the pool of objects whose positions would be swapped. The pool of such objects was sorted to ensure that the furthest objects were moved to empty spaces first.

### [Day 16](https://adventofcode.com/2024/day/16)
This puzzle provided an opportunity to revisit shortest path algorithms. 

Part 1 was a single-pair shortest path problem in an undirected, cyclic graph. Continuing in the same direction was cheap, but changing direction was expensive. It was enough to find the lowest score path without needing to store the actual path. I used `BinaryHeap` with `Reverse` to make it a min-heap. The final version is concise.

Part 2 was more complex, as there were multiple paths with the lowest score. The task was to find all unique tiles on these paths. I used the following:
- `BinaryHeap` with `Reverse` for the min-heap,
- a `HashMap` to store the lowest scores from the starting tile to each tile,
- a second `HashMap` to store predecessors.

The lowest score path was then reconstructed using predecessors, and finally, unique tiles were counted.

### [Day 17](https://adventofcode.com/2024/day/17)
Part 1 required carefully transcribing all the possible operations that could occur based on the instruction number. Operations were performed using one of two types of operands. One operand type had some additional rules to follow when built. Once everything was in place, it was simply a matter of feeding the input and getting the result of these computations.

Part 2 was challenging, as it required finding an input value that would produce the required output. A brute-force approach didn't seem feasible, so inspecting the calculation machine was in order.

The machine produced only series of 3-bit numbers (0–7 in decimal). Testing different values revealed that the output size followed specific ranges:
- 0 – 7 → output with 1 number
- 8 – 63 → output with 2 numbers
- 64 – 511 → output with 3 numbers
- 512 – 4095 → output with 4 numbers
- 4096 – 32767 → output with 5 numbers

To make it clearer, the search number ranges are:
- from 0 to 2<sup>3</sup> - 1,
- from 2<sup>3</sup> to 2<sup>6</sup> - 1,
- from 2<sup>6</sup> to 2<sup>9</sup> - 1,
- from 2<sup>9</sup> to 2<sup>12</sup> - 1,
- from 2<sup>12</sup> to 2<sup>15</sup> - 1.

If our searched value is supposed to produce output with 16 numbers, we are looking at the search range between 2<sup>45</sup> to 2<sup>48</sup> - 1.

I used the following approach:

- Start by finding a number from 0 to 7 that produces an output with just the last number of the expected output.
- Append another number from 0 to 7 by shifting the bits left by 3 and performing a bitwise OR with the new number.
- One of the 8 new numbers would produce an output that matches the last two numbers.
- Continue appending numbers from 0 to 7 and checking until the number produces all the numbers from the expected output.

### [Day 18](https://adventofcode.com/2024/day/18)
This was a shortest path problem. I used breadth-first search (BFS) to solve Part 1. There was no need to reconstruct the path; simply counting the number of steps taken was sufficient to solve it.

For Part 2, I initially used a naive approach by checking after each new obstacle whether the end position could still be reached. The next version involved constructing the shortest path and verifying if the newly added obstacle fell on this path. However, this approach proved to be incorrect, as a new path could still be formed around the obstacle. To handle this, I introduced an additional check to determine if an alternative path could be created, but this solution was inefficient. The final approach utilized binary search to efficiently reduce the number of checks needed to determine whether a path could still be formed between the start and end positions. This allowed me to pinpoint the exact obstacle that caused the end position to become unreachable.

### [Day 19](https://adventofcode.com/2024/day/19)

Initially, my approach went partially in the wrong direction. I attempted to reduce the number of patterns by checking whether the design contained them. I then built a recursive function that checked if the design started with each valid pattern. If it did, the function passed a shortened version of the design (reduced by the matched pattern's length) to itself recursively. This approach worked perfectly on the test data but failed on my input.

The issue arose with the third design, which was longer than the previous two and had 73 valid patterns. Initially, I focused on further reducing the number of valid patterns, but I couldn't find a universal solution. Eventually, I introduced memoization to minimize the number of recursive calls, which turned out to be a viable solution. Removing my initial function that attempted to reduce the pattern number actually increased performance.

Part 2 required slight modifications to the memoization logic. Instead of breaking early when any valid way to construct the design was found, the solution needed to explore all branches to account for every possible construction.
