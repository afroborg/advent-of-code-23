use core::num;
use std::collections::HashMap;

#[derive(Debug, PartialEq, Eq)]
struct Card(u8, char);

impl Ord for Card {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        self.0.cmp(&other.0).reverse()
    }
}

impl PartialOrd for Card {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl From<char> for Card {
    fn from(c: char) -> Self {
        let value = match c {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'T' => 10,
            'J' => 1,
            _ => c.to_digit(10).unwrap() as u8,
        };

        Self(value, c)
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u16,
}

#[derive(Debug, Copy, Clone)]
enum HandType {
    HighCard = 0,
    OnePair = 1,
    TwoPairs = 2,
    ThreeOfAKind = 3,
    FullHouse = 4,
    FourOfAKind = 5,
    FiveOfAKind = 6,
}

impl From<&Hand> for HandType {
    fn from(hand: &Hand) -> Self {
        let map = hand.cards.iter().fold(HashMap::new(), |mut map, card| {
            *map.entry(card.0).or_insert(0) += 1;
            map
        });

        let mut values = if let Some(jokers) = map.get(&1) {
            if jokers == &5 {
                vec!["5".to_string()]
            } else {
                let mut no_jokers = map
                    .iter()
                    .filter_map(|(k, v)| (k != &1).then_some(v))
                    .collect::<Vec<_>>();

                no_jokers.sort();

                no_jokers
                    .iter()
                    .enumerate()
                    .map(|(i, v)| {
                        if i == no_jokers.len() - 1 {
                            **v + jokers
                        } else {
                            **v
                        }
                    })
                    .map(|v| v.to_string())
                    .collect::<Vec<_>>()
            }
        } else {
            map.values().map(|v| v.to_string()).collect::<Vec<_>>()
        };

        values.sort();

        let values = values.join("");

        match values.as_str() {
            "5" => HandType::FiveOfAKind,
            "14" => HandType::FourOfAKind,
            "23" => HandType::FullHouse,
            "113" => HandType::ThreeOfAKind,
            "122" => HandType::TwoPairs,
            "1112" => HandType::OnePair,
            "11111" => HandType::HighCard,
            _ => panic!("invalid hand"),
        }
    }
}

pub fn process(input: &str) -> String {
    let hands = input.lines().map(|line| {
        let (cards, bid) = line.split_once(' ').expect("invalid input");
        let bid = bid.parse::<u16>().expect("invalid input");
        let cards = cards.chars().map(Card::from).collect::<Vec<_>>();

        Hand { cards, bid }
    });

    let mut hands = hands
        .map(|h| {
            let hand_type = HandType::from(&h);

            (hand_type, h)
        })
        .collect::<Vec<_>>();

    hands.sort_by_key(|h| (h.0 as u8, h.1.cards.iter().map(|c| c.0).collect::<Vec<_>>()));

    let score = hands
        .iter()
        .enumerate()
        .map(|(i, (_, h))| (i + 1) * h.bid as usize)
        .sum::<usize>();

    score.to_string()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_process() {
        let input = "32T3K 765
T55J5 684
KK677 28
KTJJT 220
QQQJA 483";
        assert_eq!(process(input), "5905");
    }
}
