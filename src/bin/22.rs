use std::{collections::HashSet, str::FromStr};

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
    fn lower(&mut self) {
        self.start.lower();
        self.end.lower();
    }
    fn higher(&mut self) {
        self.start.higher();
        self.end.higher();
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
impl Cube {
    fn lower(&mut self) {
        self.z -= 1;
    }
    fn higher(&mut self) {
        self.z += 1;
    }
}
pub fn part_one(input: &str) -> Option<u32> {
    let mut vec = parse(input);
    fall(&mut vec);
    Some(execute1(&vec))
}
fn fall(vec: &mut [Brick]) {
    let mut cont = true;
    while cont {
        let mut c = false;
        for i in 0..vec.len() {
            let prev = vec[i];
            while !collision_with(vec, i) {
                if vec[i].start.z > 1 {
                    vec[i].lower();
                } else {
                    vec[i].lower();
                    break;
                }
            }
            vec[i].higher();
            if prev != vec[i] {
                c = true;
            }
        }
        if !c {
            cont = false
        }
    }
}
fn how_many_fall(vec: &mut [Brick]) -> usize {
    let mut set = HashSet::new();
    let mut cont = true;
    while cont {
        let mut c = false;
        for i in 0..vec.len() {
            let prev = vec[i];
            while !collision_with(vec, i) {
                if vec[i].start.z > 1 {
                    vec[i].lower();
                } else {
                    vec[i].lower();
                    break;
                }
            }
            vec[i].higher();
            if prev != vec[i] {
                c = true;
                set.insert(i);
            }
        }
        if !c {
            cont = false
        }
    }
    set.len()
}
fn execute1(vec: &Vec<Brick>) -> u32 {
    let mut ctr = 0;
    for i in 0..vec.len() {
        let mut clone = vec.clone();
        clone.remove(i);
        if !smth_fall(&mut clone) {
            ctr += 1;
        }
    }
    ctr
}
fn execute2(vec: &Vec<Brick>) -> usize {
    (0..vec.len())
        //.into_par_iter()
        .map(|i| {
            let mut clone = vec.clone();
            clone.remove(i);
            how_many_fall(&mut clone)
        })
        .sum()
}
fn smth_fall(vec: &mut [Brick]) -> bool {
    for i in 0..vec.len() {
        let prev = vec[i];
        while !collision_with(vec, i) {
            if vec[i].start.z > 1 {
                vec[i].lower();
            } else {
                vec[i].lower();
                break;
            }
        }
        vec[i].higher();
        if prev != vec[i] {
            return true;
        }
    }

    false
}
fn collision_with(vec: &[Brick], x: usize) -> bool {
    for i in 0..vec.len() {
        if i != x && vec[i].collide(&vec[x]) {
            return true;
        }
    }
    false
}
pub fn part_two(input: &str) -> Option<usize> {
    let mut vec = parse(input);
    fall(&mut vec);
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
