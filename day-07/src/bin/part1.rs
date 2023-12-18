use std::cmp::Ordering;

fn main() {
    let str = include_str!("part1.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    dbg!(parse(input));
    "".to_string()
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            let mut line_split = line.split(' ');
            Hand {
                cards: line_split
                    .next()
                    .expect("should be the cards")
                    .chars()
                    .map(|c| Card(c))
                    .collect(),
                bid: line_split
                    .last()
                    .expect("shomsguld be the bid")
                    .parse::<u32>()
                    .expect("should be a number"),
            }
        })
        .collect()
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl Hand {
    fn strength(&self) -> u32 {
        0
    }

    fn get_value(&self) -> u32 {
        self.cards.iter().map(|card| card.get_value()).sum()
    }
}

#[derive(Debug, PartialEq)]
struct Card(char);

impl Card {
    fn get_value(&self) -> u32 {
        match self.0 {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'J' => 10,
            'T' => 9,
            '9' => 8,
            '8' => 7,
            '7' => 6,
            '6' => 5,
            '5' => 4,
            '4' => 3,
            '3' => 2,
            '2' => 1,
            _ => unreachable!("no card with other values should eixst"),
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        let result = self.get_value() as i32 - other.get_value() as i32;
        match result {
            0 => Some(Ordering::Equal),
            val if val > 0 => Some(Ordering::Greater),
            _ => Some(Ordering::Less),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_card_ordering() {
        let card1 = Card('A');
        let card2 = Card('T');

        assert!(card1 > card2);
    }

    #[test]
    fn test_input() {
        let str = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483
";

        assert_eq!("288", process_input(str));
    }
}
