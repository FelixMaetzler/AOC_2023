pub trait NonNegative {}
impl NonNegative for u8 {}
impl NonNegative for u16 {}
impl NonNegative for u32 {}
impl NonNegative for u64 {}
impl NonNegative for u128 {}
impl NonNegative for usize {}
/// Computes the Lowest Common Multiple between two Non-Negative numbers
pub fn lcm<T>(x: T, y: T) -> T
where
    T: Eq
        + NonNegative
        + Clone
        + Default
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    (x.clone() * y.clone()) / gcd(x, y)
}
/// Computes the Greatest Common Divisor in a recursive Way
pub fn gcd<T>(a: T, b: T) -> T
where
    T: Eq + NonNegative + Clone + Default + std::ops::Rem<Output = T>,
{
    if b == T::default() {
        a
    } else {
        gcd(b.clone(), a % b)
    }
}
