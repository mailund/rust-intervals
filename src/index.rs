use super::*;

use std::ops::{Index, IndexMut};

// SECTION Constraining indexing

/// Trait that determines what an index can index into.
pub trait CanIndexTag<T: ?Sized> {}

// usize can index into everything
impl<T> CanIndexTag<T> for usize {}

// A Val can index if its trait can index
impl<_Tag, T> CanIndexTag<T> for Val<_Tag>
where
    _Tag: TypeTrait,
    _Tag: CanIndexTag<T>,
{
}

// SECTION Indexing Rust sequences with Val index.
impl<_Tag, T> Index<Val<_Tag>> for Vec<T>
where
    _Tag: TypeTrait,
    _Tag: CanIndexTag<Vec<T>>,
{
    type Output = T;
    #[inline]
    fn index(&self, i: Val<_Tag>) -> &Self::Output {
        &self[i.value_as::<usize>()]
    }
}

impl<_Tag, T> IndexMut<Val<_Tag>> for Vec<T>
where
    _Tag: TypeTrait,
    _Tag: CanIndexTag<Vec<T>>,
{
    #[inline]
    fn index_mut(&mut self, i: Val<_Tag>) -> &mut Self::Output {
        &mut self[i.value_as::<usize>()]
    }
}

impl<_Tag, T> Index<Val<_Tag>> for [T]
where
    _Tag: TypeTrait,
    _Tag: CanIndexTag<[T]>,
{
    type Output = T;
    #[inline]
    fn index(&self, i: Val<_Tag>) -> &Self::Output {
        &self[i.value_as::<usize>()]
    }
}

impl<_Tag, T> IndexMut<Val<_Tag>> for [T]
where
    _Tag: TypeTrait,
    _Tag: CanIndexTag<[T]>,
{
    #[inline]
    fn index_mut(&mut self, i: Val<_Tag>) -> &mut Self::Output {
        &mut self[i.value_as::<usize>()]
    }
}
