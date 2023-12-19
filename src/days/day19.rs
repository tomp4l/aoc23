use std::{
    char,
    collections::HashMap,
    str::{FromStr, Split},
};

use itertools::Itertools;

use super::day::{Day, DayResult};

pub struct Instance;

#[derive(Debug, Clone)]
struct Part {
    x: u16,
    m: u16,
    a: u16,
    s: u16,
}

impl FromStr for Part {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("too short".to_owned());
        }
        let trimmed = &s[1..s.len() - 1];
        let mut split = trimmed.split(',');

        fn parse_assert(split: &mut Split<'_, char>, char: &str) -> Result<u16, String> {
            let s = split.next().ok_or(format!("missing {}", char))?;
            let (c, v) = s
                .split_once('=')
                .ok_or(format!("bad format for {}: {}", s, char))?;
            if c != char {
                return Err(format!("mismatch char: {} {}", c, char));
            }
            v.parse()
                .map_err(|e| format!("Parse error for {}: {}", char, e))
        }

        let x = parse_assert(&mut split, "x")?;
        let m = parse_assert(&mut split, "m")?;
        let a = parse_assert(&mut split, "a")?;
        let s = parse_assert(&mut split, "s")?;

        Ok(Part { x, m, a, s })
    }
}

impl Part {
    fn get(&self, p: &Property) -> u16 {
        match p {
            Property::X => self.x,
            Property::M => self.m,
            Property::A => self.a,
            Property::S => self.s,
        }
    }

    fn set(&mut self, p: &Property, v: u16) {
        match p {
            Property::X => self.x = v,
            Property::M => self.m = v,
            Property::A => self.a = v,
            Property::S => self.s = v,
        }
    }

    fn rating(&self) -> u32 {
        (self.x + self.m + self.a + self.s) as u32
    }
}

#[derive(Debug)]
enum Outcome {
    Accept,
    Reject,
    Workflow(String),
}

impl FromStr for Outcome {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "A" => Outcome::Accept,
            "R" => Outcome::Reject,
            s => Outcome::Workflow(s.to_owned()),
        })
    }
}

#[derive(Debug)]
enum Property {
    X,
    M,
    A,
    S,
}

impl FromStr for Property {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        Ok(match s {
            "x" => Self::X,
            "m" => Self::M,
            "a" => Self::A,
            "s" => Self::S,
            u => Err(format!("bad property: {}", u))?,
        })
    }
}

#[derive(Debug)]
enum Rule {
    Gt(Property, u16, Outcome),
    Lt(Property, u16, Outcome),
}

impl Rule {
    fn apply(&self, part: &Part) -> Option<&Outcome> {
        match self {
            Rule::Gt(p, v, o) => (part.get(p) > *v).then_some(o),
            Rule::Lt(p, v, o) => (part.get(p) < *v).then_some(o),
        }
    }
}

impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let c = if s.contains('>') {
            '>'
        } else if s.contains('<') {
            '<'
        } else {
            Err(format!("Malformed rule: {}", s))?
        };

        let (p, vo) = s.split_once(c).unwrap();
        let (v, o) = vo.split_once(':').ok_or(format!("Malformed rule: {}", s))?;

        let property = p.parse::<Property>()?;
        let value = v
            .parse::<u16>()
            .map_err(|e| format!("Parse error for rule {}: {}", s, e))?;
        let outcome = o.parse::<Outcome>()?;

        Ok(if c == '>' {
            Self::Gt(property, value, outcome)
        } else {
            Self::Lt(property, value, outcome)
        })
    }
}

#[derive(Debug)]
struct Workflow {
    name: String,
    rules: Vec<Rule>,
    default: Outcome,
}

impl FromStr for Workflow {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.len() < 2 {
            return Err("too short".to_owned());
        }
        let (name, rules_str) = s[..s.len() - 1]
            .split_once('{')
            .ok_or(format!("bad format: {}", s))?;

        let rules_split = rules_str.split(',').collect_vec();

        let rules = rules_split[..rules_split.len() - 1]
            .iter()
            .map(|p| p.parse::<Rule>())
            .try_collect()?;

        let default = rules_split.last().unwrap().parse::<Outcome>()?;

        Ok(Workflow {
            name: name.to_owned(),
            rules,
            default,
        })
    }
}

impl Workflow {
    fn apply(&self, part: &Part) -> &Outcome {
        for rule in &self.rules {
            if let Some(o) = rule.apply(part) {
                return o;
            }
        }
        &self.default
    }
}

struct Workflows<'a>(HashMap<&'a str, &'a Workflow>);

impl<'a> Workflows<'a> {
    fn from_slice(s: &'a [Workflow]) -> Self {
        let mut map = HashMap::new();
        for w in s {
            map.insert(w.name.as_str(), w);
        }

        Workflows(map)
    }
}

impl Workflows<'_> {
    fn accepts(&self, part: &Part) -> bool {
        let mut workflow = self.0["in"];

        loop {
            match workflow.apply(part) {
                Outcome::Accept => return true,
                Outcome::Reject => return false,
                Outcome::Workflow(n) => workflow = self.0[n.as_str()],
            }
        }
    }

    fn total_accepted(&self) -> u64 {
        let mut canditates = vec![(
            Part {
                x: 1,
                m: 1,
                s: 1,
                a: 1,
            },
            Part {
                x: 4000,
                m: 4000,
                s: 4000,
                a: 4000,
            },
            self.0["in"],
        )];

        let mut accepted = 0;

        fn possibility(from: &Part, to: &Part) -> u64 {
            (to.x - from.x + 1) as u64
                * (to.m - from.m + 1) as u64
                * (to.s - from.s + 1) as u64
                * (to.a - from.a + 1) as u64
        }

        'outer: while let Some((from, to, workflow)) = canditates.pop() {
            let mut from = from;
            let mut to = to;
            for rule in &workflow.rules {
                match rule {
                    Rule::Gt(p, v, o) => {
                        if from.get(p) > *v {
                            match o {
                                Outcome::Accept => {
                                    accepted += possibility(&from, &to);
                                }
                                Outcome::Reject => (),
                                Outcome::Workflow(w) => {
                                    canditates.push((from, to, self.0[w.as_str()]))
                                }
                            }
                            continue 'outer;
                        } else if to.get(p) <= *v {
                            continue;
                        }
                        let mut higher_from = from.clone();
                        higher_from.set(p, v + 1);
                        let mut lower_to = to.clone();
                        lower_to.set(p, *v);

                        match o {
                            Outcome::Accept => {
                                accepted += possibility(&higher_from, &to);
                            }
                            Outcome::Reject => (),
                            Outcome::Workflow(w) => {
                                canditates.push((higher_from, to.clone(), self.0[w.as_str()]))
                            }
                        }
                        to = lower_to
                    }
                    Rule::Lt(p, v, o) => {
                        if to.get(p) < *v {
                            match o {
                                Outcome::Accept => {
                                    accepted += possibility(&from, &to);
                                }
                                Outcome::Reject => (),
                                Outcome::Workflow(w) => {
                                    canditates.push((from, to, self.0[w.as_str()]))
                                }
                            }
                            continue 'outer;
                        } else if from.get(p) >= *v {
                            continue;
                        }
                        let mut lower_to = to.clone();
                        lower_to.set(p, v - 1);
                        let mut higher_from = from.clone();
                        higher_from.set(p, *v);
                        match o {
                            Outcome::Accept => {
                                accepted += possibility(&from, &lower_to);
                            }
                            Outcome::Reject => (),
                            Outcome::Workflow(w) => {
                                canditates.push((from.clone(), lower_to, self.0[w.as_str()]))
                            }
                        }
                        from = higher_from;
                    }
                }
            }

            match &workflow.default {
                Outcome::Accept => {
                    accepted += possibility(&from, &to);
                }
                Outcome::Reject => (),
                Outcome::Workflow(w) => canditates.push((from, to, self.0[w.as_str()])),
            }
        }

        accepted
    }
}

impl Day for Instance {
    fn run(&self, lines: Vec<String>) -> Result<DayResult, String> {
        let workflows = lines
            .iter()
            .filter_map(|l| l.parse::<Workflow>().ok())
            .collect_vec();

        let parts = lines
            .iter()
            .filter_map(|l| l.parse::<Part>().ok())
            .collect_vec();

        let workflows = Workflows::from_slice(&workflows);

        let mut part1 = 0;
        for part in parts {
            if workflows.accepts(&part) {
                part1 += part.rating();
            }
        }

        Ok(DayResult {
            part1: part1.to_string(),
            part2: Some(workflows.total_accepted().to_string()),
        })
    }
}
