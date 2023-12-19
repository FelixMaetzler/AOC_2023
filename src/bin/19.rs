use std::{collections::HashMap, str::FromStr};

advent_of_code::solution!(19);
#[derive(Eq, PartialEq, Clone)]
enum Output {
    Reject,
    Accept,
    Workflow(String),
}
impl FromStr for Output {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "A" => Ok(Self::Accept),
            "R" => Ok(Self::Reject),
            val => Ok(Self::Workflow(val.to_string())),
        }
    }
}
enum Rule {
    Less(char, u32, Output),
    Greater(char, u32, Output),
    Default(Output),
}
impl Rule {
    fn execute(&self, part: &Part) -> Option<Output> {
        match self {
            Rule::Less(c, n, o) => {
                if part.data.get(c).unwrap() < n {
                    Some(o.clone())
                } else {
                    None
                }
            }
            Rule::Greater(c, n, o) => {
                if part.data.get(c).unwrap() > n {
                    Some(o.clone())
                } else {
                    None
                }
            }
            Rule::Default(o) => Some(o.clone()),
        }
    }
}
impl FromStr for Rule {
    type Err = String;

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        if s.chars().all(|c| c.is_alphabetic()) {
            return Ok(Self::Default(Output::from_str(s).unwrap()));
        }
        if s.contains('<') {
            let (c, rem) = s.split_once('<').unwrap();
            let (n, output) = rem.split_once(':').unwrap();
            assert_eq!(c.len(), 1);
            let c = c.chars().next().unwrap();
            let n = n.parse().unwrap();
            let output = Output::from_str(output).unwrap();
            return Ok(Self::Less(c, n, output));
        }
        if s.contains('>') {
            let (c, rem) = s.split_once('>').unwrap();
            let (n, output) = rem.split_once(':').unwrap();
            assert_eq!(c.len(), 1);
            let c = c.chars().next().unwrap();
            let n = n.parse().unwrap();
            let output = Output::from_str(output).unwrap();
            return Ok(Self::Greater(c, n, output));
        }
        Err(s.to_string())
    }
}
struct Workflow {
    name: String,
    rules: Vec<Rule>,
}
impl FromStr for Workflow {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (name, rem) = s.split_once('{').unwrap();
        let name = name.to_string();
        let rem = rem.trim_end_matches('}');
        let rules = rem.split(',').map(|r| Rule::from_str(r).unwrap()).collect();
        Ok(Self { name, rules })
    }
}
struct Part {
    data: HashMap<char, u32>,
}
impl FromStr for Part {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let rem = s.trim_end_matches('}');
        let rem = rem.trim_start_matches('{');
        Ok(Self {
            data: rem
                .split(',')
                .map(|l| l.split_once('=').unwrap())
                .map(|(l, r)| (l.parse().unwrap(), r.parse().unwrap()))
                .collect(),
        })
    }
}
impl Part {
    fn rating_number(&self) -> u32 {
        self.data.values().sum()
    }
    fn is_accepted(&self, map: &HashMap<String, Workflow>) -> bool {
        match self.recurse(map, "in".to_string()) {
            Output::Reject => false,
            Output::Accept => true,
            Output::Workflow(_) => unreachable!(),
        }
    }
    fn recurse(&self, map: &HashMap<String, Workflow>, key: String) -> Output {
        let w = map.get(&key).unwrap();
        for rule in &w.rules {
            if let Some(next) = rule.execute(self) {
                match next {
                    Output::Reject => return Output::Reject,
                    Output::Accept => return Output::Accept,
                    Output::Workflow(x) => return self.recurse(map, x),
                }
            }
        }
        unreachable!()
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let (map, vec) = parse(input);
    Some(
        vec.iter()
            .filter(|p| p.is_accepted(&map))
            .map(|p| p.rating_number())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part>) {
    let (left, right) = input.trim().split_once("\n\n").unwrap();
    let map = left
        .lines()
        .map(|l| Workflow::from_str(l).unwrap())
        .map(|w| (w.name.clone(), w))
        .collect();
    let vec = right.lines().map(|l| Part::from_str(l).unwrap()).collect();
    (map, vec)
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(19_114));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(397_643));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
