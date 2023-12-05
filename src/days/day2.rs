use std::str::FromStr;

use super::day::{Day, DayResult};

pub struct Instance;

struct Handful {
    red: u8,
    green: u8,
    blue: u8,
}

impl Handful {
    fn is_possible(&self, total_red: u8, total_green: u8, total_blue: u8) -> bool {
        total_red >= self.red && total_green >= self.green && total_blue >= self.blue
    }
}

struct Game {
    id: u8,
    handfuls: Vec<Handful>,
}

impl FromStr for Handful {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;

        for colour_amount in s.split(", ") {
            let mut split = colour_amount.split(' ');
            let amount = split
                .next()
                .ok_or("missing amount")?
                .parse::<u8>()
                .map_err(|e| e.to_string())?;
            let colour = split.next().ok_or("missing colour")?;

            match colour {
                "red" => red = amount,
                "green" => green = amount,
                "blue" => blue = amount,
                other => Err(format!("unknown colour {}", other))?,
            }
        }

        Ok(Handful { red, green, blue })
    }
}

impl FromStr for Game {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut split = s.split(": ");

        let game = split.next().unwrap();

        let id = game[5..game.len()]
            .parse::<u8>()
            .map_err(|e| e.to_string())?;

        let handfuls = split
            .next()
            .ok_or("missing handfuls")?
            .split("; ")
            .map(|s| s.parse::<Handful>())
            .collect::<Result<Vec<_>, _>>()?;

        Ok(Game { id, handfuls })
    }
}

impl Game {
    fn power(&self) -> u32 {
        let (r, g, b) = self.handfuls.iter().fold((0, 0, 0), |(r, g, b), c| {
            (
                r.max(c.red as u32),
                g.max(c.green as u32),
                b.max(c.blue as u32),
            )
        });
        r * g * b
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let target_red = 12;
        let target_green = 13;
        let target_blue = 14;

        let games = lines
            .iter()
            .map(|s| s.parse::<Game>())
            .collect::<Result<Vec<_>, _>>()?;

        let part1 = games
            .iter()
            .filter(|g| {
                g.handfuls
                    .iter()
                    .all(|h| h.is_possible(target_red, target_green, target_blue))
            })
            .map(|g| g.id as u16)
            .sum::<u16>();

        let part2 = games.iter().map(|g| g.power()).sum::<u32>();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
