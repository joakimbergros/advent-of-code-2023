use std::collections::BTreeMap;

fn main() {
    let str = include_str!("part1.txt");

    let map = Map::new(10, 50, 2);
    dbg!(map.get_destination(52));

    //let sum = process_input(str);
    //println!("{sum}");
}

fn process_input(input: &str) -> String {
    let mut almanac_iterator = input.lines();
    let seeds = almanac_iterator.next()
        .expect("Should be the first line")
        .split(": ")
        .last()
        .expect("Should be only the numbers")
        .split(' ')
        .map(|seed| Seed::new(seed.parse::<u32>().expect("Should be a number")))
        .collect::<Vec<Seed>>();

    let mut shelf = Shelf::new();
    let mut current_category = Category::None;

    for line in almanac_iterator {
        if line.is_empty() {
            println!("End of block");
            continue;
        }

        if !line.chars().next().expect("Line should not be empty").is_digit(10) {
            current_category = match line {
                "seed-to-soil map:" => Category::SeedToSoil,
                "soil-to-fertilizer map:" => Category::SoilToFertilizer,
                "fertilizer-to-water map:" => Category::FertilizerToWater,
                "water-to-light map:" => Category::WaterToLight,
                "light-to-temperature map:" => Category::LightToTemperature,
                "temperature-to-humidity map:" => Category::TemperatureToHumidity,
                "humidity-to-location map:" => Category::HumidityToLocation,
                _ => Category::None
            };

            continue;
        }

        // 0 - Destination, 1 - Source, 2 - Length
        let routing = line.split(' ')
            .map(|number| number.parse::<u32>().expect("Should be a valid number"))
            .collect::<Vec<u32>>();

        dbg!(&routing);

        shelf.add(current_category, Map::new(routing[0], routing[1], routing[2]));
    }

    for seed in seeds {

    }

    dbg!(shelf);

    String::from("")
}

struct Seed {
    number: u32
}

impl Seed {
    fn new(number: u32) -> Self {
        Seed { number }
    }
}

#[derive(Debug)]
struct Shelf {
    maps: BTreeMap<Category, Vec<Map>>
}

impl Shelf {
    fn new() -> Self {
        //let maps: BTreeMap<Category, Vec<Map>> = Category::values().into_iter().map(|c| (*c, vec![])).collect();
        Shelf { maps: BTreeMap::new() }
    }

    fn add(&mut self, category: Category, map: Map) {
        self.maps.entry(category)
            .and_modify(|list| list.push(map))
            .or_insert(vec![]);
    }

    fn find_location(&self, seed: u32) -> u32 {
        self.maps.iter()
            .map(|(_, m)| m.iter()
                .map(|map| map.get_destination(seed)))
            .min()
        2
    }
}

#[derive(Debug)]
struct Map {
    destination: u32,
    source: u32,
    length: u32
}

impl Map {
    fn new(destination: u32, source: u32, length: u32) -> Self {
        Map { destination, source, length }
    }

    fn get_destination(&self, source: u32) -> u32 {
        match source >= self.source && source <= self.source + self.length {
            true => self.destination + (source - self.source),
            false => source,
        }
    }
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
    HumidityToLocation
}

impl Category {
    fn values() -> &'static [Category] {
        static VALUES: [Category; 7] = [
            Category::SeedToSoil,
            Category::SoilToFertilizer,
            Category::FertilizerToWater,
            Category::WaterToLight,
            Category::LightToTemperature,
            Category::TemperatureToHumidity,
            Category::HumidityToLocation,
        ];

        &VALUES
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
        
        assert_eq!("", process_input(str));
    }
}