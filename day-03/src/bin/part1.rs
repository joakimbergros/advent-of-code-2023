use std::collections::BTreeMap;

/// 
fn main() {
    let str = include_str!("part1.txt");
    let total = process_input(str);

    dbg!(total);
}

fn process_input(input: &str) -> String {
    let map = input.lines().enumerate().flat_map(|(row, line)| {
        line.chars().enumerate().map(move |(col, character)| {
            (Coordinate::from(row as isize, col as isize), match character {
                '.' => Value::Empty,
                c if c.is_ascii_digit() => Value::Number(c.to_digit(10)
                    .expect("Is a valid number")),
                c => Value::Symbol(c)
            })
        })
    })
    .collect::<BTreeMap<Coordinate, Value>>();

    let mut numbers: Vec<Vec<((&Coordinate, &Value), Vec<Coordinate>)>> = vec![vec![]];
    let mut number_counter = 0;

    // TODO Skip filtering away the empty arrays in this after...
    for (coordinate, val) in &map {
        if let Value::Number(_) = val {
            let neighbours = vec![
                Coordinate::from(coordinate.row - 1, coordinate.col - 1), Coordinate::from(coordinate.row - 1, coordinate.col), Coordinate::from(coordinate.row - 1, coordinate.col + 1),
                Coordinate::from(coordinate.row, coordinate.col - 1),                                                           Coordinate::from(coordinate.row, coordinate.col + 1),
                Coordinate::from(coordinate.row + 1, coordinate.col - 1), Coordinate::from(coordinate.row + 1, coordinate.col), Coordinate::from(coordinate.row + 1, coordinate.col + 1),
            ];
            numbers[number_counter].push(((coordinate, val), neighbours));
            continue;
        }
        
        number_counter += 1;
        numbers.push(vec![]);
    }

    numbers = numbers.into_iter().filter_map(|c| {
        if c.is_empty() {
            None
        } else {
            Some(c)
        }
    }).collect::<Vec<Vec<((&Coordinate, &Value), Vec<Coordinate>)>>>();

    let mut total = 0;
    for number in numbers {
        let item_number = number
            .iter()
            .fold(0, |acc, v| acc * 10 + match v.0.1 {
                Value::Number(num) => num,
                _ => &0
            });
        
        let is_part = number.iter().any(|v| {
            v.1.iter().any(|n| {
                if let Some(value) = &map.get(n) {
                    if let Value::Symbol(_) = value {
                        return true;
                    }
    
                    false
                } else { false }
            })
        });

        if is_part {
            total += item_number;
        }
    }

    total.to_string()
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
struct Coordinate {
    row: isize,
    col: isize
}

impl Coordinate {
    fn from(row: isize, col: isize) -> Self {
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
        
        assert_eq!("4361", process_input(str));
    }
}