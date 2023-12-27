use std::{
    collections::{HashMap, HashSet},
    str::FromStr,
};

advent_of_code::solution!(22);
#[derive(PartialEq, Eq, Clone, Copy)]
struct Brick {
    start: Cube,
    end: Cube,
}
impl FromStr for Brick {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (start, end) = s.split_once('~').unwrap();
        let start = Cube::from_str(start).unwrap();
        let end = Cube::from_str(end).unwrap();
        debug_assert!(start.z <= end.z);
        Ok(Self { start, end })
    }
}
impl Brick {
    fn collide(&self, other: &Brick) -> bool {
        let x_overlap = (self.start.x..=self.end.x).contains(&other.start.x)
            || (self.start.x..=self.end.x).contains(&other.end.x)
            || (other.start.x..=other.end.x).contains(&self.start.x);
        let y_overlap = (self.start.y..=self.end.y).contains(&other.start.y)
            || (self.start.y..=self.end.y).contains(&other.end.y)
            || (other.start.y..=other.end.y).contains(&self.start.y);
        let z_overlap = (self.start.z..=self.end.z).contains(&other.start.z)
            || (self.start.z..=self.end.z).contains(&other.end.z)
            || (other.start.z..=other.end.z).contains(&self.start.z);

        if x_overlap && y_overlap && z_overlap {
            let set1 = self.to_cubes();
            let set2 = other.to_cubes();
            return set1.intersection(&set2).count() != 0;
        }
        false
    }
    fn to_cubes(self) -> HashSet<Cube> {
        let mut set = HashSet::new();
        for x in self.start.x..=self.end.x {
            for y in self.start.y..=self.end.y {
                for z in self.start.z..=self.end.z {
                    set.insert(Cube { x, y, z });
                }
            }
        }
        set
    }
    fn up(&self) -> Self {
        let mut clone = *self;
        clone.start.z += 1;
        clone.end.z += 1;
        clone
    }
    fn down(&self) -> Self {
        let mut clone = *self;
        clone.start.z -= 1;
        clone.end.z -= 1;
        clone
    }
}
#[derive(PartialEq, Eq, Hash, Clone, Copy)]
struct Cube {
    x: u32,
    y: u32,
    z: u32,
}
impl FromStr for Cube {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',').map(|n| n.parse().unwrap());
        let x = it.next().unwrap();
        let y = it.next().unwrap();
        let z = it.next().unwrap();
        debug_assert_eq!(it.next(), None);
        Ok(Self { x, y, z })
    }
}
pub fn part_one(input: &str) -> Option<usize> {
    let mut vec = parse(input);
    fall_new(&mut vec);
    Some(execute1(&vec))
}
fn fall_new(vec: &mut [Brick]) {
    vec.sort_unstable_by_key(|b| b.start.z);
    let mut map = HashMap::new();
    for brick in vec.iter_mut() {
        let z = get_height(&map, brick);
        let diff = brick.end.z - brick.start.z;
        brick.start.z = z;
        brick.end.z = z + diff;
        set_height(&mut map, brick);
    }
}
fn get_height(map: &HashMap<(u32, u32), u32>, brick: &Brick) -> u32 {
    let mut curr_max = None;
    for x in brick.start.x..=brick.end.x {
        for y in brick.start.y..=brick.end.y {
            curr_max = curr_max.max(map.get(&(y, x)));
        }
    }
    curr_max.unwrap_or(&0) + 1
}
fn set_height(map: &mut HashMap<(u32, u32), u32>, brick: &Brick) {
    for x in brick.start.x..=brick.end.x {
        for y in brick.start.y..=brick.end.y {
            map.entry((y, x))
                .and_modify(|e| *e = brick.end.z)
                .or_insert(brick.end.z);
        }
    }
}
fn build_supports_map(vec: &[Brick]) -> HashMap<usize, HashSet<usize>> {
    vec.iter()
        .enumerate()
        .map(|(i, b)| (i, collisions_with(vec, i, &b.up())))
        .collect()
}
fn build_is_supported_by_map(vec: &[Brick]) -> HashMap<usize, HashSet<usize>> {
    vec.iter()
        .enumerate()
        .map(|(i, b)| (i, collisions_with(vec, i, &b.down())))
        .collect()
}
fn collisions_with(vec: &[Brick], j: usize, x: &Brick) -> HashSet<usize> {
    vec.iter()
        .enumerate()
        .filter(|(i, _)| i != &j)
        .filter(|(_, b)| x.collide(b))
        .map(|(i, _)| i)
        .collect()
}

fn execute1(vec: &[Brick]) -> usize {
    let is_supported_by = build_is_supported_by_map(vec);
    let support = build_supports_map(vec);
    support
        .values()
        .filter(|s| s.iter().all(|i| is_supported_by.get(i).unwrap().len() > 1))
        .count()
}
fn execute2(vec: &Vec<Brick>) -> usize {
    let is_supported_by = build_is_supported_by_map(vec);
    let supports = build_supports_map(vec);

    (0..vec.len())
        .map(|i| how_many_fall(&supports, &is_supported_by, i, &mut HashSet::new()).len())
        .sum()
}
fn how_many_fall(
    supports: &HashMap<usize, HashSet<usize>>,
    is_supported_by: &HashMap<usize, HashSet<usize>>,
    index: usize,
    curr_disintegrated: &mut HashSet<usize>,
) -> HashSet<usize> {
    let sup = supports.get(&index).unwrap();
    curr_disintegrated.insert(index);
    for i in sup {
        let n = is_supported_by.get(i).unwrap();
        let diff = n.difference(curr_disintegrated);
        if diff.count() == 0 {
            let ret = how_many_fall(supports, is_supported_by, *i, curr_disintegrated);
            curr_disintegrated.extend(ret.into_iter());
            curr_disintegrated.insert(*i);
        }
    }
    curr_disintegrated.remove(&index);
    curr_disintegrated.clone()
}

pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = parse(input);
    fall_new(&mut vec);
    Some(execute2(&vec))
}
fn parse(input: &str) -> Vec<Brick> {
    input
        .trim()
        .lines()
        .map(|l| Brick::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = part_one(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(5));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(530));
    }
    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, Some(7));
    }
    #[test]
    fn test_part_two_actual() {
        let result = part_two(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(93_292));
    }
}
