use std::collections::{BTreeMap, BTreeSet, HashMap};

/// 
fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let mut numbers: HashMap<Coordinate, char> = HashMap::new();
    let mut symbols: HashMap<Coordinate, char> = HashMap::new();

    for (line_index, line) in input.lines().take(5).enumerate() {
        for (char_index, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            } else if char.is_digit(10) {
                numbers.insert(Coordinate::from(line_index as isize, char_index as isize), char);
            } else if char.is_symbol() {
                symbols.insert(Coordinate::from(line_index as isize, char_index as isize), char);
            }
        }
    }

    for (coordinate, char) in &numbers {
        dbg!(coordinate);
        dbg!(r#char);
        let neighbours = vec![
            Coordinate::from(coordinate.y - 1, coordinate.x - 1), Coordinate::from(coordinate.y - 1, coordinate.x), Coordinate::from(coordinate.y - 1, coordinate.x + 1),
            Coordinate::from(coordinate.y, coordinate.x - 1),                                                       Coordinate::from(coordinate.y, coordinate.x + 1),
            Coordinate::from(coordinate.y + 1, coordinate.x - 1), Coordinate::from(coordinate.y + 1, coordinate.x), Coordinate::from(coordinate.y + 1, coordinate.x + 1),
        ];
        let has_neighbours = neighbours
            .iter()
            .filter(|neighbour| {
                symbols.contains_key(neighbour)
            })
            .count() > 0;

        if has_neighbours {
            println!("{:?} at {:?} has a symbol neighbour", char, coordinate);
        }

        //println!("{:?}", has_neighbours);
    }

    // for line in input.lines() {
    //     let mut map_line: Vec<char> = vec![];
    //     for char in line.chars() {
    //         map_line.push(char);
    //     }
    //     map.push(map_line);
    // }
    // println!("{:?}", map);

    // for (i, set) in map.iter().take(1).enumerate() {
    //     let set_len = set.len();

    //     for (i, needle) in set.iter().enumerate() {
    //         let is_digit = needle.is_digit(10);

    //         // Check right
    //         if is_digit && set_len > i && (set[i + 1] != '.' || !set[i + 1].is_digit(10)) {
    //             let test = needle.is_symbol();
    //         }

    //         // Check left
    //         if i > 0 && is_digit {
                
    //         }

    //         // Check north

    //         // Check south
    //     }
    // }

    String::from("")
}

#[derive(PartialEq, Eq, Hash, Debug)]
struct Coordinate {
    x: isize,
    y: isize
}

impl Coordinate {
    fn from(x: isize, y: isize) -> Self {
        Coordinate { x, y }
    }
}

struct Symbol<'a> {
    x: u32,
    y: u32,
    value: &'a char
}

struct Number {
    x: u32,
    y: u32,
    len: u32,
    value: u32
}

trait Symbolize {
    fn is_symbol(&self) -> bool;
}

impl Symbolize for char {
    fn is_symbol(&self) -> bool {
        !self.is_alphabetic() || !self.is_alphanumeric()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() { 
        let str = "467..114..
...*......
..35..633.
......#...
617*......
.....+.58.
..592.....
......755.
...$.*....
.664.598..";
        
        assert_eq!("", process_input(str));
    }
}