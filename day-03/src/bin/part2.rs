use std::collections::BTreeMap;

/// 
fn main() {
    let str = include_str!("part2.txt");
    let total = process_input(str);

    dbg!(total);
}

fn process_input(input: &str) -> String {
    let map = input.lines().enumerate().take(10).flat_map(|(row, line)| {
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

    // Index both number and symbol, then filter on any number containing symbol and then product

    let mut numbers: Vec<Vec<((&Coordinate, &Value), Vec<Coordinate>)>> = vec![vec![]];
    let mut number_counter = 0;
    let mut cogs: Vec<Vec<((&Coordinate, &Value), Vec<Coordinate>)>> = vec![vec![]];
    let mut cog_counter = 0;

    for (coordinate, val) in &map {
        let neighbours = vec![
            Coordinate::new(coordinate.row - 1, coordinate.col - 1), Coordinate::new(coordinate.row - 1, coordinate.col), Coordinate::new(coordinate.row - 1, coordinate.col + 1),
            Coordinate::new(coordinate.row, coordinate.col - 1),                                                           Coordinate::new(coordinate.row, coordinate.col + 1),
            Coordinate::new(coordinate.row + 1, coordinate.col - 1), Coordinate::new(coordinate.row + 1, coordinate.col), Coordinate::new(coordinate.row + 1, coordinate.col + 1),
        ];

        match val {
            Value::Number(_) => {
                numbers[number_counter].push(((coordinate, val), neighbours));
                continue;
            },
            Value::Symbol(sym) if *sym == '*' => {
                cogs[cog_counter].push(((coordinate, val), neighbours));
                continue;
            },
            _ => {
                number_counter += 1;
                numbers.push(vec![]);
                cog_counter += 1;
                cogs.push(vec![]);
            }
        }
    }

    numbers.iter()
        .filter(|number| {
            
        })

    dbg!(cogs);

    "_total".to_string()
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

/* struct Node<'a> {
    coordinate: Coordinate,
    value: &'a char,
    neighbours: Vec<Coordinate>
} */

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