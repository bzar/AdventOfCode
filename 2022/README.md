# Advent of Code - 2022

Implementation comments

## day01
- Quick work with `itertools::batching`

## day02
- Enum variant values for scoring

## day03
- Two variants, regular and optimized
  - Regular is easier to read, but does a lot of duplicate work and reserves a lot of memory
  - Optimized uses sorting and an O(n) algorithm for finding the match without any additional memory

## day04
- First one with a nom parser, I was still learning here
- Basically the entire solution is imµemented within the parser

## day05
- Two variants, with and without nom
  - nom variant is not that good, I was still learning
- Stack manipulation itself was simple
- part1/part2 are just ways to execute Actions

## day06
- Super simple with `itertools::windows` and `itertools::all_unique`

## day07
- Better nom parsing already! Avoided tons of allocations by storing only string slices

## day08
- Implementing a ray casting function helped a ton
- `itertools::cartesian_product` was nice as well
- Used `bittle` bitfields for the visibility map for fun

## day09
- Used a const generic parametrized `tail_positions` function to implement both parts at the same time without allocating like a madlad
- The scanning lambda moves the snake around and emits tail positions, so they can be counted after passing through a HashSet

## day10
- Two variants, regular and minimized allocations (AFAICT no allocations within my code)
- nom parsing, still learning

## day11
- Starting to learn nom around here
- `monkey_business` could be nicer, but ran out of time

## day12
- Basic BFS

## day13
- Figured I could use `include_str!` for reading input around here
- Implemented `PartialOrd` for the value enum and everything was easy after that

## day14
- This is a mess to read, but I eventually got it quite fast by varying the sand dropping position
- I also merged walls and floors to have less ranges to match against

## day15
- Reused the range merging from day14, nice nom parsing and stuff

## day16
- Not too nice
- Getting the pruning right was quite hard and the result is not that clean

## day17
- Took a long time because I accidentally used puzzle input for the tests
- Fun bit fiddling
- Part 2 was not so nice

## day18
- Basically geometry and DFS to find air cubes

## day19
- Bruteforced part2 with rayon :D
- Takes hours to run but works

## day20
- Had to write a custom test for the rotate function as its intended behavior wasn't at all clear
- After getting `rotate` right it was easy peasy

## day21
- Solving the "Human" value is not at all pretty here, but it works
- Parsing and `eval` were quite straightforward

## day22
- Had to resort to hard coding puzzle faces in part 2
- Also needed a custom test to find a bug
- Part 1 was fun, part 2 not so much

## day23
- Used a thread local hashmap to avoid allocating for every round :D
- Nothing too special otherwise

## day24
- Implemented a generic A* algorithm for this one
- Predicting the storms instead of simulating them was a good idea
- Generalizing `path_length` for multiple waypoints was straightforward for part2

## day25
- Some fun playing with numbers, nothing special

