use std::{collections::HashMap, ops::Range, str::FromStr};

use advent_of_code::{Count, RangeExt};

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
    Less(char, u64, Output),
    Greater(char, u64, Output),
    Default(Output),
}
impl Rule {
    fn execute1(&self, part: &Part1) -> Option<Output> {
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
    fn execute2(&self, part: &Part2) -> (Option<(Part2, Output)>, Option<Part2>) {
        let (range, cmp, c, o) = match self {
            Rule::Less(c, n, o) => {
                let range = 1..*n;
                let cmp = part.data.get(c).unwrap();
                (range, cmp, c, o)
            }
            Rule::Greater(c, n, o) => {
                let range = n + 1..4001;
                let cmp = part.data.get(c).unwrap();
                (range, cmp, c, o)
            }
            Rule::Default(o) => return (Some((part.clone(), o.clone())), None),
        };
        let intersection = cmp.get_intersection(&range); // Has to go to next key
        let non_intersection = cmp.get_non_intersection(&range); // Has to go to next rule
        let ret1 = intersection.map(|r| (part.new_with(*c, r), o.clone()));
        let ret2 = match non_intersection {
            Count::None => None,
            Count::Single(r) => Some(part.new_with(*c, r)),
            Count::Double(_, _) => unreachable!(),
        };
        (ret1, ret2)
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
            debug_assert_eq!(c.len(), 1);
            let c = c.chars().next().unwrap();
            let n = n.parse().unwrap();
            let output = Output::from_str(output).unwrap();
            return Ok(Self::Less(c, n, output));
        }
        if s.contains('>') {
            let (c, rem) = s.split_once('>').unwrap();
            let (n, output) = rem.split_once(':').unwrap();
            debug_assert_eq!(c.len(), 1);
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
struct Part1 {
    data: HashMap<char, u64>,
}
#[derive(Clone)]
struct Part2 {
    data: HashMap<char, Range<u64>>,
}

impl Part2 {
    fn new() -> Self {
        let mut data = HashMap::new();
        data.insert('x', 1..4001);
        data.insert('m', 1..4001);
        data.insert('a', 1..4001);
        data.insert('s', 1..4001);
        Self { data }
    }
    fn new_with(&self, c: char, r: Range<u64>) -> Self {
        let mut n = self.clone();
        n.data.insert(c, r);
        n
    }
    fn comb(&self) -> usize {
        self.data.values().map(|r| r.clone().count()).product()
    }
    fn recurse(&self, map: &HashMap<String, Workflow>, key: String) -> usize {
        let workflow = map.get(&key).unwrap();
        let mut curr = vec![self.clone()];
        let mut sum = 0;
        for rule in &workflow.rules {
            let mut next = vec![];
            for part in &curr {
                let (n, push) = rule.execute2(part);
                if let Some((p, o)) = n {
                    match o {
                        Output::Reject => sum += 0,
                        Output::Accept => sum += p.comb(),
                        Output::Workflow(o) => sum += p.recurse(map, o),
                    }
                }
                if let Some(x) = push {
                    next.push(x);
                }
            }
            curr = next;
        }
        sum
    }
}
impl FromStr for Part1 {
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
impl Part1 {
    fn rating_number(&self) -> u64 {
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
            if let Some(next) = rule.execute1(self) {
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
pub fn part_one(input: &str) -> Option<u64> {
    let (map, vec) = parse(input);
    Some(
        vec.iter()
            .filter(|p| p.is_accepted(&map))
            .map(|p| p.rating_number())
            .sum(),
    )
}

pub fn part_two(input: &str) -> Option<usize> {
    let (map, _) = parse(input);
    let part = Part2::new();
    Some(part.recurse(&map, "in".to_string()))
}
fn parse(input: &str) -> (HashMap<String, Workflow>, Vec<Part1>) {
    let (left, right) = input.trim().split_once("\n\n").unwrap();
    let map = left
        .lines()
        .map(|l| Workflow::from_str(l).unwrap())
        .map(|w| (w.name.clone(), w))
        .collect();
    let vec = right.lines().map(|l| Part1::from_str(l).unwrap()).collect();
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
        assert_eq!(result, Some(167_409_079_868_000));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(132_392_981_697_081));
    }
}
