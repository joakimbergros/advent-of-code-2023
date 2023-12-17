use nom::{sequence::{terminated, preceded, tuple}, complete::tag, character::complete::{multispace0, u8}, Parser, bytes::complete::take_till, IResult};
use nom_supreme::ParserExt;

fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    
    String::from("")
}

fn parse(input: &str) -> IResult<&str, Vec<(u8, u8)>> {
    let (input, holds) = tuple((
        tag("Time:")
    )).parse(input)?
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "Time:      7  15   30
Distance:  9  40  200";
        
        assert_eq!("288", process_input(str));
    }
}