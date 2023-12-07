use std::collections::{HashMap, BTreeMap};

/// 
fn main() {

    /* let num = '2';
    let sym = '*';
    let node1 = Node::new_number(&num, Coordinate::from(1, 2));
    let node2 = Node::new_symbol(&sym, Coordinate::from(1, 3));

    let list = vec![node1, node2];

    for (index, node) in list.iter_mut().enumerate() {
        println!("Checking {:?}", node);
        if index > 0 && list.contains(list[index]) {

            println!("We have neigbour!");
        }
    }

    dbg!(&list[0]);
    dbg!(&list[1]); */

    let str = include_str!("part1.txt");

    let tree = str.lines().enumerate().take(1).flat_map(|(y, line)| {
        line.chars().enumerate().map(move |(x, character)| {
            ((y, x), match character {
                '.' => Value::Empty,
                c if c.is_ascii_digit() => Value::Number(c.to_digit(10).expect("Is a valid number")),
                c => Value::Symbol(c)
            })
        })
    })
    .collect::<BTreeMap<(usize, usize), Value>>();

    dbg!(tree);

    /* let mut map: Vec<((isize, isize), usize, &char)> = vec![];
    let mut current_symbol = '.'; */



    /* for (row, line) in str.lines().enumerate() {
        for (col, char) in line.chars().enumerate() {
            if char == '.' {
                continue;
            }

            if char.is_digit(10) {
                map.push(((row as isize, col as isize), 1, &char));
                continue;
            }

            
        }
    } */

    //let sum = process_input(str);
    //println!("{sum}");
}

fn process_input(input: &str) -> String {
    let mut numbers: HashMap<Coordinate, char> = HashMap::new();
    let mut symbols: HashMap<Coordinate, char> = HashMap::new();
    let mut matches: Vec<(&Coordinate, &char)> = vec![];

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
        let neighbours = vec![
            Coordinate::from(coordinate.row - 1, coordinate.col - 1), Coordinate::from(coordinate.row - 1, coordinate.col), Coordinate::from(coordinate.row - 1, coordinate.col + 1),
            Coordinate::from(coordinate.row, coordinate.col - 1),                                                           Coordinate::from(coordinate.row, coordinate.col + 1),
            Coordinate::from(coordinate.row + 1, coordinate.col - 1), Coordinate::from(coordinate.row + 1, coordinate.col), Coordinate::from(coordinate.row + 1, coordinate.col + 1),
        ];
        //println!("neighbours: {:?}", neighbours);
        let has_neighbours = neighbours
            .iter()
            .filter(|neighbour| {
                symbols.contains_key(neighbour)
            })
            //.inspect(|i| println!("neighbour: {:?}", i))
            .count() > 0;

        if has_neighbours {
            //println!("{:?} at {:?} has a symbol neighbour", char, coordinate);
            matches.push((coordinate, char));
        }
    }

    matches.sort_by(|left, right| {
        left.0.cmp(right.0)
    });

    println!("{:?}", matches);

    String::from("")
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
enum Variant {
    Number,
    Symbol
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
enum Value {
    Number(u32),
    Symbol(char),
    Empty
}

#[derive(PartialEq, Eq, Ord, PartialOrd, Hash, Debug)]
struct Node<'a> {
    value: &'a char,
    link: Option<&'a Self>,
    variant: Variant,
    coordinate: Coordinate
}

impl<'a> Node<'a> {
    fn new_number(value: &'a char, coordinate: Coordinate) -> Self {
        Node { value, link: None, variant: Variant::Number, coordinate }
    }

    fn new_symbol(value: &'a char, coordinate: Coordinate) -> Self {
        Node { value, link: None, variant: Variant::Symbol, coordinate }
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