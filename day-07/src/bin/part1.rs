use std::ops::Range;

use nom::{
    bytes::complete::tag,
    character::complete::{multispace0, newline, u32},
    multi::separated_list1,
    sequence::{preceded, tuple},
    IResult,
};

fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let (_, values) = parse(input).expect("parse should have worked");

    let possibilities = values
        .into_iter()
        .map(|run| {
            let range = Range {
                start: 0,
                end: run.0,
            };
            range
                .into_iter()
                .filter_map(|travel_speed| {
                    let remaining_time = run.0 - travel_speed;
                    let length = travel_speed * remaining_time;
                    match length > run.1 {
                        true => Some(length),
                        false => None,
                    }
                })
                .count() as u32
        })
        .product::<u32>();

    possibilities.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<(u32, u32)>> {
    let (input, holds) = preceded(
        tuple((tag("Time:"), multispace0)),
        separated_list1(multispace0, u32),
    )(input)?;
    let (input, times) = preceded(
        tuple((newline, tag("Distance:"), multispace0)),
        separated_list1(multispace0, u32),
    )(input)?;

    let mut runs: Vec<(u32, u32)> = vec![];

    for index in 0..=(holds.len() - 1) {
        runs.push((holds[index], times[index]));
    }

    Ok((input, runs))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!("288", process_input(str));
    }
}
