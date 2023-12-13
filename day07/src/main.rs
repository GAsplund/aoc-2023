use std::{collections::BTreeSet, fmt::Debug};

#[derive(Eq, Ord)]
struct Hand {
    pub cards: [char; 5],
    pub bid: u32,
    pub joker: bool,
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards
    }
}

impl Debug for Hand {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(self.cards.iter().collect::<String>().as_str())
    }
}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        match self.value().partial_cmp(&other.value()) {
            Some(core::cmp::Ordering::Equal) => {
                let idx = self.cards.iter().enumerate().map(|(i, _)| i).find(|i| Hand::card_value(self.cards[*i], self.joker).cmp(&Hand::card_value(other.cards[*i], other.joker)) != core::cmp::Ordering::Equal).unwrap_or(0);
                return Some(Hand::card_value(self.cards[idx], self.joker).cmp(&Hand::card_value(other.cards[idx], other.joker)));
            }
            ord => return ord,
        }
    }
}

pub fn joker_val(idx: usize, val: u32) -> u32 {
    match idx {
        11 => 0,
        _ => val,
    }
}

impl Hand {
    pub fn from_str(s: &str, joker: bool) -> Hand {
        let (hand_str, bid_str) = s.split_at(5);
        let c: [char; 5] = hand_str.chars().collect::<Vec<char>>()[0..5].try_into().unwrap();
        Hand{ cards: c, bid: bid_str[1..].parse().unwrap(), joker }
    }

    pub fn value(&self) -> u32 {
        let mut counts: [u32; 15] = [0; 15];
        for card in self.cards {
            counts[Hand::card_value(card, false) as usize] += 1;
        }

        let jokers = if self.joker {
            counts[11]
        } else {0};

        if counts.iter().enumerate().any(|(i, c)| (joker_val(i, jokers) + *c) == 5) {
            7
        } else if counts.iter().enumerate().any(|(i, c)| (joker_val(i, jokers) + *c) == 4) {
            6
        } else if counts.iter().enumerate().any(|(i, c)| (joker_val(i, jokers) + *c) == 3) {
            if counts.iter().enumerate().any(|(i, c)| (joker_val(i, jokers) + *c) == 2) {
                5
            } else {4}
        } else if counts.iter().enumerate().any(|(i, c)| (joker_val(i, jokers) + *c) == 2) {
            if counts.iter().enumerate().filter(|(i, c)| (joker_val(*i, jokers) + *c) == 2).count() == 2 {
                3
            } else {2}
        } else if counts.iter().all(|c| *c == 1 || *c == 0) {
            1
        } else {
            0
        }
    }

    pub fn card_value(card: char, joker: bool) -> u32 {
        match card {
            'A' => 14,
            'K' => 13,
            'Q' => 12,
            'J' => {if joker {1} else {11}},
            'T' => 10,
            '9' => 9,
            '8' => 8,
            '7' => 7,
            '6' => 6,
            '5' => 5,
            '4' => 4,
            '3' => 3,
            '2' => 2,
            _ => 0,
        }
    }
}

fn main() {
    let hands: BTreeSet<Hand> = include_str!("../input.txt").lines().map(|l| Hand::from_str(l, false)).collect();
    //println!("{:?}", hands);
    let winnings: u32 = hands.iter().enumerate().map(|(i, b)| (i+1) as u32*b.bid).sum();
    println!("Solution 1: {}", winnings);

    let hands: BTreeSet<Hand> = include_str!("../input.txt").lines().map(|l| Hand::from_str(l, true)).collect();
    let winnings: u32 = hands.iter().enumerate().map(|(i, b)| (i+1) as u32*b.bid).sum();
    println!("Solution 2: {}", winnings);
}
}
