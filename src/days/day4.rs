use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

use super::day::{Day, DayResult};

pub struct Instance;

struct Card {
    id: u8,
    winners: Vec<u8>,
    numbers: Vec<u8>,
}

impl FromStr for Card {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");
        let card_id = split.next().unwrap();
        let winners_numbers = split.next().ok_or("missing numbers")?;

        let id = card_id[5..card_id.len()]
            .replace(' ', "")
            .parse::<u8>()
            .map_err(|e| e.to_string())?;

        let mut i = 0;
        let mut is_winners = true;

        let mut winners = Vec::new();
        let mut numbers = Vec::new();

        while i < winners_numbers.len() {
            if &winners_numbers[i..=i] == "|" {
                is_winners = false;
                i += 2;
            }
            let n = winners_numbers[i..i + 2]
                .replace(' ', "")
                .parse::<u8>()
                .map_err(|e| format!("{}: {}", e, &winners_numbers[i..i + 2]))?;

            if is_winners {
                winners.push(n);
            } else {
                numbers.push(n);
            }

            i += 3;
        }

        Ok(Card {
            id,
            winners,
            numbers,
        })
    }
}

impl Card {
    fn wins(&self) -> u8 {
        let winners: HashSet<&u8> = HashSet::from_iter(self.winners.iter());
        let numbers: HashSet<&u8> = HashSet::from_iter(self.numbers.iter());
        winners.intersection(&numbers).count() as u8
    }

    fn score(&self) -> u16 {
        if self.wins() == 0 {
            0
        } else {
            1 << (self.wins() - 1)
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let cards = lines
            .iter()
            .map(|l| l.parse::<Card>())
            .collect::<Result<Vec<_>, _>>()?;

        let total_score = cards.iter().map(|c| c.score()).sum::<u16>();

        let mut card_counts = HashMap::new();

        for card in cards.iter().rev() {
            let mut total = 1;
            for w in 1..=card.wins() {
                let id = card.id + w;
                total += card_counts[&id];
            }

            card_counts.insert(card.id, total);
        }

        let total_cards: u32 = card_counts.values().sum();

        Ok(DayResult {
            part1: total_score.to_string(),
            part2: Some(total_cards.to_string()),
        })
    }
}
