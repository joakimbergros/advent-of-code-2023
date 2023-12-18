use std::ops::Range;

fn main() {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let seeds = input
        .lines()
        .next()
        .expect("should be the first line")
        .split("seeds: ")
        .last()
        .expect("should be all nubers")
        .split(' ')
        .map(|num| num.parse::<u64>().expect("should be a valid number"))
        .collect::<Vec<u64>>();

    let maps = input
        .lines()
        .skip(3) // File is always the same
        .filter_map(parse_maps)
        .collect::<Vec<SeedMap>>();

    dbg!(seeds, maps);
    "lowest_path".to_string()
}

fn parse_maps(string: &str) -> Option<SeedMap> {
    match string.chars().next() {
        Some(val) if val.is_ascii_digit() => {
            let tuple = string.split(' ').fold((0, (0, 0, 0)), |mut acc, str| {
                match acc.0 {
                    0 => acc.1 .0 = str.parse::<u64>().expect("should be the destination"),
                    1 => acc.1 .1 = str.parse::<u64>().expect("should be the source"),
                    2 => acc.1 .2 = str.parse::<u64>().expect("should be the length"),
                    _ => unreachable!("should only be 3 numbers"),
                }

                acc.0 += 1;

                acc
            });

            Some(SeedMap {
                source: tuple.1 .1..tuple.1 .1 + tuple.1 .2,
                destination: tuple.1 .0..tuple.1 .0 + tuple.1 .2,
            })
        }
        _ => None,
    }
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
