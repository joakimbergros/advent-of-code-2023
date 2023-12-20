use std::{
    cmp::{self, Ordering},
    collections::BTreeMap,
};

use nom::CompareResult;

fn main() {
    let str = include_str!("part2.txt");
    let sum = process_input(str);
    println!("{sum}");
}

fn process_input(input: &str) -> String {
    let mut hands = parse(input);
    hands.sort();

    let result = hands
        .iter()
        .enumerate()
        .fold(0, |acc, (index, val)| acc + val.bid * (index as u32 + 1));

    result.to_string()
}

fn parse(input: &str) -> Vec<Hand> {
    input
        .lines()
        .map(|line| {
            line.split_once(' ')
                .map(|(c, b)| Hand::from(c, b))
                .expect("should be a valid line")
        })
        .collect()
}

#[derive(Debug, PartialEq, Eq)]
struct Hand {
    cards: Vec<Card>,
    hand_type: HandType,
    bid: u32,
}

impl Hand {
    fn from(cards: &str, bid: &str) -> Self {
        use HandType::*;

        let mapped_cards = cards.chars().map(Card).collect::<Vec<Card>>();

        let mut hash_map: BTreeMap<&Card, u32> =
            mapped_cards.iter().fold(BTreeMap::new(), |mut map, card| {
                map.entry(card).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        // Take J into account and modify the hash accordingly
        let jokers = hash_map
            .iter()
            .filter_map(|(card, count)| match card.0 {
                'J' => Some(count),
                _ => None,
            })
            .sum::<u32>();

        let mut ordered_list = hash_map.values().collect::<Vec<&u32>>();
        ordered_list.sort();

        let hand_type = match (&ordered_list[..], jokers) {
            ([5], _) => FiveOfAKind,
            ([1, 4], 1) => FiveOfAKind,
            ([2, 3], 2) => FiveOfAKind,
            ([2, 3], 3) => FiveOfAKind,
            ([1, 4], 4) => FiveOfAKind,
            ([1, 4], 0) => FourOfAKind,
            ([1, 1, 3], 1) => FourOfAKind,
            ([1, 2, 2], 2) => FourOfAKind,
            ([1, 1, 3], 3) => FourOfAKind,
            ([2, 3], 0) => FullHouse,
            ([1, 2, 2], 1) => FullHouse,
            ([1, 1, 3], 0) => ThreeOfAKind,
            ([1, 1, 1, 2], 1) => ThreeOfAKind,
            ([1, 1, 1, 2], 2) => ThreeOfAKind,
            ([1, 2, 2], 0) => TwoPair,
            ([1, 1, 1, 2], 0) => OnePair,
            ([1, 1, 1, 1, 1], 1) => OnePair,
            ([1, 1, 1, 1, 1], 0) => HighCard,
            _ => unreachable!("should be no more combos"),
        };

        //dbg!(cards, jokers, hand_type);

        Hand {
            cards: mapped_cards,
            hand_type,
            bid: bid.parse::<u32>().expect("should be a valid bid"),
        }
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> Ordering {
        let compare = self.hand_type.cmp(&other.hand_type);
        if compare != Ordering::Equal {
            return compare;
        }

        let mut other_cards = other.cards.iter();
        let mut cards = self.cards.iter();

        loop {
            let card_strength = cards.next().expect("should be a card");
            let other_card_strength = other_cards.next().expect("should be a card");

            let comparison = card_strength.cmp(other_card_strength);
            if comparison == Ordering::Equal {
                continue;
            }

            return comparison;
        }
    }
}

#[derive(Debug, PartialEq, Eq, Clone, Copy)]
struct Card(char);

impl Card {
    fn strength(&self) -> u32 {
        match self.0 {
            'A' => 13,
            'K' => 12,
            'Q' => 11,
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            'J' => 1,
            _ => 0,
        }
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Card {
    fn cmp(&self, other: &Self) -> Ordering {
        let result = self.strength() as i32 - other.strength() as i32;
        match result {
            0 => Ordering::Equal,
            val if val > 0 => Ordering::Greater,
            _ => Ordering::Less,
        }
    }
}

#[derive(Debug, PartialEq, Eq, Copy, Clone)]
enum HandType {
    FiveOfAKind,
    FourOfAKind,
    FullHouse,
    ThreeOfAKind,
    TwoPair,
    OnePair,
    HighCard,
}

impl HandType {
    fn value(&self) -> u32 {
        match self {
            HandType::FiveOfAKind => 6,
            HandType::FourOfAKind => 5,
            HandType::FullHouse => 4,
            HandType::ThreeOfAKind => 3,
            HandType::TwoPair => 2,
            HandType::OnePair => 1,
            HandType::HighCard => 0,
        }
    }
}

impl PartialOrd for HandType {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for HandType {
    fn cmp(&self, other: &Self) -> Ordering {
        self.value().cmp(&other.value())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_type_ord() {
        assert!(HandType::FiveOfAKind > HandType::FourOfAKind);
    }

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand::from("AAAAA", "1");
        let hand2 = Hand::from("AAAJJ", "2");

        dbg!(&hand1, &hand2);

        assert!(hand1 > hand2);
    }

    #[test]
    fn test_hand_typing() {
        use HandType::*;

        let tests = vec![
            // ([5], 0) => FiveOfAKind,
            (Hand::from("AAAAA", "1"), FiveOfAKind),
            // ([1, 4], 1) => FiveOfAKind,
            (Hand::from("AAAAJ", "1"), FiveOfAKind),
            // ([2, 3], 2) => FiveOfAKind,
            (Hand::from("AAAJJ", "1"), FiveOfAKind),
            // ([2, 3], 3) => FiveOfAKind,
            (Hand::from("AAJJJ", "1"), FiveOfAKind),
            // ([1, 4], 4) => FiveOfAKind,
            (Hand::from("AJJJJ", "1"), FiveOfAKind),
            //
            // ([1, 4], 0) => FourOfAKind,
            (Hand::from("AAAAT", "1"), FourOfAKind),
            // ([1, 1, 3], 1) => FourOfAKind,
            (Hand::from("AAAJT", "1"), FourOfAKind),
            // ([1, 2, 2], 2) => FourOfAKind,
            (Hand::from("AAJJT", "1"), FourOfAKind),
            // ([1, 1, 3], 3) => FourOfAKind,
            (Hand::from("AJJJT", "1"), FourOfAKind),
            //
            // ([2, 3], 0) => FullHouse,
            (Hand::from("AATTT", "1"), FullHouse),
            // ([1, 2, 2], 1) => FullHouse,
            (Hand::from("AATTJ", "1"), FullHouse),
            //
            // ([1, 1, 3], 0) => ThreeOfAKind,
            (Hand::from("AAAKT", "1"), ThreeOfAKind),
            // ([1, 1, 1, 2], 1) => ThreeOfAKind,
            (Hand::from("AAKTJ", "1"), ThreeOfAKind),
            // ([1, 1, 1, 2], 2) => ThreeOfAKind,
            (Hand::from("AKTJJ", "1"), ThreeOfAKind),
            // ([1, 2, 2], 0) => TwoPair,
            (Hand::from("AATTQ", "1"), TwoPair),
            // ([1, 1, 1, 2], 0) => OnePair,
            (Hand::from("12344", "1"), OnePair),
            // ([1, 1, 1, 1, 1], 1) => OnePair,
            (Hand::from("1234J", "1"), OnePair),
            // ([1, 1, 1, 1, 1], 1) => HighCard,
            (Hand::from("12345", "1"), HighCard),
        ];

        for test in tests {
            assert_eq!(test.0.hand_type, test.1);
        }
    }

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

        assert_eq!("5905", process_input(str));
    }
}
