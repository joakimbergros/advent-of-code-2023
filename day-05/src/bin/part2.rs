use core::panic;
use std::{collections::BTreeMap, ops::RangeInclusive};

fn main() {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let mut almanac_iterator = input.lines();
    /* let seeds = almanac_iterator.next()
    .expect("Should be the first line")
    .split(": ")
    .last()
    .expect("Should be only the numbers")
    .split(' ')
    .map(|seed| seed.parse::<u64>().expect("Should be a number"))
    .collect::<Vec<u64>>(); */
    /* let mut seeds: Vec<u64> = vec![];
    let mut current_numbers: Vec<u64> = vec![];

    let mut current_number: u64 = 0;
    let mut current_number_finished = false;
    let mut current_length: u64 = 0;
    let mut current_length_finished = false; */

    //println!("=== Parsing seeds ===");
    let seeds = almanac_iterator
        .next()
        .expect("Should be the first line with seeds")
        .split(' ')
        .skip(1)
        .collect::<Vec<&str>>()
        .chunks(2)
        .map(|pair| {
            //println!("[i] Parsing '{:?}' ===", pair);
            (
                pair[0]
                    .parse::<u64>()
                    .expect("Should be the first of two numbers"),
                pair[1]
                    .parse::<u64>()
                    .expect("Should be the first of two numbers"),
            )
        })
        .collect::<Vec<(u64, u64)>>();

    let mut current_category = Category::None;
    let mut maps: BTreeMap<Category, Vec<(u64, u64, u64)>> = BTreeMap::new();
    let mut lowest_path: u64 = 0;

    for line in almanac_iterator {
        if line.is_empty() {
            //println!("=== End of block ===");
            //println!();
            continue;
        }

        if !line
            .chars()
            .next()
            .expect("Line should not be empty")
            .is_ascii_digit()
        {
            //println!("=== Parsing '{}' ===", line);
            current_category = match line {
                "seed-to-soil map:" => Category::SeedToSoil,
                "soil-to-fertilizer map:" => Category::SoilToFertilizer,
                "fertilizer-to-water map:" => Category::FertilizerToWater,
                "water-to-light map:" => Category::WaterToLight,
                "light-to-temperature map:" => Category::LightToTemperature,
                "temperature-to-humidity map:" => Category::TemperatureToHumidity,
                "humidity-to-location map:" => Category::HumidityToLocation,
                _ => unreachable!("Should always be a category assigned"),
            };

            continue;
        }

        //println!("[i] Parsing map '{}'", line);
        // 0 - Destination, 1 - Source, 2 - Length
        let routing = line
            .split(' ')
            .map(|number| number.parse::<u64>().expect("Should be a valid number"))
            .collect::<Vec<u64>>();
        let touple = (routing[0], routing[1], routing[2]);

        maps.entry(current_category)
            .and_modify(|map| map.push(touple))
            .or_insert(vec![touple]);
    }

    for range in seeds {
        //println!("*** Running range '{:?}' ***", &range);

        for seed in range.0..(range.0 + range.1) {
            //println!("*** Mapping seed '{:?}' ***", &seed);
            let mut currently_mapped = seed;

            for (category, maps) in &maps {
                let mut lowest_number: Option<u64> = None;

                //println!("[i] Category is '{:?}'", &category);
                for map in maps {
                    //println!("[i] Map is '{:?}'", &map);
                    if currently_mapped < map.1 || currently_mapped > (map.1 + map.2) {
                        //println!("[-] Seed is not in range");
                        continue;
                    }

                    //println!("[+] Seed is in range");
                    let candidate = map.0 + currently_mapped - map.1;

                    lowest_number = match lowest_number {
                        Some(val) if val < candidate => Some(val),
                        _ => Some(candidate),
                    };

                    break;
                }

                currently_mapped = lowest_number.unwrap_or(currently_mapped);
                //println!("*** New value is {:?}", currently_mapped);
                //println!();
            }

            //println!("<======!!! Lowest map is '{}' !!!======>", currently_mapped);
            //println!();

            if lowest_path == 0 || currently_mapped < lowest_path {
                lowest_path = currently_mapped;
            }
        }
    }

    println!("<======!!! Winner is '{}' !!!======>", &lowest_path);
    lowest_path.to_string()
}

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Clone, Copy)]
enum Category {
    None,
    SeedToSoil,
    SoilToFertilizer,
    FertilizerToWater,
    WaterToLight,
    LightToTemperature,
    TemperatureToHumidity,
    HumidityToLocation,
}

impl Category {
    const VALUES: [Category; 7] = [
        Category::SeedToSoil,
        Category::SoilToFertilizer,
        Category::FertilizerToWater,
        Category::WaterToLight,
        Category::LightToTemperature,
        Category::TemperatureToHumidity,
        Category::HumidityToLocation,
    ];
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
