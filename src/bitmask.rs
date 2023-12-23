use crate::NonNegative;
#[derive(Clone, Copy)]
pub struct Bitmask<T: NonNegative>(T);
impl<T> Bitmask<T>
where
    T: NonNegative,
{
    pub fn new(x: T) -> Self {
        Bitmask(x)
    }
    pub fn set(&mut self, i: T)
    where
        T: std::ops::BitOrAssign<T> + std::ops::Shl<Output = T>,
    {
        self.0 |= T::one() << i;
    }
    pub fn get(&self, i: T) -> bool
    where
        T: std::ops::Shl<Output = T> + std::ops::BitAnd<Output = T> + Copy + Eq,
    {
        T::zero() != self.0 & (T::one() << i)
    }
}

impl<T> Default for Bitmask<T>
where
    T: NonNegative + Default,
{
    fn default() -> Self {
        Self(T::default())
    }
}
