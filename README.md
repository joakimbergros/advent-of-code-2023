# advent-of-code-2023

My attempt at doing Advent Of Code 2023 in Rust

## Day 1

Regex sollution didn't end up working as there were strings in the input where the digit words were overlapping.
For '2twonebs', this only matched 'two' and not both 'two' and 'one'

Iterating over the sliced line one index at a time made me able to check if slice started with the digit word.
The `std::iter::from_fn` was a nice tip and it makes you able to create an iteration from the callback return value.

Great help from [Youtube](https://www.youtube.com/watch?v=JOgQMjpGum0) after I got stuck debugging this!

## Day 2

## Day 3

## Day 4

## Day 5

## Day 6

## Day 7

## Day 8

Part 1 was no problem, and part 2 i wrote logic that would've given the right answer but bruteforcing it would've taken months or years, so I ended up looking up someone elses sollution.  
It got me to learn about LCM so that's always a plus!