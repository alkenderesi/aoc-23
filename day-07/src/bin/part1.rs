use std::cmp::Ordering;
use std::collections::HashMap;
use std::iter::zip;

fn main() {
    let input = include_str!("./input1.txt");
    let output = part1(input);
    dbg!(output);
}

fn part1(input: &str) -> String {
    let mut hands = process_input(input);

    hands.sort_by(|a, b| a.cmp(b));

    let total_winnings = hands
        .iter()
        .enumerate()
        .map(|(i, hand)| hand.bid * (i + 1) as u128)
        .sum::<u128>();

    total_winnings.to_string()
}

struct Card {
    rank: char,
}

impl Card {
    const RANKS: [char; 13] = [
        '2', '3', '4', '5', '6', '7', '8', '9', 'T', 'J', 'Q', 'K', 'A',
    ];

    fn strength(&self) -> usize {
        Self::RANKS.iter().position(|&c| self.rank == c).unwrap()
    }

    fn from_rank(rank: &char) -> Self {
        if !Self::RANKS.contains(rank) {
            panic!("Invalid rank: {rank}");
        }
        Self { rank: *rank }
    }
}

struct Hand {
    cards: Vec<Card>,
    bid: u128,
    strength: usize,
}

impl Hand {
    const TYPES: [[(usize, usize); 5]; 6] = [
        [(5, 0), (4, 0), (3, 0), (2, 1), (1, 3)], // One pair
        [(5, 0), (4, 0), (3, 0), (2, 2), (1, 1)], // Two pair
        [(5, 0), (4, 0), (3, 1), (2, 0), (1, 2)], // Three of a kind
        [(5, 0), (4, 0), (3, 1), (2, 1), (1, 0)], // Full house
        [(5, 0), (4, 1), (3, 0), (2, 0), (1, 1)], // Four of a kind
        [(5, 1), (4, 0), (3, 0), (2, 0), (1, 0)], // Five of a kind
    ];

    fn cmp(&self, other: &Self) -> Ordering {
        if self.strength < other.strength {
            return Ordering::Less;
        }
        if self.strength > other.strength {
            return Ordering::Greater;
        }

        for (card, other_card) in zip(&self.cards, &other.cards) {
            if card.strength() < other_card.strength() {
                return Ordering::Less;
            }
            if card.strength() > other_card.strength() {
                return Ordering::Greater;
            }
        }

        Ordering::Equal
    }

    fn update_strength(&mut self) {
        let mut rank_counter = HashMap::new();

        for card in &self.cards {
            rank_counter
                .entry(&card.rank)
                .and_modify(|count| *count += 1)
                .or_insert(1);
        }

        let mut match_counter: HashMap<usize, usize> =
            HashMap::from_iter((1..self.cards.len() + 1).into_iter().map(|i| (i, 0)));

        for rank_count in rank_counter.values() {
            match_counter
                .entry(*rank_count)
                .and_modify(|count| *count += 1);
        }

        let mut full_match;
        let mut updated_strength = 0;

        for (strength, hand_type) in Self::TYPES.iter().enumerate() {
            full_match = true;

            for (card_match, count) in hand_type {
                if &match_counter[card_match] != count {
                    full_match = false;
                    break;
                }
            }

            if full_match {
                updated_strength = strength + 1;
                break;
            }
        }

        self.strength = updated_strength;
    }

    fn from_line(line: &str) -> Self {
        let (card_text, bid_text) = match line.split_once(' ') {
            Some((card_text, bid_text)) => (card_text, bid_text),
            None => panic!("No space found in line: {line}"),
        };

        let bid = match bid_text.parse::<u128>() {
            Ok(bid) => bid,
            Err(_e) => panic!("No parsable bid in: {bid_text}"),
        };

        let cards = card_text
            .chars()
            .map(|rank| Card::from_rank(&rank))
            .collect();

        let mut hand = Self {
            cards,
            bid,
            strength: 0,
        };

        hand.update_strength();

        hand
    }
}

fn process_input(input: &str) -> Vec<Hand> {
    input.lines().map(|line| Hand::from_line(line)).collect()
}
