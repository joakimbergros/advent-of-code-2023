use nom_supreme::ParserExt;
use std::{error::Error, ops::Range};

use nom::{
    bytes::complete::{tag, take_until},
    character::complete::{line_ending, space1, u64},
    multi::{many1, separated_list1},
    sequence::{preceded, tuple},
    IResult, Parser,
};

fn main() -> Result<(), Box<dyn Error>> {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    let (input, seeds) = parse(str)?;
    println!("{sum}");

    Ok(())
}

fn process_input(input: &str) -> String {
    let location = parse(input).expect("msg");
    location.1.to_string()
}

fn parse(input: &str) -> IResult<&str, u64> {
    //let (input, seeds) = preceded(tag("seeds:"), many1(preceded(tag(" "), u64)))(input)?;
    let (input, seeds) = tag("seeds: ")
        .precedes(separated_list1(space1, u64))
        .parse(input)?;

    let (input, maps) = many1(
        take_until("map:")
            .precedes(tag("map:"))
            .precedes(many1(line_ending.precedes(map_line)).map(|mappings| SeedMap { mappings })),
    )
    .parse(input)?;

    //dbg!(&seeds, maps);

    let location = seeds
        .iter()
        .map(|seed| maps.iter().fold(*seed, |acc, map| map.translate(acc)))
        .min()
        .expect("should have a min mapped value");

    Ok((input, location))
}

fn map_line(input: &str) -> IResult<&str, (Range<u64>, Range<u64>)> {
    let (input, (destination, source, num)) =
        tuple((u64, u64.preceded_by(tag(" ")), u64.preceded_by(tag(" "))))(input)?;

    Ok((
        input,
        (source..(source + num), destination..(destination + num)),
    ))
}

#[derive(Debug)]
struct SeedMap {
    mappings: Vec<(Range<u64>, Range<u64>)>,
}

impl SeedMap {
    fn translate(&self, source: u64) -> u64 {
        let valid_mapping = self
            .mappings
            .iter()
            .find(|(source_range, _)| source_range.contains(&source));

        let Some((source_range, destination_range)) = valid_mapping else {
            return source;
        };

        let offset = source - source_range.start;

        destination_range.start + offset
    }
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

        assert_eq!("35", process_input(str));
    }
}
