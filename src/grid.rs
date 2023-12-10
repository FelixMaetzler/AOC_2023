use std::fmt::Debug;
use std::fmt::Write;
use std::ops::Deref;
use std::ops::DerefMut;
use std::ops::Index;

pub trait OwnIndex<T>
where
    Self: Copy,
{
    fn to_flat_index(&self, grid: &Grid<T>) -> usize;
    fn to_2d_index(&self, grid: &Grid<T>) -> (usize, usize);
}
impl<T> OwnIndex<T> for usize {
    fn to_flat_index(&self, _: &Grid<T>) -> usize {
        *self
    }

    fn to_2d_index(&self, grid: &Grid<T>) -> (usize, usize) {
        (self / grid.cols, self % grid.cols)
    }
}
impl<T> OwnIndex<T> for (usize, usize) {
    fn to_flat_index(&self, grid: &Grid<T>) -> usize {
        debug_assert!(self.0 < grid.height());
        debug_assert!(self.1 < grid.width());
        self.0 * grid.cols + self.1
    }

    fn to_2d_index(&self, _: &Grid<T>) -> (usize, usize) {
        *self
    }
}

pub struct Grid<T> {
    data: Vec<T>,
    rows: usize,
    cols: usize,
}
impl<T> Grid<T> {
    pub fn from_iter(it: impl Iterator<Item = T>, cols: usize) -> Self {
        let data: Vec<_> = it.collect();
        let rows = data.len() / cols;
        assert_eq!(rows * cols, data.len());
        Self { data, rows, cols }
    }
    pub fn from_iter_iter(it: impl Iterator<Item = impl Iterator<Item = T>>) -> Self {
        let mut data = vec![];

        let mut cols = None;
        for v in it {
            data.append(&mut v.collect());
            if cols.is_none() {
                cols = Some(data.len());
            }
        }
        let cols = cols.expect("grid is not empty");
        let rows = data.len() / cols;
        assert_eq!(rows * cols, data.len());
        Self { data, rows, cols }
    }
    pub fn get(&self, index: impl OwnIndex<T>) -> Option<&T> {
        let idx = index.to_2d_index(self);
        if idx.0 < self.height() && idx.1 < self.width() {
            self.data.get(index.to_flat_index(self))
        } else {
            None
        }
    }

    pub fn neighbours4(&self, index: impl OwnIndex<T>) -> Vec<T>
    where
        T: Clone,
    {
        let (y, x) = index.to_2d_index(self);
        let mut ret = vec![];
        if y.checked_sub(1).is_some() {
            ret.push(self.get((y - 1, x)).unwrap().clone())
        }
        if x.checked_sub(1).is_some() {
            ret.push(self.get((y, x - 1)).unwrap().clone())
        }
        if let Some(a) = self.get((y + 1, x)) {
            ret.push(a.clone());
        }
        if let Some(a) = self.get((y, x + 1)) {
            ret.push(a.clone());
        }
        ret
    }
    pub fn neighbours8(&self, index: impl OwnIndex<T>) -> Vec<T>
    where
        T: Clone,
    {
        let (y, x) = index.to_2d_index(self);
        let mut ret = vec![];
        if y.checked_sub(1).is_some() {
            ret.push(self.get((y - 1, x)).unwrap().clone());
            if let Some(a) = self.get((y - 1, x + 1)) {
                ret.push(a.clone());
            }
        }
        if x.checked_sub(1).is_some() {
            ret.push(self.get((y, x - 1)).unwrap().clone());
            if let Some(a) = self.get((y + 1, x - 1)) {
                ret.push(a.clone());
            }
        }
        if x.checked_sub(1).is_some() && y.checked_sub(1).is_some() {
            ret.push(self.get((y - 1, x - 1)).unwrap().clone());
        }
        if let Some(a) = self.get((y + 1, x)) {
            ret.push(a.clone());
        }
        if let Some(a) = self.get((y, x + 1)) {
            ret.push(a.clone());
        }
        if let Some(a) = self.get((y + 1, x + 1)) {
            ret.push(a.clone());
        }
        ret
    }
    pub fn neighbours8_with_index(&self, index: impl OwnIndex<T>) -> Vec<(T, impl OwnIndex<T>)>
    where
        T: Clone,
    {
        let (y, x) = index.to_2d_index(self);
        let mut ret = vec![];
        if y.checked_sub(1).is_some() {
            let index = (y - 1, x);
            ret.push((self.get(index).unwrap().clone(), index));
            let index = (y - 1, x + 1);
            if let Some(a) = self.get(index) {
                ret.push((a.clone(), index));
            }
        }
        if x.checked_sub(1).is_some() {
            let index = (y, x - 1);
            ret.push((self.get(index).unwrap().clone(), index));
            let index = (y + 1, x - 1);
            if let Some(a) = self.get(index) {
                ret.push((a.clone(), index));
            }
        }
        if x.checked_sub(1).is_some() && y.checked_sub(1).is_some() {
            let index = (y - 1, x - 1);
            ret.push((self.get(index).unwrap().clone(), index));
        }
        let index = (y + 1, x);
        if let Some(a) = self.get(index) {
            ret.push((a.clone(), index));
        }
        let index = (y + 1, x + 1);
        if let Some(a) = self.get(index) {
            ret.push((a.clone(), index));
        }
        let index = (y, x + 1);
        if let Some(a) = self.get(index) {
            ret.push((a.clone(), index));
        }

        ret
    }
    pub fn get_north(&self, index: impl OwnIndex<T>) -> Option<(impl OwnIndex<T>, &T)> {
        let index = index.to_2d_index(self);
        if index.0 == 0 {
            None
        } else {
            let index = (index.0 - 1, index.1);
            Some((index, self.get(index).unwrap()))
        }
    }
    pub fn get_south(&self, index: impl OwnIndex<T>) -> Option<(impl OwnIndex<T>, &T)> {
        let index = index.to_2d_index(self);
        let index = (index.0 + 1, index.1);
        if index.0 >= self.height() {
            None
        } else {
            Some((index, self.get(index).unwrap()))
        }
    }
    pub fn get_west(&self, index: impl OwnIndex<T>) -> Option<(impl OwnIndex<T>, &T)> {
        let index = index.to_2d_index(self);
        if index.1 == 0 {
            None
        } else {
            let index = (index.0, index.1 - 1);
            Some((index, self.get(index).unwrap()))
        }
    }
    pub fn get_east(&self, index: impl OwnIndex<T>) -> Option<(impl OwnIndex<T>, &T)> {
        let index = index.to_2d_index(self);
        let index = (index.0, index.1 + 1);
        if index.1 >= self.width() {
            None
        } else {
            Some((index, self.get(index).unwrap()))
        }
    }
    pub fn height(&self) -> usize {
        self.rows
    }
    pub fn width(&self) -> usize {
        self.cols
    }
}
impl<T> Index<(usize, usize)> for Grid<T> {
    type Output = T;

    fn index(&self, index: (usize, usize)) -> &Self::Output {
        &self.data[index.0 * self.cols + index.1]
    }
}
impl<T> Index<usize> for Grid<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.data[index]
    }
}
impl<T> Debug for Grid<T>
where
    T: Debug,
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let s = self
            .data
            .chunks(self.cols)
            .fold("\n".to_string(), |mut output, b| {
                let _ = writeln!(output, "{b:?}");
                output
            });
        write!(f, "{s}")
    }
}
impl<T> IntoIterator for Grid<T> {
    type Item = T;

    type IntoIter = <Vec<T> as IntoIterator>::IntoIter;

    fn into_iter(self) -> Self::IntoIter {
        self.data.into_iter()
    }
}
impl<T> Deref for Grid<T> {
    type Target = [T];

    fn deref(&self) -> &Self::Target {
        &self.data[..]
    }
}
impl<T> DerefMut for Grid<T> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.data[..]
    }
}
