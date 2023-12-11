use std::collections::HashSet;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, Hash, PartialEq, Eq, PartialOrd, Ord)]
struct Coord(usize, usize);

struct Galaxy(HashSet<Coord>);

impl Galaxy {
    fn new(lines: &[String]) -> Self {
        let mut set = HashSet::new();
        for (y, l) in lines.iter().enumerate() {
            for (x, c) in l.chars().enumerate() {
                if c == '#' {
                    set.insert(Coord(x, y));
                }
            }
        }
        Galaxy(set)
    }

    fn distances(&self, expand: usize) -> usize {
        let max_x = self.0.iter().map(|c| c.0).max().unwrap();
        let max_y = self.0.iter().map(|c| c.1).max().unwrap();

        let mut empty_x = HashSet::new();
        for x in 0..max_x {
            if self.0.iter().find(|c| c.0 == x).is_none() {
                empty_x.insert(x);
            }
        }

        let mut empty_y = HashSet::new();
        for y in 0..max_y {
            if self.0.iter().find(|c| c.1 == y).is_none() {
                empty_y.insert(y);
            }
        }
        self.0
            .iter()
            .flat_map(|a| {
                self.0
                    .iter()
                    .filter_map(move |b| if a < b { Some((a, b)) } else { None })
            })
            .map(|(a, b)| {
                let range_x = a.0.min(b.0)..a.0.max(b.0);
                let range_y = a.1.min(b.1)..a.1.max(b.1);

                range_x.len()
                    + range_y.len()
                    + (empty_x.iter().filter(|x| range_x.contains(x)).count()
                        + empty_y.iter().filter(|y| range_y.contains(y)).count())
                        * (expand - 1)
            })
            .sum()
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let galaxy = Galaxy::new(&lines);
        Ok(DayResult {
            part1: galaxy.distances(2).to_string(),
            part2: Some(galaxy.distances(1000000).to_string()),
        })
    }
}
