# AoC 2023 (Day 05)

## Progress Report
- Part 1
  - Parsing was fun
  - Keeping modules in lib.rs today just to see how it feels
  - Unit tests pass
  - Input could not be parsed
  - Numbers in input are too big
  - Update i32 to i64
  - Input correctly parsed and answer generated was correct
- Part 2
  - Easy enough to implement, just one extra method on Almanac struct
  - Pretty sure this can be optimised, as it took nearly 10 minutes to calculate the answer
  - But that's a problem for another day perhaps
  - Total time to calculate answer: ~9 minutes
- Part 2 Refactor
  - My input results in: 1,638,141,121 (1.6 billion) seeds, and around 230
    possible mapping transformations to check
  - That's a lot of iterations
  - Not to mention keeping all the resulting calculated locations in memory to
    find the lowest number after all that.
  - The highest seed number is 4,114,434,577 so we can assume the highest
    location is also something similar
  - Working backwards we can lower the iterations by checking a _maximum_ of
    ~4billion and exiting as soon as we have a positive result, rather than
    going through all 350billion+ iterations and finding the lowest
  - Total time to calculate answer: ~2.5 seconds
- Notes
  - I tried using Rayon to parallelise the iterations across multiple threads
    but for some reason this almost doubled the execution time (?!). Probably
    has to do with how Rayon splits the numbers to be checked.
  - Rayon might have had more of a better impact on performance if I had added
    before I refactored the solution
