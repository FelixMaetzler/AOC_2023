use std::str::FromStr;

advent_of_code::solution!(2);
#[derive(Debug)]
struct Subset {
    red: u32,
    green: u32,
    blue: u32,
}
impl FromStr for Subset {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut red = 0;
        let mut green = 0;
        let mut blue = 0;
        for subset in s.split(", ") {
            let (n, color) = subset.split_once(' ').unwrap();
            let n = n.parse().unwrap();
            match color {
                "red" => red = n,
                "green" => green = n,
                "blue" => blue = n,
                _ => panic!(),
            }
        }
        Ok(Self { red, green, blue })
    }
}
impl Subset {
    fn is_possible_in_reference_to(&self, other: &Self) -> bool {
        self.green <= other.green && self.red <= other.red && self.blue <= other.blue
    }
}
#[derive(Debug)]
struct Game {
    id: u32,
    cubes: Vec<Subset>,
}
impl Game {
    fn is_possible_in_reference_to(&self, other: &Subset) -> bool {
        self.cubes
            .iter()
            .all(|s| s.is_possible_in_reference_to(other))
    }
    fn power(&self) -> u32 {
        let red = self.cubes.iter().map(|s| s.red).max().unwrap();
        let green = self.cubes.iter().map(|s| s.green).max().unwrap();
        let blue = self.cubes.iter().map(|s| s.blue).max().unwrap();
        red * green * blue
    }
}
impl FromStr for Game {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (game, remainder) = s.split_once(": ").unwrap();
        let (_, id) = game.split_once(' ').unwrap();
        let id = id.parse().unwrap();
        Ok(Self {
            id,
            cubes: remainder
                .split("; ")
                .map(|s| Subset::from_str(s).unwrap())
                .collect(),
        })
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let vec = parse(input);
    let vgl = &Subset {
        red: 12,
        green: 13,
        blue: 14,
    };
    Some(
        vec.into_iter()
            .filter(|g| g.is_possible_in_reference_to(vgl))
            .map(|g| g.id)
            .sum(),
    )
}
fn parse(input: &str) -> Vec<Game> {
    input
        .trim()
        .lines()
        .map(|l| Game::from_str(l).unwrap())
        .collect()
}
pub fn part_two(input: &str) -> Option<u32> {
    let vec = parse(input);
    Some(vec.into_iter().map(|g| g.power()).sum())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(8));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(2105));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(2286));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(72422));
    }
}
