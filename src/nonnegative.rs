pub trait NonNegative {
    fn one() -> Self;
    fn zero() -> Self;
}
impl NonNegative for u8 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}
impl NonNegative for u16 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}
impl NonNegative for u32 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}
impl NonNegative for u64 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}
impl NonNegative for u128 {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}
impl NonNegative for usize {
    fn one() -> Self {
        1
    }

    fn zero() -> Self {
        0
    }
}
