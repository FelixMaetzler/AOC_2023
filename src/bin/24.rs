use std::str::FromStr;

struct Hailstone {
    pos: Point,
    vel: Point,
}
impl FromStr for Hailstone {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let (pos, vel) = s.split_once(" @ ").unwrap();
        let pos = Point::from_str(pos).unwrap();
        let vel = Point::from_str(vel).unwrap();
        Ok(Self { pos, vel })
    }
}
struct Point {
    x: i64,
    y: i64,
    z: i64,
}
impl FromStr for Point {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let mut it = s.split(',');
        let x = it.next().unwrap().trim().parse().unwrap();
        let y = it.next().unwrap().trim().parse().unwrap();
        let z = it.next().unwrap().trim().parse().unwrap();
        debug_assert_eq!(it.next(), None);
        Ok(Self { x, y, z })
    }
}
enum Intersection {
    ParallelOrIdentical,
    Past,
    Intersection(f64, f64),
}
advent_of_code::solution!(24);

pub fn part_one(input: &str) -> Option<u32> {
    execute(input, 200_000_000_000_000, 400_000_000_000_000)
}
fn execute(input: &str, min: i64, max: i64) -> Option<u32> {
    let vec = parse(input);
    let mut ctr = 0;
    for i in 0..vec.len() - 1 {
        for j in i + 1..vec.len() {
            if solve(&vec[i], &vec[j], min, max) {
                ctr += 1;
            }
        }
    }
    Some(ctr)
}
fn solve(h1: &Hailstone, h2: &Hailstone, min: i64, max: i64) -> bool {
    let inter = calc_intersection(h1, h2);
    if let Intersection::Intersection(x, y) = inter {
        (min as f64 <= x && x <= max as f64) && (min as f64 <= y && y <= max as f64)
    } else {
        false
    }
}
fn calc_intersection(h1: &Hailstone, h2: &Hailstone) -> Intersection {
    let det = det_2x2(h1.vel.x, -h2.vel.x, h1.vel.y, -h2.vel.y);
    if det == 0 {
        return Intersection::ParallelOrIdentical;
    }
    let s = -h2.vel.y * (h2.pos.x - h1.pos.x) + h2.vel.x * (h2.pos.y - h1.pos.y);
    let t = -h1.vel.y * (h2.pos.x - h1.pos.x) + h1.vel.x * (h2.pos.y - h1.pos.y);
    let s = (s as f64) / (det as f64);
    let t = (t as f64) / (det as f64);
    if s < 0.0 || t < 0.0 {
        return Intersection::Past;
    }
    let x = h1.pos.x as f64 + s * h1.vel.x as f64;
    let y = h1.pos.y as f64 + s * h1.vel.y as f64;
    Intersection::Intersection(x, y)
}
fn det_2x2(a: i64, b: i64, c: i64, d: i64) -> i64 {
    a * d - b * c
}
pub fn part_two(input: &str) -> Option<u32> {
    None
}
fn parse(input: &str) -> Vec<Hailstone> {
    input
        .trim()
        .lines()
        .map(|l| Hailstone::from_str(l).unwrap())
        .collect()
}
#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_part_one() {
        let result = execute(&advent_of_code::template::read_file("examples", DAY), 7, 27);
        assert_eq!(result, Some(2));
    }
    #[test]
    fn test_part_one_actual() {
        let result = part_one(&advent_of_code::template::read_file("inputs", DAY));
        assert_eq!(result, Some(11_995));
    }

    #[test]
    fn test_part_two() {
        let result = part_two(&advent_of_code::template::read_file("examples", DAY));
        assert_eq!(result, None);
    }
}
