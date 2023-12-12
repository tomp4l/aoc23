use std::{collections::HashMap, str::FromStr};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, Clone, Copy, Hash, PartialEq, Eq)]
enum State {
    Unknown,
    Damaged,
    Operational,
}

#[derive(Debug)]
struct Record {
    condition: Vec<State>,
    contiguity: Vec<usize>,
}

impl FromStr for Record {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (a, b) = s.split_once(' ').ok_or("bad format")?;

        let condition = a
            .chars()
            .map(|c| match c {
                '.' => Ok(State::Operational),
                '#' => Ok(State::Damaged),
                '?' => Ok(State::Unknown),
                _ => Err(""),
            })
            .try_collect()?;
        let contiguity = b
            .split(',')
            .map(|s| s.parse::<usize>().map_err(|e| format!("{}: {}", e, b)))
            .try_collect()?;

        Ok(Record {
            condition,
            contiguity,
        })
    }
}

impl Record {
    fn combinations(&self) -> usize {
        let mut cache: HashMap<(Vec<State>, Vec<usize>), usize> = HashMap::new();
        fn cached_result(
            condition: &[State],
            contiguity: &[usize],
            cache: &mut HashMap<(Vec<State>, Vec<usize>), usize>,
        ) -> usize {
            let key = (condition.to_vec(), contiguity.to_vec());
            if let Some(v) = cache.get(&key) {
                return *v;
            }

            let ret = if contiguity.is_empty() {
                if condition.iter().any(|c| matches!(c, State::Damaged)) {
                    0
                } else {
                    1
                }
            } else {
                let broken: usize = contiguity.iter().sum();
                let gaps = contiguity.len() - 1;
                let state = condition.len();
                let spaces = state - gaps - broken;
                let current = contiguity[0];
                let mut total = 0;

                for i in 0..=spaces {
                    let mut candidate = Vec::new();
                    candidate.extend(vec![State::Operational; i]);
                    candidate.extend(vec![State::Damaged; current]);

                    match candidate.len().cmp(&condition.len()) {
                        std::cmp::Ordering::Less => candidate.push(State::Operational),
                        std::cmp::Ordering::Equal => (),
                        std::cmp::Ordering::Greater => break,
                    }

                    let is_invalid = candidate.iter().zip(condition).any(|s| {
                        matches!(
                            s,
                            (State::Damaged, State::Operational)
                                | (State::Operational, State::Damaged)
                        )
                    });

                    if is_invalid {
                        continue;
                    }

                    total += cached_result(&condition[candidate.len()..], &contiguity[1..], cache);
                }

                total
            };

            cache.insert(key, ret);
            ret
        }

        cached_result(&self.condition, &self.contiguity, &mut cache)
    }

    fn times_five_combinations(&self) -> usize {
        self.times(5).combinations()
    }

    fn times(&self, n: usize) -> Self {
        let mut condition_n = self.condition.clone();
        condition_n.push(State::Unknown);
        let mut condition = condition_n.repeat(n);
        condition.pop();
        let contiguity = self.contiguity.repeat(n);

        Record {
            condition,
            contiguity,
        }
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let records: Vec<_> = lines.iter().map(|l| l.parse::<Record>()).try_collect()?;

        let part1 = records
            .iter()
            .map(|r| r.combinations())
            .sum::<usize>()
            .to_string();

        let part2 = records
            .iter()
            .map(|r| r.times_five_combinations())
            .sum::<usize>()
            .to_string();

        Ok(DayResult {
            part1,
            part2: Some(part2),
        })
    }
}
