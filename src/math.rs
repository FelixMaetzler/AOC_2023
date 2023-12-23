use crate::NonNegative;

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
/// Computes the Lowest Common Multiple for a slice
pub fn lcm_over_slice<T>(slice: &[T]) -> T
where
    T: Eq
        + NonNegative
        + Clone
        + Default
        + std::ops::Mul<Output = T>
        + std::ops::Div<Output = T>
        + std::ops::Rem<Output = T>,
{
    slice.iter().cloned().reduce(|acc, e| lcm(acc, e)).unwrap()
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
