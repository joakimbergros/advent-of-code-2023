use std::{ops::Range, error::Error};

use nom::{sequence::{tuple, preceded, terminated}, character::complete::{digit1, digit0, u8, u64}, IResult, bytes::complete::tag, multi::many1};

fn main() -> Result<(), Box<dyn Error>> {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    let (input, seeds) = parse(str)?;
    dbg!(seeds);
    println!("{sum}");

    Ok(())
}

fn process_input(input: &str) -> String {
    
    "lowest_path".to_string()
}

fn parse(input: &str) -> IResult<&str, Vec<u64>> {
    let (input, seeds) = preceded(tag("seeds:"), many1(preceded(tag(" "), u64)))(input)?;
    //let (input, maps) = 

    Ok((input, seeds))
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