use std::{fmt::Write, ops::Range};

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
    dbg!(&values);
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
                .count() as u64
        })
        .product::<u64>();

    possibilities.to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<(u64, u64)>> {
    let (input, holds) = preceded(
        tuple((tag("Time:"), multispace0)),
        separated_list1(multispace0, u32),
    )(input)?;
    let (input, times) = preceded(
        tuple((newline, tag("Distance:"), multispace0)),
        separated_list1(multispace0, u32),
    )(input)?;

    dbg!(&holds, &times);

    let hold = holds.iter().fold(String::new(), |mut acc, val| {
        write!(&mut acc, "{}", val).expect("should be a valid number");
        acc
    });
    let time = times.iter().fold(String::new(), |mut acc, val| {
        write!(&mut acc, "{}", val).expect("should be a valid number");
        acc
    });

    dbg!(&hold, &time);

    Ok((
        input,
        vec![(
            hold.parse::<u64>().expect("should work"),
            time.parse::<u64>().expect("should work"),
        )],
    ))
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "Time:      7  15   30
Distance:  9  40  200
";

        assert_eq!("71503", process_input(str));
    }
}
