use std::collections::{HashMap, HashSet};
use std::fs::File;
use std::io::{BufRead, BufReader};

#[derive(Debug)]
pub struct Card {
    number: usize,
    winning_numbers: HashSet<u32>,
    guessed_numbers: HashSet<u32>,
}

impl Card {
    fn calc_win(&self) -> u32 {
        self.winning_numbers.intersection(&self.guessed_numbers).fold(1u32, |acc, _| acc * 2) / 2
    }
}

pub fn read_input(file_name: &str) -> Vec<Card> {
    let file = File::open(file_name).unwrap();
    let mut reader = BufReader::new(file);
    let mut buf = String::new();

    let mut cards = Vec::new();
    let mut number = 1usize;

    while let Ok(n) = reader.read_line(&mut buf) {
        if n == 0 {
            break;
        }

        let items = &mut buf.trim()[8..]
            .split('|')
            .map(|s| s.split(' ')
                .filter_map(|num| num.trim().parse::<u32>().ok())
                .collect::<HashSet<_>>())
            .into_iter();

        let winning_numbers = items.next().unwrap();
        let guessed_numbers = items.next().unwrap();

        cards.push(Card { number, winning_numbers, guessed_numbers });
        buf = String::new();
        number += 1;
    }

    let distinct_count_of_winning_numbers_sets = cards.iter()
        .map(|card| card.winning_numbers.len())
        .collect::<HashSet<_>>()
        .len();

    let distinct_count_of_guessed_numbers_sets = cards.iter()
        .map(|card| card.guessed_numbers.len())
        .collect::<HashSet<_>>()
        .len();

    assert_eq!(distinct_count_of_winning_numbers_sets, 1);
    assert_eq!(distinct_count_of_guessed_numbers_sets, 1);

    cards
}

fn scratchcards_part_1(filename: &str) -> u32 {
    let cards = read_input(filename);

    cards.iter().map(|card| card.calc_win()).sum()
}

fn scratchcards_part_2(filename: &str) -> u32 {
    let cards = read_input(filename);
    let mut cards_qty = HashMap::<usize, u32>::from_iter(cards.iter().map(|c| (c.number, 1)));

    // cards are sorted by number already
    for card in cards.iter() {
        let guessed_count = card.winning_numbers.intersection(&card.guessed_numbers).count();
        let card_type_count = cards_qty[&card.number];
        for card_num in (card.number + 1)..=(card.number + guessed_count) {
            cards_qty.entry(card_num).and_modify(|count| *count += card_type_count);
        }
    }

    for (k, v) in cards_qty.iter() {
        println!("{k:?}: {v:?}");
    }

    cards_qty.values().map(|x| *x).sum()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn read_input_test() {
        let cards = read_input("inputs/4_input_example.txt");

        for card in cards.iter() {
            println!("{card:?}");
        }
    }

    #[test]
    fn part_1_input_example() {
        let answer = scratchcards_part_1("inputs/4_input_example.txt");

        println!("part 1 - example - answer: {:?}", answer);
        assert_eq!(answer, 13);
    }

    #[test]
    fn part_1_input() {
        let answer = scratchcards_part_1("inputs/4_input.txt");

        println!("part 1 - original - answer: {:?}", answer);
        assert_eq!(answer, 26346);
    }

    #[test]
    fn part_2_input_example() {
        let answer = scratchcards_part_2("inputs/4_input_example.txt");

        println!("part 2 - example - answer: {:?}", answer);
        assert_eq!(answer, 30);
    }

    #[test]
    fn part_2_input() {
        let answer = scratchcards_part_2("inputs/4_input.txt");

        println!("part 2 - original - answer: {:?}", answer);
        assert_eq!(answer, 8467762);
    }
}
