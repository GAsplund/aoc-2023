use std::collections::BTreeSet;

fn read_file() -> Vec<String> {
    include_str!("../input.txt")
        .lines()
        .map(|l| l.to_string())
        .collect()
}

fn winning_duplicates(cards: &Vec<(usize, BTreeSet<u32>, BTreeSet<u32>)>) -> usize {
    let queue: Vec<usize> = cards.iter().map(|c| c.0 - 1).clone().collect();
    let mut won_cards: Vec<usize> = vec![1; cards.len()];

    for t in queue.iter() {
        let mul = won_cards[*t];

        let c = &cards[*t];
        let winning = read_winning(c.0, &c.1, &c.2);
        for o in winning.iter() {
            won_cards[*o] += mul;
        }
    }
    won_cards.iter().sum()
}

fn read_winning(id: usize, winning: &BTreeSet<u32>, gotten: &BTreeSet<u32>) -> Vec<usize> {
    (id..(id + winning.intersection(&gotten).count())).collect()
}

fn read_numbers(numbers: &str) -> BTreeSet<u32> {
    numbers
        .split_ascii_whitespace()
        .map(|n| n.parse().unwrap())
        .collect()
}

fn read_card(card: &str) -> (usize, BTreeSet<u32>, BTreeSet<u32>) {
    let (card_id, numbers) = card.split_once(':').unwrap();
    let (winning, gotten) = numbers.split_once('|').unwrap();
    let id = card_id
        .split_ascii_whitespace()
        .last()
        .unwrap()
        .parse()
        .unwrap();
    (id, read_numbers(winning), read_numbers(gotten))
}

fn score(winning: &BTreeSet<u32>, gotten: &BTreeSet<u32>) -> u32 {
    let cnt = gotten.intersection(winning).count();
    if cnt > 0 {
        2_u32.pow(cnt as u32 - 1)
    } else {
        0
    }
}

fn main() {
    let scores: u32 = read_file()
        .iter()
        .map(|c| {
            let r = read_card(c);
            score(&r.1, &r.2)
        })
        .sum();
    println!("Solution 1: {}", scores);
    let cards = read_file().iter().map(|c| read_card(c)).collect();
    let scores_won = winning_duplicates(&cards);
    println!("Solution 2: {}", scores_won);
}
