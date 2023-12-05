use std::collections::HashMap;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, Clone, PartialEq, Eq)]
struct Range {
    from: u64,
    length: u64,
}

impl Range {
    fn from_length(from: u64, length: u64) -> Self {
        Range { from, length }
    }

    fn from_bounds(from: u64, to_excl: u64) -> Self {
        Range {
            from,
            length: to_excl - from,
        }
    }

    fn to_excl(&self) -> u64 {
        self.from + self.length
    }
}

#[derive(Debug)]
struct MapRange {
    source: u64,
    destination: u64,
    range: u64,
}

impl MapRange {
    fn source_end_excl(&self) -> u64 {
        self.source + self.range
    }

    fn try_map(&self, v: u64) -> Option<u64> {
        if (self.source..self.source_end_excl()).contains(&v) {
            Some(self.destination + v - self.source)
        } else {
            None
        }
    }

    fn map_range(&self, r: &Range) -> (Vec<Range>, Option<Range>) {
        let mut unmapped = Vec::new();
        let mut mapped = None;
        if r.from < self.source {
            let to_excl = r.to_excl().min(self.source);
            unmapped.push(Range::from_bounds(r.from, to_excl))
        }

        if r.to_excl() > self.source_end_excl() {
            let from = r.from.max(self.source_end_excl());
            unmapped.push(Range::from_bounds(from, r.to_excl()))
        }

        let from = r.from.max(self.source);
        let to_excl = r.to_excl().min(self.source_end_excl());
        let length = to_excl.checked_sub(from);

        if let (Some(mapped_from), Some(length)) = (self.try_map(from), length) {
            mapped = Some(Range::from_length(mapped_from, length))
        }

        (unmapped, mapped)
    }
}
#[derive(Debug)]
struct Map {
    to: String,
    ranges: Vec<MapRange>,
}

impl Map {
    fn map_range(&self, range: Range) -> Vec<Range> {
        let mut current_ranges = vec![range];
        let mut all_mapped = Vec::new();

        for r in &self.ranges {
            let mut new_ranges = Vec::new();
            for c in &current_ranges {
                let (unmapped, mapped) = r.map_range(c);

                new_ranges.extend(unmapped);
                all_mapped.extend(mapped);
            }
            current_ranges = new_ranges;
        }

        all_mapped.extend(current_ranges);

        all_mapped
    }
}

#[derive(Debug)]
struct Almanac {
    seeds: Vec<u64>,
    maps: HashMap<String, Map>,
}

impl Almanac {
    fn from_lines(lines: &[String]) -> Result<Almanac, String> {
        let mut lines = lines.iter();

        let seeds_line = lines.next().ok_or("missing seeds")?;
        let seeds = seeds_line
            .split(' ')
            .skip(1)
            .map(|i| i.parse::<u64>())
            .collect::<Result<Vec<_>, _>>()
            .map_err(|e| format!("{} {}", e, seeds_line))?;

        lines.next();

        let mut maps = HashMap::new();
        let mut ranges = &mut Vec::new();

        for line in lines {
            if line.is_empty() {
                continue;
            } else if !line.chars().next().map_or(false, |c| c.is_ascii_digit()) {
                let mut split = line.split('-');

                let from = split.next().unwrap().to_owned();
                split.next();
                let to_raw = split
                    .next()
                    .ok_or(format!("missing mapping to: {}", line))?;
                let to = to_raw[..to_raw.len() - 5].to_owned();

                let map = maps.entry(from).or_insert(Map {
                    to,
                    ranges: Vec::new(),
                });
                ranges = &mut map.ranges;
            } else {
                let mut split = line.split(' ');
                let destination = split
                    .next()
                    .unwrap()
                    .parse::<u64>()
                    .map_err(|e| format!("{}: {}", e, line))?;
                let source = split
                    .next()
                    .ok_or(format!("missing source: {}", line))?
                    .parse::<u64>()
                    .map_err(|e| format!("{}: {}", e, line))?;
                let range = split
                    .next()
                    .ok_or(format!("missing range: {}", line))?
                    .parse::<u64>()
                    .map_err(|e| format!("{}: {}", e, line))?;

                let range = MapRange {
                    source,
                    destination,
                    range,
                };
                ranges.push(range);
            }
        }

        Ok(Almanac { seeds, maps })
    }

    fn lowest_seeds(&self, seeds: Vec<Range>) -> u64 {
        let mut current = "seed";

        let mut current_values = seeds;

        while let Some(map) = self.maps.get(current) {
            let mut new_values = Vec::new();
            for value in current_values {
                new_values.extend(map.map_range(value));
            }

            current_values = new_values;
            current = map.to.as_str();
        }

        current_values.into_iter().map(|c| c.from).min().unwrap()
    }

    fn lowest(&self) -> u64 {
        let seeds: Vec<_> = self
            .seeds
            .iter()
            .map(|a| Range::from_length(a.to_owned(), 1))
            .collect();

        self.lowest_seeds(seeds)
    }

    fn lowest_range(&self) -> u64 {
        let seeds: Vec<_> = self
            .seeds
            .chunks(2)
            .map(|c| match c {
                [a, b] => Range::from_length(a.to_owned(), b.to_owned()),
                _ => panic!(),
            })
            .collect();

        self.lowest_seeds(seeds)
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let almanac = Almanac::from_lines(&lines)?;

        let part1 = almanac.lowest().to_string();
        let part2 = almanac.lowest_range().to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn map_range_before() {
        let range = Range::from_length(1, 10);
        let map_range = MapRange {
            source: 100,
            destination: 10,
            range: 11,
        };

        let (unmapped, mapped) = map_range.map_range(&range);
        assert!(unmapped.len() == 1);
        assert!(mapped.is_none());
        assert_eq!(unmapped[0], range);
    }

    #[test]
    fn map_range_after() {
        let range = Range::from_length(1000, 10);
        let map_range = MapRange {
            source: 100,
            destination: 10,
            range: 11,
        };

        let (unmapped, mapped) = map_range.map_range(&range);

        assert!(unmapped.len() == 1);
        assert!(mapped.is_none());
        assert_eq!(unmapped[0], range);
    }

    #[test]
    fn map_range_over_start() {
        let range = Range::from_length(1, 10);
        let map_range = MapRange {
            source: 5,
            destination: 10,
            range: 11,
        };

        let (unmapped, mapped) = map_range.map_range(&range);
        assert!(unmapped.len() == 1);
        assert!(mapped.is_some());
        assert_eq!(unmapped[0], Range::from_length(1, 4));
        assert_eq!(mapped.unwrap(), Range::from_length(10, 6));
    }

    #[test]
    fn map_range_over_end() {
        let range = Range::from_length(100, 10);
        let map_range = MapRange {
            source: 95,
            destination: 10,
            range: 10,
        };

        let (unmapped, mapped) = map_range.map_range(&range);
        assert!(unmapped.len() == 1);
        assert!(mapped.is_some());
        assert_eq!(unmapped[0], Range::from_length(105, 5));
        assert_eq!(mapped.unwrap(), Range::from_length(15, 5));
    }

    #[test]
    fn map_range_over_all() {
        let range = Range::from_bounds(1, 20);
        let map_range = MapRange {
            source: 10,
            destination: 30,
            range: 5,
        };

        let (unmapped, mapped) = map_range.map_range(&range);
        assert!(unmapped.len() == 2);
        assert!(mapped.is_some());
        assert_eq!(unmapped[0], Range::from_bounds(1, 10));
        assert_eq!(unmapped[1], Range::from_bounds(15, 20));

        assert_eq!(mapped.unwrap(), Range::from_length(30, 5));
    }

    #[test]
    fn map_range_within() {
        let range = Range::from_length(15, 5);
        let map_range = MapRange {
            source: 1,
            destination: 31,
            range: 100,
        };

        let (unmapped, mapped) = map_range.map_range(&range);
        assert!(unmapped.len() == 0);
        assert!(mapped.is_some());

        assert_eq!(mapped.unwrap(), Range::from_length(45, 5));
    }
}
