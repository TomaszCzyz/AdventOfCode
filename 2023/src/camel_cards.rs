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

#[derive(Debug)]
pub struct PlayerHand {
    cards: String,
    cards_mapped: String,
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
                for (self_char, other_char) in self.cards_mapped.chars().zip(other.cards_mapped.chars()) {
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

        match char_occurrences.values().max().unwrap() {
            5 => HandType::FiveOfKind,
            4 => HandType::FourOfKind,
            3 => if char_occurrences.values().any(|&x| x == 2) {
                HandType::FullHouse
            } else {
                HandType::ThreeOfKind
            },
            2 => if char_occurrences.values().filter(|&&x| x == 2).count() == 2 {
                HandType::TwoPairs
            } else {
                HandType::OnePair
            },
            1 => HandType::HighCard,
            _ => panic!()
        }
    }
}

pub fn read_input(file_name: &str) -> Vec<PlayerHand> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let char_map = HashMap::from([
        ('2', 'A'),
        ('3', 'B'),
        ('4', 'C'),
        ('5', 'D'),
        ('6', 'E'),
        ('7', 'F'),
        ('8', 'G'),
        ('9', 'H'),
        ('T', 'I'),
        ('J', 'J'),
        ('Q', 'K'),
        ('K', 'L'),
        ('A', 'M'),
    ]);
    let mut poker_hands = Vec::new();

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let hand = buf[0..=4].to_string();
        let bid = buf[5..].trim().parse::<u32>().unwrap();

        poker_hands.push(PlayerHand {
            cards: hand.clone(),
            cards_mapped: hand.chars().map(|ch| char_map.get(&ch).unwrap()).collect(),
            bid,
        });

        buf = String::new();
    }

    poker_hands
}

fn camel_cards_part_1(filename: &str) -> u32 {
    let mut player_hands = read_input(filename);

    player_hands.sort_unstable_by(|a, b| b.cmp(a));

    player_hands.iter()
        .enumerate()
        .map(|(i, hand)| (i + 1) as u32 * hand.bid)
        .sum()
}

fn camel_cards_part_2(_filename: &str) -> u32 {
    todo!()
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
        assert_eq!(answer, 0);
    }

    #[test]
    fn part_2_input() {
        let answer = camel_cards_part_2("inputs/7_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 0);
    }
}
