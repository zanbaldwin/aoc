# AoC 2023 (Day 10)

## Progress Report

### Setup
- It's here!
- The kind of problem that I couldn't solve in previous years!
- Graph traversal (I think)
- I'll be really proud of myself if I can solve this without looking up
  somebody else's solution
- I know that whatever I come up with will be inefficient and lacking any
  decent algorithms
- But I'll try my best!

### Part 1
- `thread 'part1::tests::test_part1_input2' has overflowed its stack`
- Kind of excited
- Been programming for over a decade, used StackOverflow.com many many times
- Never had an actual stack overflow error before!
- Finally completed, that took so much longer and so many more unit tests than
  I thought it would

### Part 2
- Oh. My. God.
- You want me to do what?
- Okay, I know how to do this, just count how many times you cross a boundary
  until you reach the edge
- Odd means in, even means out
- Whoops, I was counting only ground tiles - need to include pipes that aren't
  part of the loop too
- Several hours later, I need to find a hint online
- Found ray-casting algorithm and jordan curve algorithm on wikipedia
- Honestly didn't understand what it was saying or how to apply it
- But there's pictures? And they make sense?
- Aha! How many crossings depends on whether two corners make a U-bend or a
  S-curve
- I've spent several more hours at this and I want to give up
- I've refactored the unit tests, bought in RsTest, just need to keep adding
  test cases until I find the bug in my code
- For the second time today: **Oh. My. God.**
- My dumbass brain got east and west mixed up
- Everything works
- I wasted **hours** on east/west bug
- If it wasn't a Sunday where I can just throw an entire day at it, I would
  have given up and failed

### Notes
- This solution runs in approx half a second
- There are so many iterations that could be saved by using a better algorithm
- I can already think of a few places to speed up the logic
- But guess what? _Not today,_ ~~Eric Wastl~~ _Satan_.
- Also, I need to stop throwing all of the logic in with the models, it's so
  object-orientated of me
- I didn't use [nom](https://docs.rs/nom/latest/nom/) today!
