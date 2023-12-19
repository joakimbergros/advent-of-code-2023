use std::{cmp::Ordering, collections::BTreeMap};

fn main() {
    let str = include_str!("test.txt");
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
        let mapped_cards = cards.chars().map(Card).collect::<Vec<Card>>();

        let hash_map: BTreeMap<&Card, u32> =
            mapped_cards.iter().fold(BTreeMap::new(), |mut map, card| {
                map.entry(card).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        let mut ordered_list = hash_map.values().collect::<Vec<&u32>>();
        ordered_list.sort();

        let hand_type = match ordered_list[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        };

        Hand {
            cards: mapped_cards,
            hand_type,
            bid: bid.parse::<u32>().expect("should be a valid bid"),
        }
    }

    fn get_type(&self) -> HandType {
        let hash_map: BTreeMap<&Card, u32> =
            self.cards.iter().fold(BTreeMap::new(), |mut map, card| {
                map.entry(card).and_modify(|count| *count += 1).or_insert(1);
                map
            });

        let ordered_list = dbg!(hash_map.values().collect::<Vec<&u32>>());
        match ordered_list[..] {
            [5] => HandType::FiveOfAKind,
            [1, 4] => HandType::FourOfAKind,
            [2, 3] => HandType::FullHouse,
            [1, 1, 3] => HandType::ThreeOfAKind,
            [1, 2, 2] => HandType::TwoPair,
            [1, 1, 1, 2] => HandType::OnePair,
            _ => HandType::HighCard,
        }
    }

    fn get_value(&self) -> u32 {
        self.cards.iter().map(|card| card.strength()).sum()
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

        Ordering::Less
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

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfAKind = 0,
    FourOfAKind = 1,
    FullHouse = 2,
    ThreeOfAKind = 3,
    TwoPair = 4,
    OnePair = 5,
    HighCard = 6,
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hand_ordering() {
        let hand1 = Hand::from("AAAAA", "1");
        let hand2 = Hand::from("AAATT", "2");

        dbg!(&hand1, &hand2);

        assert!(hand1 > hand2);
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

        assert_eq!("288", process_input(str));
    }
}
