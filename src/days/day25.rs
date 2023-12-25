use std::collections::{HashMap, HashSet};

use super::day::{Day, DayResult};

pub struct Instance;

struct Wires<'a> {
    connected: HashMap<&'a str, Vec<&'a str>>,
}

impl<'a> Wires<'a> {
    fn new(lines: &'a [String]) -> Self {
        let mut connected = HashMap::new();

        for line in lines {
            let (from, tos) = line.split_once(": ").unwrap();
            for to in tos.split(' ') {
                connected
                    .entry(from)
                    .and_modify(|c: &mut Vec<_>| c.push(to))
                    .or_insert(vec![to]);
                connected
                    .entry(to)
                    .and_modify(|c: &mut Vec<_>| c.push(from))
                    .or_insert(vec![from]);
            }
        }
        Wires { connected }
    }
}

impl Wires<'_> {
    fn traverse(&self) -> usize {
        let all_wires: HashSet<_> = self.connected.keys().copied().collect();
        let guess = all_wires.len() * 5 / 12;
        let connection_count =
            |partition_a: &HashSet<&str>, partition_b: &HashSet<&str>| -> usize {
                let mut connection_count = 0;
                for a in partition_a {
                    let conns = &self.connected[a];
                    for c in conns {
                        if partition_b.contains(c) {
                            connection_count += 1;
                        }
                    }
                }

                connection_count
            };

        for start in self.connected.keys().copied() {
            let mut visiting = vec![(start, 0)];
            let mut visited = HashMap::new();
            visited.insert(start, 0);

            while let Some((v, d)) = visiting.pop() {
                for k in &self.connected[&v] {
                    if !visited.contains_key(k) {
                        visited.insert(*k, d + 1);
                        visiting.push((*k, d + 1));
                    }
                }

                visited.entry(v).and_modify(|v| *v += 1).or_insert(1);
            }

            let max_distance = visited.values().max().unwrap();

            let a = max_distance / 4;
            let b = max_distance * 3 / 4;

            for i in a..b {
                let partition_a: HashSet<_> = visited
                    .iter()
                    .filter(|(_, v)| **v <= i)
                    .map(|(s, _)| *s)
                    .collect();

                if partition_a.len() < guess {
                    continue;
                }

                let partition_b: HashSet<_> = all_wires.difference(&partition_a).copied().collect();

                if partition_b.len() < guess {
                    break;
                }

                let c = connection_count(&partition_a, &partition_b);

                if c == 3 {
                    return partition_a.len() * partition_b.len();
                }
            }
        }
        panic!("couldn't find split")
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let wires = Wires::new(&lines);

        let part1 = wires.traverse().to_string();

        Ok(DayResult { part1, part2: None })
    }
}
