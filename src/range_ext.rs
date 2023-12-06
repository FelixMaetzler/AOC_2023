use std::ops::Range;

pub trait RangeExt<T>
where
    Self: Sized,
{
    fn status(&self, other: &Self) -> Relation;
    fn get_intersection(&self, other: &Self) -> Option<Self>;
    fn get_non_intersection(&self, other: &Self) -> Count<Self>;
}
#[derive(PartialEq, Eq, Debug)]
pub enum Relation {
    Lower,
    IntersectingLow,
    Contained,
    IntersectingHigh,
    Higher,
}
impl<T> RangeExt<T> for Range<T>
where
    T: PartialOrd + Clone,
{
    fn status(&self, b: &Self) -> Relation {
        if self.end <= b.start {
            return Relation::Lower;
        }
        if self.end > b.start && self.start < b.start {
            return Relation::IntersectingLow;
        }
        if self.start >= b.start && self.end <= b.end {
            return Relation::Contained;
        }
        if self.start < b.end && self.end > b.end {
            return Relation::IntersectingHigh;
        }
        if self.end >= self.start {
            return Relation::Higher;
        }
        unreachable!()
    }

    fn get_intersection(&self, b: &Self) -> Option<Self> {
        match self.status(b) {
            Relation::Lower => None,
            Relation::IntersectingLow => Some(b.clone().start..self.clone().end),
            Relation::Contained => Some(self.clone()),
            Relation::IntersectingHigh => Some(self.clone().start..b.clone().end),
            Relation::Higher => None,
        }
    }

    fn get_non_intersection(&self, b: &Self) -> Count<Self> {
        match self.status(b) {
            Relation::Lower => Count::Single(self.clone()),
            Relation::IntersectingLow => Count::Single(self.clone().start..(b.clone().start)),
            Relation::Contained => Count::None,
            Relation::IntersectingHigh => Count::Single(b.clone().end..self.clone().end),
            Relation::Higher => Count::Single(self.clone()),
            /*
            Relation::Equal => Count::None,
            Relation::Contains => Count::Double(
                self.clone().start..b.clone().start,
                b.clone().end..self.clone().end,
            ),
            */
        }
    }
}
#[derive(PartialEq, Eq, Debug)]
pub enum Count<T> {
    None,
    Single(T),
    Double(T, T),
}
#[test]
fn test_lower() {
    assert_eq!((0..1).status(&(1..2)), Relation::Lower);
    assert_eq!((0..1).get_intersection(&(1..2)), None);
    assert_eq!((0..1).get_non_intersection(&(1..2)), Count::Single(0..1));
}
#[test]
fn test_intersection_low() {
    assert_eq!((0..5).status(&(3..7)), Relation::IntersectingLow);
    assert_eq!((0..5).get_intersection(&(3..7)), Some(3..5));
    assert_eq!((0..5).get_non_intersection(&(3..7)), Count::Single(0..3));
}
#[test]
fn test_is_contained() {
    assert_eq!((3..6).status(&(3..7)), Relation::Contained);
    assert_eq!((3..6).get_intersection(&(3..7)), Some(3..6));
    assert_eq!((3..6).get_non_intersection(&(3..7)), Count::None);
}
#[test]
fn test_intersection_high() {
    assert_eq!((5..10).status(&(3..7)), Relation::IntersectingHigh);
    assert_eq!((5..10).get_intersection(&(3..7)), Some(5..7));
    assert_eq!((5..10).get_non_intersection(&(3..7)), Count::Single(7..10));
}
#[test]
fn test_higher() {
    assert_eq!((1..2).status(&(0..1)), Relation::Higher);
    assert_eq!((1..2).get_intersection(&(0..1)), None);
    assert_eq!((1..2).get_non_intersection(&(0..1)), Count::Single(1..2));
}
/*
#[test]
fn test_contains() {
    assert_eq!((0..10).status(&(3..7)), Relation::Contains);
    assert_eq!((0..10).get_intersection(&(3..7)), Some(3..7));
    assert_eq!(
        (0..10).get_non_intersection(&(3..7)),
        Count::Double(0..3, 7..10)
    );
}
#[test]
fn test_equal() {
    assert_eq!((3..7).status(&(3..7)), Relation::Equal);
    assert_eq!((3..7).get_intersection(&(3..7)), Some(3..7));
    assert_eq!((3..7).get_non_intersection(&(3..7)), Count::None);
}

*/
