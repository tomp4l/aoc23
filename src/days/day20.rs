use std::{
    collections::{HashMap, VecDeque},
    iter,
    str::FromStr,
};

use itertools::Itertools;

use super::{
    day::{Day, DayResult},
    util::lcm,
};

pub struct Instance;

#[derive(Debug)]
enum ModuleType {
    FlipFlop,
    Conjunction,
    Broadcast,
}

#[derive(Debug)]
struct Module {
    module_type: ModuleType,
    source: String,
    destinations: Vec<String>,
}

impl FromStr for Module {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (t, s) = if s.starts_with("broadcaster") {
            (ModuleType::Broadcast, s)
        } else if let Some(s) = s.strip_prefix('&') {
            (ModuleType::Conjunction, s)
        } else {
            (ModuleType::FlipFlop, &s[1..])
        };

        let (source, destination) = s.split_once(" -> ").ok_or("")?;

        let destinations = destination.split(", ").map(|s| s.to_owned()).collect_vec();

        Ok(Module {
            module_type: t,
            source: source.to_owned(),
            destinations,
        })
    }
}

#[derive(Clone, Debug, PartialEq, Eq)]
enum Pulse {
    High,
    Low,
}

#[derive(Debug)]
struct Circuit<'a> {
    broadcaster: &'a Vec<String>,
    flip_flops: HashMap<&'a str, (Pulse, &'a Vec<String>)>,
    conjunctions: HashMap<&'a str, (HashMap<&'a str, Pulse>, &'a Vec<String>)>,
    high_signals: usize,
    low_signals: usize,
    button_pushes: usize,
}

impl<'a> Circuit<'a> {
    fn from_modules(modules: &'a [Module]) -> Self {
        let broadcaster = modules
            .iter()
            .find(|m| matches!(m.module_type, ModuleType::Broadcast))
            .unwrap();
        let flip_flops = modules
            .iter()
            .filter(|m| matches!(m.module_type, ModuleType::FlipFlop))
            .map(|m| (m.source.as_str(), (Pulse::Low, &m.destinations)))
            .collect();
        let conjunctions = modules
            .iter()
            .filter(|m| matches!(m.module_type, ModuleType::Conjunction))
            .map(|m| {
                let inputs = modules
                    .iter()
                    .filter(|o| o.destinations.contains(&m.source))
                    .map(|m| (m.source.as_str(), Pulse::Low))
                    .collect();

                (m.source.as_str(), (inputs, &m.destinations))
            })
            .collect();

        Circuit {
            broadcaster: &broadcaster.destinations,
            flip_flops,
            conjunctions,
            high_signals: 0,
            low_signals: 0,
            button_pushes: 0,
        }
    }
}

impl Circuit<'_> {
    fn push_the_button(&mut self) -> Vec<(&str, &str, Pulse)> {
        let mut active_pulses: VecDeque<_> = self
            .broadcaster
            .iter()
            .zip(iter::repeat(("broadcast", Pulse::Low)))
            .collect();

        self.low_signals += 1;
        self.button_pushes += 1;

        let mut ret = Vec::new();

        while let Some((target, (source, pulse))) = active_pulses.pop_front() {
            ret.push((target.as_str(), source, pulse.clone()));
            match pulse {
                Pulse::High => self.high_signals += 1,
                Pulse::Low => self.low_signals += 1,
            }
            if matches!(pulse, Pulse::Low) {
                if let Some((pulse, outputs)) = self.flip_flops.get_mut(target.as_str()) {
                    *pulse = match pulse {
                        Pulse::High => Pulse::Low,
                        Pulse::Low => Pulse::High,
                    };
                    active_pulses.extend(
                        outputs
                            .iter()
                            .map(|s| (s, (target.as_str(), pulse.clone()))),
                    )
                }
            }

            if let Some((inputs, outputs)) = self.conjunctions.get_mut(target.as_str()) {
                inputs.insert(source, pulse);
                let p = if inputs.values().all(|v| matches!(v, Pulse::High)) {
                    Pulse::Low
                } else {
                    Pulse::High
                };
                active_pulses.extend(outputs.iter().map(|o| (o, (target.as_str(), p.clone()))));
            }
        }

        ret
    }

    fn analyze(&mut self) -> usize {
        let target = self
            .conjunctions
            .iter()
            .find(|m| m.1 .1.contains(&"rx".to_string()))
            .unwrap();

        let name = target.0.to_owned();

        let mut cycles = HashMap::new();

        loop {
            let x = self.push_the_button();

            let pulses = x
                .iter()
                .filter(|s| s.0 == name && s.2 == Pulse::High)
                .map(|s| s.1.to_owned())
                .collect_vec();

            for p in pulses {
                cycles
                    .entry(p)
                    .and_modify(|p: &mut Vec<usize>| p.push(self.button_pushes))
                    .or_insert(vec![self.button_pushes]);
            }

            if !cycles.is_empty() && cycles.values().all(|v| v.len() > 1) {
                break;
            }
        }

        let mut r = 1;

        for v in cycles.values() {
            let d = v.iter().tuple_windows().map(|(a, b)| b - a).collect_vec();

            r = lcm(r, d[0]);
        }

        r
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let modules: Vec<_> = lines.iter().map(|l| l.parse::<Module>()).try_collect()?;

        let mut circuit = Circuit::from_modules(&modules);

        for _ in 0..1000 {
            circuit.push_the_button();
        }

        let part1 = circuit.low_signals * circuit.high_signals;
        let part2 = circuit.analyze();

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(part2.to_string()),
        })
    }
}
