use super::*;

use std::ops::{Index, IndexMut};

// Uniform interface to base types and Val<_Tag> indices.
#[rustfmt::skip]
mod index_type {
    // Using a private Seal to avoid problems with the orphan rule
    // when we implement generic indexing.
    mod private {
        use crate::macros::*;
        use crate::{TypeTrait, Val};

        pub trait Seal {}

        macro_rules! seal_base { ($t:ty) => { impl Seal for $t {} } }
        apply_base_types!(seal_base);

        impl<_Tag: TypeTrait> Seal for Val<_Tag> {}
    }

    use crate::{NumType, TypeTrait, Val};
    pub trait IndexType: private::Seal {
        fn index(self) -> usize;
    }
    impl<T: NumType + private::Seal> IndexType for T {
        fn index(self) -> usize { self.cast() }
    }
    impl<_Tag: TypeTrait> IndexType for Val<_Tag> {
        fn index(self) -> usize { self.cast() }
    }
}
pub use index_type::IndexType;

// SECTION Constraining indexing
/// Trait that determines what an index can index into.
pub trait CanIndex<T: ?Sized> {}

// A Val can index if its trait can index
impl<_Tag, T> CanIndex<T> for Val<_Tag>
where
    _Tag: TypeTrait,
    _Tag: CanIndex<T>,
{
}

// SECTION Indexing Rust sequences with Val index.
impl<_Tag, T> Index<Val<_Tag>> for Vec<T>
where
    _Tag: TypeTrait,
    _Tag: CanIndex<Vec<T>>,
    Val<_Tag>: IndexType,
{
    type Output = T;
    #[inline]
    fn index(&self, i: Val<_Tag>) -> &Self::Output {
        &self[i.index()]
    }
}

impl<_Tag, T> IndexMut<Val<_Tag>> for Vec<T>
where
    _Tag: TypeTrait,
    _Tag: CanIndex<Vec<T>>,
    Val<_Tag>: IndexType,
{
    #[inline]
    fn index_mut(&mut self, i: Val<_Tag>) -> &mut Self::Output {
        &mut self[i.index()]
    }
}

impl<_Tag, T> Index<Val<_Tag>> for [T]
where
    _Tag: TypeTrait,
    _Tag: CanIndex<[T]>,
    Val<_Tag>: IndexType,
{
    type Output = T;
    #[inline]
    fn index(&self, i: Val<_Tag>) -> &Self::Output {
        &self[i.cast::<usize>()/*i.index()*/]
    }
}

impl<_Tag, T> IndexMut<Val<_Tag>> for [T]
where
    _Tag: TypeTrait,
    _Tag: CanIndex<[T]>,
    Val<_Tag>: IndexType,
{
    #[inline]
    fn index_mut(&mut self, i: Val<_Tag>) -> &mut Self::Output {
        &mut self[i.index()]
    }
}

// FIXME: other builtin sequences...

// SECTION: tests
#[cfg(test)]
mod test {
    use crate::*;

    #[rustfmt::skip]
    mod types {
        use crate::*;
        #[derive(Clone, Copy)]
        pub struct X{}
        impl TypeTrait for X { type Type = u32; }
        impl<T> CanIndex<Vec<T>> for X {}
        
        #[derive(Clone, Copy)]
        pub struct Y{}
        impl TypeTrait for Y { type Type = i64; }
        impl<T> CanIndex<[T]> for Y {}
    }
    use types::*;

    #[test]
    fn test_indexing() {
        let i: Val<X> = 0.into();
        let j: Val<Y> = 1.into();
        let v: Vec<u32> = vec![1, 2, 3, 4, 5];
        let w: &[u32] = &v[2..];
        println!("{:?}", v);
        println!("{:?}", w);
        println!("v[i] = {}", v[i]);
        println!("w[j] = {}", w[j]);
    }
}
