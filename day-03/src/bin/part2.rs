use std::{collections::BTreeMap, vec};

fn main() {
    let str = include_str!("part2.txt");
    let total = process_input(str);

    dbg!(total);
}

fn process_input(input: &str) -> String {
    let map = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().map(move |(col, character)| {
            (Coordinate::new(row as isize, col as isize), match character {
                '.' => Value::Empty,
                c if c.is_ascii_digit() => Value::Number(c.to_digit(10)
                    .expect("Is a valid number")),
                c => Value::Symbol(c)
            })
        })
    })
    .collect::<BTreeMap<Coordinate, Value>>();

    let mut cog_map: BTreeMap<&Coordinate, Vec<u32>> = BTreeMap::new();
    for (coordinate, value) in &map {
        match value {
            Value::Symbol(sym) if *sym == '*' => {
                cog_map.insert(coordinate, vec![]);
            },
            _ => {}
        }
    }

    let mut numbers: Vec<Vec<((&Coordinate, &Value), Vec<Coordinate>)>> = vec![vec![]];
    let mut number_counter = 0;

    // TODO Skip filtering away the empty arrays in this after...
    for (coordinate, val) in &map {
        if let Value::Number(_) = val {
            let neighbours = vec![
                Coordinate::new(coordinate.row - 1, coordinate.col - 1), Coordinate::new(coordinate.row - 1, coordinate.col), Coordinate::new(coordinate.row - 1, coordinate.col + 1),
                Coordinate::new(coordinate.row, coordinate.col - 1),                                                           Coordinate::new(coordinate.row, coordinate.col + 1),
                Coordinate::new(coordinate.row + 1, coordinate.col - 1), Coordinate::new(coordinate.row + 1, coordinate.col), Coordinate::new(coordinate.row + 1, coordinate.col + 1),
            ];
            numbers[number_counter].push(((coordinate, val), neighbours));
            continue;
        }

        number_counter += 1;
            numbers.push(vec![]);
    }

    for (coordinate, batches) in cog_map.iter_mut() {
        for batch in &numbers {
            if !batch.iter()
                .any(|item| {
                    item.1.contains(coordinate)
                }) {
                continue;
            };

            let number = batch.iter()
                .map(|item| {
                    if let Value::Number(val) = item.0.1 {
                        *val
                    } else {
                        0
                    }
                })
                .fold(0, |acc, val| {
                    acc * 10 + val
                });

            batches.push(number);
        }
    }

    let total = cog_map.iter().filter_map(|cog| {
        if cog.1.len() != 2 {
            None
        } else {
            Some(cog.1.iter().product::<u32>())
        }
    }).sum::<u32>();

    total.to_string()
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
struct Coordinate {
    row: isize,
    col: isize
}

impl Coordinate {
    fn new(row: isize, col: isize) -> Self {
        Coordinate { row, col }
    }
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
enum Value {
    Number(u32),
    Symbol(char),
    Empty
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
        
        assert_eq!("467835", process_input(str));
    }
}