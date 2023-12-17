use std::{ops::Range, error::Error};
use nom_supreme::ParserExt;

use nom::{sequence::{preceded, tuple}, character::complete::{u64, line_ending}, IResult, bytes::complete::{tag, take_until}, multi::{many1, separated_list1}, Parser};

fn main() -> Result<(), Box<dyn Error>> {
    let str = include_str!("test.txt");
    let sum = process_input(str);
    let (input, seeds) = parse(str)?;
    println!("{sum}");

    Ok(())
}

fn process_input(input: &str) -> String {
    
    "lowest_path".to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    //let (input, seeds) = preceded(tag("seeds:"), many1(preceded(tag(" "), u64)))(input)?;
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(tag(" "), u64))
        .parse(input)?;

    let (input, maps) = take_until("map:")
        .precedes(tag("map:"))
        .precedes(many1(line_ending.precedes(map_line.map(|ranges| SeedMap {
            source: ranges.0,
            destination: ranges.1
        }))))
        .parse(input)?;

    dbg!(&seeds, maps);

    Ok((input, seeds))
}

fn map_line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) = tuple((
        u64,
        u64.preceded_by(tag(" ")),
        u64.preceded_by(tag(" "))
    ))(input)?;

    Ok((
        input,
        (
            source..(source + num),
            destination..(destination + num)
        )
    ))
}

#[derive(Debug)]
struct SeedMap {
    source: Range<u64>,
    destination: Range<u64>,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "seeds: 79 14 55 13

seed-to-soil map:
50 98 2
52 50 48

soil-to-fertilizer map:
0 15 37
37 52 2
39 0 15

fertilizer-to-water map:
49 53 8
0 11 42
42 0 7
57 7 4

water-to-light map:
88 18 7
18 25 70

light-to-temperature map:
45 77 23
81 45 19
68 64 13

temperature-to-humidity map:
0 69 1
1 0 69

humidity-to-location map:
60 56 37
56 93 4";
        
        assert_eq!("46", process_input(str));
    }
}