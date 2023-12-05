use std::{collections::HashMap, str::FromStr};

use super::day::{Day, DayResult};

pub struct Instance;

struct Part {
    x: u8,
    y: u8,
    value: u16,
    len: u8,
}

impl Part {
    fn neighbours(&self) -> Vec<(u8, u8)> {
        let mut neighbours = Vec::new();
        let y = self.y;

        for x in (self.x.saturating_sub(1))..=(self.x + self.len) {
            if y > 0 {
                neighbours.push((x, y - 1));
            }

            neighbours.push((x, y + 1));
        }

        if self.x > 0 {
            neighbours.push((self.x - 1, y));
        }
        neighbours.push((self.x + self.len, y));

        neighbours
    }
}

struct Schematic {
    parts: Vec<Part>,
    symbols: HashMap<(u8, u8), char>,
}

fn push_part(
    parts: &mut Vec<Part>,
    n_acc: &mut Option<Vec<char>>,
    n_pos: &mut Option<(u8, u8)>,
) -> Result<(), String> {
    if let (Some(acc), Some(p)) = (n_acc.as_ref(), n_pos.as_ref()) {
        let n = acc
            .iter()
            .collect::<String>()
            .parse::<u16>()
            .map_err(|e| format!("{}: {}", e, acc.iter().collect::<String>()))?;
        parts.push(Part {
            x: p.0,
            y: p.1,
            value: n,
            len: acc.len() as u8,
        });
        *n_acc = None;
        *n_pos = None;
    }
    Ok(())
}

impl FromStr for Schematic {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut n_acc: Option<Vec<char>> = None;
        let mut n_pos: Option<(u8, u8)> = None;

        let mut parts = Vec::new();
        let mut symbols = HashMap::new();

        for (y, l) in s.split('\n').enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c.is_ascii_digit() {
                    if let Some(n) = n_acc.as_mut() {
                        n.push(c);
                    } else {
                        n_acc = Some(vec![c]);
                        n_pos = Some((x as u8, y as u8));
                    }
                } else {
                    push_part(&mut parts, &mut n_acc, &mut n_pos)?;
                    if c != '.' {
                        symbols.insert((x as u8, y as u8), c);
                    }
                }
            }
            push_part(&mut parts, &mut n_acc, &mut n_pos)?;
        }

        Ok(Schematic { parts, symbols })
    }
}

impl Schematic {
    fn part_numbers(&self) -> Vec<u16> {
        let mut parts = Vec::new();

        for part in self.parts.iter() {
            for neighbour in part.neighbours() {
                if self.symbols.contains_key(&neighbour) {
                    parts.push(part.value);
                    break;
                }
            }
        }

        parts
    }

    fn gears(&self) -> Vec<u32> {
        let mut gears = HashMap::new();

        for part in self.parts.iter() {
            for neighbour in part.neighbours() {
                if let Some(s) = self.symbols.get(&neighbour) {
                    if *s == '*' {
                        gears
                            .entry(neighbour)
                            .and_modify(|p: &mut Vec<u16>| p.push(part.value))
                            .or_insert(vec![part.value]);
                    }
                }
            }
        }

        gears
            .values()
            .filter(|l| l.len() == 2)
            .map(|l| l.iter().map(|i| *i as u32).product::<u32>())
            .collect()
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let schematic = lines.join("\n").parse::<Schematic>()?;

        let part1: u32 = schematic.part_numbers().iter().map(|i| *i as u32).sum();
        let part2: u32 = schematic.gears().iter().sum();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
