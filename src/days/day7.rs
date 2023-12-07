use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, PartialEq, Eq, PartialOrd, Ord, Hash, Clone, Copy)]
enum Card {
    Joker,
    Number(u8),
    Jack,
    Queen,
    King,
    Ace,
}

impl Card {
    fn from_char(c: char) -> Self {
        match c {
            'A' => Card::Ace,
            'K' => Card::King,
            'Q' => Card::Queen,
            'J' => Card::Jack,
            'T' => Card::Number(10),
            n => Card::Number(n.to_digit(10).unwrap() as u8),
        }
    }
}

#[derive(Debug)]
struct Hand {
    cards: Vec<Card>,
    bid: u32,
}

impl FromStr for Hand {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (cards, bid) = s.split_once(' ').ok_or("Missing space")?;
        let cards = cards.chars().map(Card::from_char).collect();
        let bid = bid.parse::<u32>().map_err(|e| format!("{}: {}", e, bid))?;

        Ok(Hand { cards, bid })
    }
}

impl PartialEq for Hand {
    fn eq(&self, other: &Self) -> bool {
        self.cards == other.cards && self.bid == other.bid
    }
}

impl Eq for Hand {}

impl PartialOrd for Hand {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        Some(self.cmp(other))
    }
}

impl Ord for Hand {
    fn cmp(&self, other: &Self) -> std::cmp::Ordering {
        match self.type_rank().cmp(&other.type_rank()) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }

        match self.cards.cmp(&other.cards) {
            core::cmp::Ordering::Equal => {}
            ord => return ord,
        }
        self.bid.cmp(&other.bid)
    }
}

impl Hand {
    fn card_counts(&self) -> HashMap<&Card, usize> {
        self.cards
            .iter()
            .sorted()
            .group_by(|s| *s)
            .into_iter()
            .map(|(c, g)| (c, g.count()))
            .collect()
    }

    fn is_five_of_kind(&self) -> bool {
        self.max_with_jokers() == 5
    }

    fn is_four_of_kind(&self) -> bool {
        self.max_with_jokers() == 4
    }

    fn is_three_of_kind(&self) -> bool {
        !self.is_full_house() && self.max_with_jokers() == 3
    }

    fn is_full_house(&self) -> bool {
        if self.joker_count() > 1 {
            return false;
        }

        if self.joker_count() == 1 {
            let mut counts = self.card_counts();
            counts.remove(&Card::Joker);
            return self.card_counts().values().filter(|l| **l == 2).count() == 2;
        }

        self.card_counts().values().max() == Some(&3)
            && self.card_counts().values().min() == Some(&2)
    }

    fn is_two_pair(&self) -> bool {
        if self.joker_count() > 0 {
            return false;
        }

        self.card_counts().values().filter(|l| **l == 2).count() == 2
    }

    fn is_pair(&self) -> bool {
        !self.is_two_pair() && !self.is_full_house() && self.max_with_jokers() == 2
    }

    fn type_rank(&self) -> u8 {
        if self.is_pair() {
            return 1;
        }
        if self.is_two_pair() {
            return 2;
        }
        if self.is_three_of_kind() {
            return 3;
        }
        if self.is_full_house() {
            return 4;
        }
        if self.is_four_of_kind() {
            return 5;
        }
        if self.is_five_of_kind() {
            return 6;
        }
        0
    }

    fn with_jokers(&self) -> Self {
        let cards = self
            .cards
            .iter()
            .map(|&c| if c == Card::Jack { Card::Joker } else { c })
            .collect();
        Hand {
            cards,
            bid: self.bid,
        }
    }

    fn joker_count(&self) -> usize {
        self.card_counts().get(&Card::Joker).copied().unwrap_or(0)
    }

    fn max_with_jokers(&self) -> usize {
        let max = self
            .card_counts()
            .iter()
            .filter(|(c, _)| ***c != Card::Joker)
            .map(|(_, i)| *i)
            .max()
            .unwrap_or(0);
        max + self.joker_count()
    }
}

fn total_winnings(hands: &[Hand]) -> u32 {
    let mut vec: Vec<_> = hands.iter().collect();
    vec.sort();
    let ranked: Vec<_> = vec.iter().zip(1..).collect();
    ranked.iter().map(|(h, r)| h.bid * r).sum()
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let parsed: Vec<_> = lines
            .iter()
            .map(|l| l.parse::<Hand>())
            .collect::<Result<_, _>>()?;

        let with_jokers: Vec<_> = parsed.iter().map(|h| h.with_jokers()).collect();

        Ok(DayResult {
            part1: total_winnings(&parsed).to_string(),
            part2: Some(total_winnings(&with_jokers).to_string()),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn five_of_a_kind() {
        assert!(dbg!(Hand::from_str("JJJJJ 0").unwrap().with_jokers().type_rank()) == 6);
        assert!(dbg!(Hand::from_str("JJJJA 0").unwrap().with_jokers().type_rank()) == 6);
        assert!(dbg!(Hand::from_str("JJAAJ 0").unwrap().with_jokers().type_rank()) == 6);
        assert!(dbg!(Hand::from_str("AJAJA 0").unwrap().with_jokers().type_rank()) == 6);
        assert!(dbg!(Hand::from_str("AAAAJ 0").unwrap().with_jokers().type_rank()) == 6);
        assert!(dbg!(Hand::from_str("AAAAA 0").unwrap().with_jokers().type_rank()) == 6);
    }
}
