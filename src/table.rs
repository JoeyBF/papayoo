use std::ops::{Index, IndexMut};

pub struct Table<T> {
    inner: Vec<T>,
    dealer: usize,
}

impl<T> Table<T> {
    pub fn new(inner: Vec<T>) -> Self {
        Self { inner, dealer: 0 }
    }

    pub fn len(&self) -> usize {
        self.inner.len()
    }

    pub fn iter_from(&self, index: usize) -> impl Iterator<Item = &T> {
        let (last_half, first_half) = self.inner.split_at(index);
        first_half.iter().chain(last_half.iter())
    }

    pub fn iter(&self) -> impl Iterator<Item = &T> {
        self.iter_from(self.dealer)
    }

    pub fn iter_mut_from(&mut self, index: usize) -> impl Iterator<Item = &mut T> {
        let (last_half, first_half) = self.inner.split_at_mut(index);
        first_half.iter_mut().chain(last_half.iter_mut())
    }

    pub fn iter_mut(&mut self) -> impl Iterator<Item = &mut T> {
        self.iter_mut_from(self.dealer)
    }

    pub fn set_dealer(&mut self, new_dealer: usize) {
        self.dealer = new_dealer;
    }
}

impl<T> Index<usize> for Table<T> {
    type Output = T;

    fn index(&self, index: usize) -> &Self::Output {
        &self.inner[(index + self.dealer) % self.inner.len()]
    }
}

impl<T> IndexMut<usize> for Table<T> {
    fn index_mut(&mut self, index: usize) -> &mut Self::Output {
        let len = self.inner.len();
        &mut self.inner[(index + self.dealer) % len]
    }
}
