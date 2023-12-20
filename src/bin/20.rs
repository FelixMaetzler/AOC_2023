use std::{
    collections::{HashMap, VecDeque},
    str::FromStr,
};

advent_of_code::solution!(20);
#[derive(Clone)]
enum Operation {
    FlipFlop,
    Conjunction,
    Broadcast,
    Button,
}
#[derive(Clone)]
struct Module {
    inputs: HashMap<String, bool>,
    name: String,
    op: Operation,
    outputs: Vec<String>,
    curr_output: bool,
}
impl Module {
    fn update(&mut self, comes_from: &str, val: bool) -> Option<bool> {
        match self.op {
            Operation::FlipFlop => {
                if !val {
                    self.curr_output = !self.curr_output;
                    Some(self.curr_output)
                } else {
                    None
                }
            }
            Operation::Conjunction => {
                self.inputs.insert(comes_from.to_string(), val);
                self.curr_output = !self.inputs.values().all(|b| *b);
                Some(self.curr_output)
            }
            Operation::Broadcast => {
                self.curr_output = val;
                Some(val)
            }
            Operation::Button => unreachable!(),
        }
    }
}
impl FromStr for Module {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (left, right) = s.split_once(" -> ").unwrap();
        let outputs = right.split(", ").map(|s| s.parse().unwrap()).collect();
        let op = match left.chars().nth(0).unwrap() {
            '%' => Operation::FlipFlop,
            '&' => Operation::Conjunction,
            _ => Operation::Broadcast,
        };
        let left = left.trim_start_matches('%');
        let left = left.trim_start_matches('&');
        let name = left.to_string();
        Ok(Self {
            inputs: HashMap::new(),
            name,
            op,
            outputs,
            curr_output: false,
        })
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut map = parse(input);
    for val in map.clone().values() {
        for s in &val.outputs {
            map.entry(s.clone()).and_modify(|m| {
                m.inputs.insert(val.name.clone(), false);
            });
        }
    }
    map.insert(
        "button".to_string(),
        Module {
            inputs: HashMap::new(),
            name: "button".to_string(),
            op: Operation::Button,
            outputs: vec!["broadcaster".to_string()],
            curr_output: false,
        },
    );
    let erg = (0..1000)
        .map(|_| execute(&mut map))
        .reduce(|acc, e| (acc.0 + e.0, acc.1 + e.1))
        .unwrap();
    Some(erg.0 * erg.1)
}
fn execute(map: &mut HashMap<String, Module>) -> (u32, u32) {
    let mut sum_high = 0;
    let mut sum_low = 0;
    let mut queue = VecDeque::new();
    queue.push_back("button".to_string());
    while let Some(curr) = queue.pop_front() {
        let module = map.get(&curr).unwrap();
        let output = module.curr_output;
        let outputs = module.outputs.clone();
        for n in outputs {
            if let Some(m) = map.get_mut(&n.to_string()) {
                if m.update(&curr, output).is_some() {
                    queue.push_back(n);
                }
            }
            if output {
                sum_high += 1
            } else {
                sum_low += 1
            }
        }
    }
    (sum_low, sum_high)
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> HashMap<String, Module> {
    input
        .trim()
        .lines()
        .map(|l| Module::from_str(l).unwrap())
        .map(|m| (m.name.clone(), m))
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (first, second) = input.split_once("\n\n").unwrap();
        assert_eq!(part_one(first), Some(32_000_000));
        assert_eq!(part_one(second), Some(11_687_500));
    }

    #[test]
    fn test_part_two() {
        let input = advent_of_code::template::read_file("examples", DAY);
        let (first, second) = input.split_once("\n\n").unwrap();
        assert_eq!(part_two(first), None);
        assert_eq!(part_two(second), None);
    }
}
