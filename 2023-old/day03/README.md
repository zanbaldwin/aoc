# AoC 2023 (Day 03)

## Progress Report
- Part 1
  - Completed solution that passed all unit tests
  - Answer that was generated and submitted to AoC was incorrect
  - Cloned Chris Biscardi's solution to generate the correct answer so I could
    move on to part 2 (but did not look at code)
- Part 2
  - Completed solution but it was not passing unit tests
  - Found it would only counting 2 gears instead of 1
  - Discovered bug in calculating the bounding box coordinates (off by 1 error)
  - Fixing that bug also caused part 1 solution to work
- Other
  - Added externally published method to parse input text and export and JSON structure
  - Added C library format as compilation target
  - Defined externally published method in a C header file
  - Added example PHP script that passes the input text to Rust to be parsed
  - Currently have tested that the FFI code works (provided FFI extension is installed in PHP)
  - Added PHP code to solve the problem using the data structure returned from Rust/FFI
