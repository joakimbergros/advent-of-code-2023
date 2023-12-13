use std::collections::BTreeMap;

fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let mut almanac_iterator = input.lines();
    let mut seeds = almanac_iterator.next()
        .expect("Should be the first line")
        .split(": ")
        .last()
        .expect("Should be only the numbers")
        .split(' ')
        .map(|seed| Seed::new(seed.parse::<u64>().expect("Should be a number")))
        .collect::<Vec<Seed>>();

    let mut shelf = Shelf::new();
    let mut current_category = Category::None;

    for line in almanac_iterator {
        if line.is_empty() {
            println!("End of block");
            continue;
        }

        if !line.chars().next().expect("Line should not be empty").is_ascii_digit() {
            current_category = match line {
                "seed-to-soil map:" => Category::SeedToSoil,
                "soil-to-fertilizer map:" => Category::SoilToFertilizer,
                "fertilizer-to-water map:" => Category::FertilizerToWater,
                "water-to-light map:" => Category::WaterToLight,
                "light-to-temperature map:" => Category::LightToTemperature,
                "temperature-to-humidity map:" => Category::TemperatureToHumidity,
                "humidity-to-location map:" => Category::HumidityToLocation,
                _ => unreachable!("Should always be a category assigned")
            };

            continue;
        }

        // 0 - Destination, 1 - Source, 2 - Length
        let routing = line.split(' ')
            .map(|number| number.parse::<u64>().expect("Should be a valid number"))
            .collect::<Vec<u64>>();

        //dbg!(&routing);

        shelf.add(current_category, Map::new(routing[0], routing[1], routing[2]));
    }

    let minimum_number = seeds.iter_mut()
        .map(|seed| shelf.find_location(seed))
        .min()
        .expect("Should have a number");

    minimum_number.to_string()
}

#[derive(Debug)]
struct Seed {
    //start: u32,
    route: Vec<u64>
}

impl Seed {
    fn new(start: u64) -> Self {
        Seed { /* start, */ route: vec![start] }
    }

    fn set(&mut self, next: u64) {
        self.route.push(next);
    }

    fn get(&self) -> u64 {
        *self.route.iter().last().expect("Should be at least one")
    }
}

#[derive(Debug)]
struct Shelf {
    shelf: BTreeMap<Category, Vec<Map>>
}

impl Shelf {
    fn new() -> Self {
        //let maps: BTreeMap<Category, Vec<Map>> = Category::values().into_iter().map(|c| (*c, vec![])).collect();
        Shelf { shelf: BTreeMap::new() }
    }

    fn add(&mut self, category: Category, map: Map) {
        self.shelf.entry(category)
            .and_modify(|list| list.push(map))
            .or_insert(vec![map]);
    }

    fn find_location(&self, seed: &mut Seed) -> u64 {
        let final_value = self.shelf.values()
            .filter_map(|maps| {
                //dbg!(&maps);
                match maps.iter()
                    .filter_map(|map| {
                        dbg!(&seed);
                        dbg!(&map);
                        dbg!(&map.in_range(seed));
                        match map.in_range(seed) {
                            true => Some(map.redirect(seed)),
                            false => None
                        }
                    })
                    .inspect(|location| {
                        dbg!(location);
                    })
                    .min() {
                        Some(val) => {
                            seed.set(val);
                            Some(val)
                        },
                        None => {
                            None
                        }
                    }
            })
            .inspect(|contender| {
                dbg!(contender);
            })
            .min()
            .expect("Should have found a number");
        //dbg!(final_value);
        final_value
    }
}

#[derive(Debug, Copy, Clone)]
struct Map {
    destination: u64,
    source: u64,
    length: u64
}

impl Map {
    fn new(destination: u64, source: u64, length: u64) -> Self {
        Map { destination, source, length }
    }

    fn in_range(&self, source: &Seed) -> bool {
        source.get() >= self.source && source.get() <= self.source + self.length
    }

    fn redirect(&self, source: &Seed) -> u64 {
        match source.get() >= self.source && source.get() <= self.source + self.length {
            true => self.destination + (source.get() - self.source),
            false => source.get(),
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
    fn test_in_range() {
        let seed = Seed::new(99);
        let map = Map::new(50 , 98, 2);

        assert!(map.in_range(&seed));
    }

    #[test]
    fn test_redirect() {
        let seed = Seed::new(79);
        let maps = vec![Map::new(52, 50, 48), Map::new(50 , 98, 2)];

        let number = maps.iter()
            .filter_map(|map| match map.in_range(&seed) {
                true => Some(map.redirect(&seed)),
                false => None
            })
            .min()
            .expect("Should be a number");

        assert_eq!(number, 81);
    }

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