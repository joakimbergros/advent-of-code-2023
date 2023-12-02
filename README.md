# advent-of-code-2023

My attempt at doing Advent Of Code 2023 in Rust

## Day 1

Regex sollution didn't end up working as there were strings in the input where the digit words were overlapping.
For '2twonebs', this only matched 'two' and not both 'two' and 'one'

Iterating over the sliced line one index at a time made me able to check if slice started with the digit word.
The `std::iter::from_fn` was a nice tip and it makes you able to create an iteration from the callback return value.

Great help from [Youtube](https://www.youtube.com/watch?v=JOgQMjpGum0) after I got stuck debugging this!