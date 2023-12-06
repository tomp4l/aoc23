use lazy_static::lazy_static;
use regex::Regex;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug)]
struct Race {
    time: u64,
    distance: u64,
}

impl Race {
    fn wins(&self) -> usize {
        (1..self.time)
            .rev()
            .zip(1..self.time)
            .filter(|(remaining, speed)| remaining * speed > self.distance)
            .count()
    }
}

lazy_static! {
    static ref NUMBERS_REGEX: Regex = Regex::new(r"\d+").unwrap();
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let times = NUMBERS_REGEX
            .captures_iter(&lines[0])
            .map(|i| i.extract::<0>().0.parse::<u64>().unwrap());
        let distances = NUMBERS_REGEX
            .captures_iter(&lines[1])
            .map(|i| i.extract::<0>().0.parse::<u64>().unwrap());

        let races: Vec<_> = times
            .zip(distances)
            .map(|(time, distance)| Race { time, distance })
            .collect();

        let mut part1 = 1;
        for race in races {
            part1 *= race.wins();
        }

        let real_time = NUMBERS_REGEX
            .captures_iter(&lines[0].replace(' ', ""))
            .map(|i| i.extract::<0>().0.parse::<u64>().unwrap())
            .next()
            .unwrap();
        let real_distance = NUMBERS_REGEX
            .captures_iter(&lines[1].replace(' ', ""))
            .map(|i| i.extract::<0>().0.parse::<u64>().unwrap())
            .next()
            .unwrap();

        let real_race = Race {
            time: real_time,
            distance: real_distance,
        };

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(real_race.wins().to_string()),
        })
    }
}
