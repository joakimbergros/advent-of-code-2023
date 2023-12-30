use std::ops::Index;

use nom::{
    branch::alt,
    character::complete::{i32, line_ending, space1},
    combinator::eof,
    multi::{many1, separated_list1},
    sequence::terminated,
    IResult,
};

fn main() {
    let str = include_str!("test.txt");
    let sum = process(str);
    println!("{sum}");
}

fn parse(input: &str) -> IResult<&str, Vec<Vec<Vec<i32>>>> {
    many1(terminated(
        many1(separated_list1(space1, i32)),
        alt((line_ending, eof)),
    ))(input)
}

fn process(input: &str) -> String {
    let (input, mut sequences) = parse(input).expect("successful parse");
    debug_assert_eq!(input, "");

    dbg!(&sequences);

    let answer: i32 = sequences.into_iter()
        .map(|sequence| {
            sequence.iter().map(|series| {
                let mut collection: Vec<Vec<i32>> = vec![];
                collection.push(series.clone());

                dbg!(collection);
            });
            0
        })
        .sum();

    "".to_string()
}

fn reduce(numbers: &[i32]) -> Vec<i32> {
    numbers
        .windows(2)
        .map(|l| l[1] - l[0])
        .collect::<Vec<i32>>()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "0 3 6 9 12 15
1 3 6 10 15 21
10 13 16 21 30 45
";

        assert_eq!("114", process(str));
    }
}
