#![allow(dead_code)]

use std::cmp::Ordering;
use std::collections::HashMap;
use std::fmt::Debug;
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord)]
enum HandType {
    FiveOfKind = 0,
    FourOfKind = 1,
    FullHouse = 2,
    ThreeOfKind = 3,
    TwoPairs = 4,
    OnePair = 5,
    HighCard = 6,
}

pub trait Deck {
    fn has_jokers(&self) -> bool;
    fn card_value(&self, ch: char) -> u8;
}

pub struct SimpleDeck;

impl Deck for SimpleDeck {
    fn has_jokers(&self) -> bool {
        false
    }

    fn card_value(&self, ch: char) -> u8 {
        if ch.is_digit(10) {
            ch.to_digit(10).unwrap() as u8
        } else {
            match ch {
                'T' => 10,
                'J' => 11,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!()
            }
        }
    }
}

pub struct DeckWithJokers;

impl Deck for DeckWithJokers {
    fn has_jokers(&self) -> bool {
        true
    }

    fn card_value(&self, ch: char) -> u8 {
        if ch.is_digit(10) {
            ch.to_digit(10).unwrap() as u8
        } else {
            match ch {
                'T' => 10,
                'J' => 1,
                'Q' => 12,
                'K' => 13,
                'A' => 14,
                _ => panic!()
            }
        }
    }
}

#[derive(Debug)]
pub struct PlayerHand {
    has_jokers: bool,
    cards: String,
    cards_values: [u8; 5],
    bid: u32,
}

impl Eq for PlayerHand {}

impl PartialEq<Self> for PlayerHand {
    fn eq(&self, other: &Self) -> bool {
        self.get_hand_type() == other.get_hand_type()
    }
}

impl PartialOrd<Self> for PlayerHand {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for PlayerHand {
    fn cmp(&self, other: &Self) -> Ordering {
        match self.get_hand_type().cmp(&other.get_hand_type()) {
            Ordering::Equal => {
                for (self_char, other_char) in self.cards_values.iter().zip(other.cards_values.iter()) {
                    match other_char.cmp(&self_char) {
                        Ordering::Equal => continue,
                        t => return t,
                    }
                }
                unreachable!();
            }
            t => t
        }
    }
}

impl PlayerHand {
    fn get_hand_type(&self) -> HandType {
        let char_occurrences = self.cards.chars()
            .fold(HashMap::new(), |mut acc, c| {
                *acc.entry(c).or_insert(0u32) += 1;
                acc
            });


        let occurrences = if self.has_jokers {
            substitute_jokers(char_occurrences)
        } else {
            char_occurrences
        };

        get_hand_type_no_jokers(occurrences)
    }
}

fn substitute_jokers(cards_occurrences: HashMap<char, u32>) -> HashMap<char, u32> {
    if let Some(jokers_count) = cards_occurrences.get(&'J') {
        let most_common_card = cards_occurrences.iter()
            .filter(|(&ch, _)| ch != 'J')
            .max_by(|(_, &count1), (_, &count2)| count1.cmp(&count2))
            .map(|(&ch, _)| ch);

        match most_common_card {
            None => cards_occurrences,
            Some(card) => {
                let mut modified_cards_occurrences = cards_occurrences.clone();
                modified_cards_occurrences.entry('J').and_modify(|count| *count = 0);
                modified_cards_occurrences.entry(card).and_modify(|count| *count += *jokers_count);
                modified_cards_occurrences
            }
        }
    } else {
        cards_occurrences
    }
}

fn get_hand_type_no_jokers(cards_occurrences: HashMap<char, u32>) -> HandType {
    match cards_occurrences.values().max().unwrap() {
        5 => HandType::FiveOfKind,
        4 => HandType::FourOfKind,
        3 => if cards_occurrences.values().any(|&x| x == 2) {
            HandType::FullHouse
        } else {
            HandType::ThreeOfKind
        },
        2 => if cards_occurrences.values().filter(|&&x| x == 2).count() == 2 {
            HandType::TwoPairs
        } else {
            HandType::OnePair
        },
        1 => HandType::HighCard,
        _ => panic!()
    }
}

pub fn read_input(file_name: &str, deck: impl Deck) -> Vec<PlayerHand> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();
    let mut poker_hands = Vec::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let hand = buf[0..=4].to_string();
        let bid = buf[5..].trim().parse::<u32>().unwrap();

        poker_hands.push(PlayerHand {
            has_jokers: deck.has_jokers(),
            cards: hand.clone(),
            cards_values: hand.chars().map(|ch| deck.card_value(ch)).collect::<Vec<u8>>().try_into().unwrap(),
            bid,
        });

        buf = String::new();
    }

    poker_hands
}

fn camel_cards_part_1(filename: &str) -> u32 {
    let mut player_hands = read_input(filename, SimpleDeck);

    player_hands.sort_unstable_by(|a, b| b.cmp(a));

    player_hands.iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

fn camel_cards_part_2(filename: &str) -> u32 {
    let mut player_hands = read_input(filename, DeckWithJokers);

    player_hands.sort_unstable_by(|a, b| b.cmp(a));

    player_hands.iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn part_1_example_input() {
        let answer = camel_cards_part_1("inputs/7_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 6440);
    }

    #[test]
    fn part_1_input() {
        let answer = camel_cards_part_1("inputs/7_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 250474325);
    }

    #[test]
    fn part_2_input_example() {
        let answer = camel_cards_part_2("inputs/7_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 5905);
    }

    #[test]
    fn part_2_input() {
        let answer = camel_cards_part_2("inputs/7_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 248909434);
    }
}
