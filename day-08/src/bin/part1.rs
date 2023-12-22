use std::collections::BTreeMap;

fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let choices = input.lines().next().unwrap().chars().collect::<Vec<char>>();

    let maps = input
        .lines()
        .skip(2)
        .map(|line| {
            line.replace(['(', ')'], "")
                .split_once(" = ")
                .map(|(id, paths)| {
                    (
                        id.to_string(),
                        paths
                            .split_once(", ")
                            .map(|(l, r)| (l.to_string(), r.to_string()))
                            .expect("should be left and right"),
                    )
                })
                .expect("should be a valid map")
        })
        .collect::<BTreeMap<String, (String, String)>>();

    let mut current = maps
        .iter()
        .find(|map| *map.0 == String::from("AAA"))
        .expect("should always be a entry id");
    let goal_id = String::from("ZZZ");
    let mut steps: u32 = 0;

    for choice in choices.iter().cycle() {
        if *current.0 == goal_id {
            return steps.to_string();
        }

        let next = match choice {
            'R' => &current.1 .1,
            'L' => &current.1 .0,
            _ => unreachable!("should be no other choice"),
        };

        current = maps
            .iter()
            .find(|map| map.0 == next)
            .expect("id of next should exist");
        steps += 1;
    }

    "fall".to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "RL

AAA = (BBB, CCC)
BBB = (DDD, EEE)
CCC = (ZZZ, GGG)
DDD = (DDD, DDD)
EEE = (EEE, EEE)
GGG = (GGG, GGG)
ZZZ = (ZZZ, ZZZ)";

        assert_eq!("", process_input(str));
    }
}
