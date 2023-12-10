use core::num;
use std::collections::BTreeMap;

fn main() {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    println!("{sum}");
}


fn process_input(input: &str) -> String {
    let tickets = input.lines()
        /* .skip(1)
        .take(1) */
        .map(|line| {
            line.split(": ")
                .last()
                .expect("Should be all the numbers")
                .split(" | ")
                .map(|numbers| {
                    numbers.split(' ')
                        .filter_map(|number| {
                            if number.is_empty() {
                                None
                            } else {
                                Some(number.parse::<u32>().expect("Should be a parsable number"))
                            }
                        })
                        .collect()
                })
                .fold(vec![], |mut acc, val| {
                    acc.push(val);
                    acc
                })
    }).collect::<Vec<Vec<Vec<u32>>>>();

    let mut copy_map: BTreeMap<usize, usize> = BTreeMap::new();
    for (index, ticket) in tickets.iter().enumerate() {
        let times = copy_map.entry(index).and_modify(|n| *n += 1).or_insert(1);

        let winning_numbers = ticket[0].iter()
            .filter(|num| ticket[1].contains(num))
            .count();

        for _ in 0..*times {
            for num in index + 1 .. index + 1 + winning_numbers {
                copy_map.entry(num).and_modify(|n| *n += 1).or_insert(1);
            }
        }
    }

    let total = copy_map.into_iter()
        .map(|tree| tree.1)
        .sum::<usize>();

    total.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_input() {
        let str = "Card 1: 41 48 83 86 17 | 83 86  6 31 17  9 48 53
Card 2: 13 32 20 16 61 | 61 30 68 82 17 32 24 19
Card 3:  1 21 53 59 44 | 69 82 63 72 16 21 14  1
Card 4: 41 92 73 84 69 | 59 84 76 51 58  5 54 83
Card 5: 87 83 26 28 32 | 88 30 70 12 93 22 82 36
Card 6: 31 18 13 56 72 | 74 77 10 23 35 67 36 11";
        
        assert_eq!("30", process_input(str));
    }
}