use nom::{
    branch::alt,
    bytes::complete::tag,
    character::complete::{self, alphanumeric1, line_ending, multispace1},
    combinator::eof,
    multi::{fold_many1, many1},
    sequence::{delimited, separated_pair, terminated},
    IResult, Parser,
};
use std::collections::BTreeMap;

fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

#[derive(Debug, Clone)]
enum Direction {
    Left,
    Right,
}

#[allow(clippy::type_complexity)]
fn parser(input: &str) -> IResult<&str, (Vec<Direction>, BTreeMap<&str, (&str, &str)>)> {
    let (input, instructions) = many1(alt((
        complete::char('R').map(|_| Direction::Right),
        complete::char('L').map(|_| Direction::Left),
    )))(input)?;
    let (input, _) = multispace1(input)?;
    let (input, map) = fold_many1(
        terminated(
            separated_pair(
                alphanumeric1,
                tag(" = "),
                delimited(
                    complete::char('('),
                    separated_pair(alphanumeric1, tag(", "), alphanumeric1),
                    complete::char(')'),
                ),
            ),
            alt((line_ending, eof)),
        ),
        BTreeMap::new,
        |mut acc: BTreeMap<&str, (&str, &str)>, (key, value)| {
            acc.insert(key, value);
            acc
        },
    )(input)?;

    Ok((input, (instructions, map)))
}

fn process_input(input: &str) -> String {
    let (input, (instructions, map)) = parser(input).expect("should validly parse");

    debug_assert_eq!(input, "");

    // let choices = input.lines().next().unwrap().chars().map(|c| match c {
    //     'L' => Direction::Left,
    //     'R' => Direction::Right,
    //     _ => unreachable!("should be only L and R")
    // }).collect::<Vec<Direction>>();
    //
    // let maps = input
    //     .lines()
    //     .skip(2)
    //     .map(|line| {
    //         line.replace(['(', ')'], "")
    //             .split_once(" = ")
    //             .map(|(id, paths)| {
    //                 (
    //                     id,
    //                     paths
    //                         .split_once(", ")
    //                         .map(|(l, r)| (l, r))
    //                         .expect("should be left and right"),
    //                 )
    //             })
    //         .expect("should be a valid map")
    // })
    // .collect::<BTreeMap<&str, (&str, &str)>>();

    let current_maps = map
        .keys()
        .filter(|key| key.ends_with('A'))
        .cloned()
        .collect::<Vec<_>>();

    let results = current_maps
        .iter()
        .map(|node| {
            let mut visited_nodes = vec![*node];
            let mut current_node = *node;

            instructions
                .iter()
                .cycle()
                .enumerate()
                .find_map(|(index, instruction)| {
                    let options = map
                        .get(current_node)
                        .expect("should always be a valid node");

                    let next_node = match instruction {
                        Direction::Left => options.0,
                        Direction::Right => options.1,
                    };

                    if next_node.ends_with('Z') {
                        Some(index + 1)
                    } else {
                        visited_nodes.push(next_node);
                        current_node = next_node;
                        None
                    }
                })
                .expect("should find a cycle")
        })
        .collect::<Vec<usize>>();

    // dbg!(results);

    let min_cycle = lcm(&results);

    min_cycle.to_string()

    // choices.iter().cycle().enumerate().find_map(|(index, choice)| {
    //
    // });
    // for (step, choice) in (0u32..).zip(choices.iter().cycle()) {
    //     // println!("Step '{step}'");
    //     //
    //     if current_maps.iter().all(|map| map.0.ends_with('Z')) {
    //         return step.to_string();
    //     }
    //
    //     current_maps = current_maps.iter().map(|map| {
    //         let next = match choice {
    //             'R' => &map.1 .1,
    //             'L' => &map.1 .0,
    //             _ => unreachable!("should be no other choice"),
    //         };
    //
    //         maps.iter().find(|m| m.0 == next).expect("should be a map with id '{next}'")
    //     })
    //     .collect::<Vec<_>>();
    // }

    // let mut steps: u32 = 0;

    // for (step, choice) in (0u32..).zip(choices.iter()) {
    //     if *starts.0 == GOAL_ID {
    //         return step.to_string();
    //     }
    //
    //     §let next = match choice {
    //         'R' => &starts.1 .1,
    //         'L' => &starts.1 .0,
    //         _ => unreachable!("should be no other choice"),
    //     };
    //
    //     starts = maps
    //         .iter()
    //         .find(|map| map.0 == next)
    //         .expect("id of next should exist");
    // }
    //
    // for choice in choices.iter().cycle() {
    //     if *current.0 == goal_id {
    //         return steps.to_string();
    //     }
    //
    //     let next = match choice {
    //         'R' => &current.1 .1,
    //         'L' => &current.1 .0,
    //         _ => unreachable!("should be no other choice"),
    //     };
    //
    //     current = maps
    //         .iter()
    //         .find(|map| map.0 == next)
    //         .expect("id of next should exist");
    //     steps += 1;
    // }
}

pub fn lcm(nums: &[usize]) -> usize {
    if nums.len() == 1 {
        return nums[0];
    }
    let a = nums[0];
    let b = lcm(&nums[1..]);
    a * b / gcd_of_two_numbers(a, b)
}

fn gcd_of_two_numbers(a: usize, b: usize) -> usize {
    if b == 0 {
        return a;
    }
    gcd_of_two_numbers(b, a % b)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "LR

11A = (11B, XXX)
11B = (XXX, 11Z)
11Z = (11B, XXX)
22A = (22B, XXX)
22B = (22C, 22C)
22C = (22Z, 22Z)
22Z = (22B, 22B)
XXX = (XXX, XXX)";

        assert_eq!("6", process_input(str));
    }
}
