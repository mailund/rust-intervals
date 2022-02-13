use super::*;

use std::ops::{Deref, DerefMut, Index, IndexMut, Range};

/// Trait that different types of sequences must implement.
/// The generic parameter T is a hack so we can work with both
/// generic and concrete underlying types.
pub trait SeqTrait {
    /// The type a sequence is a sequence of
    type Type;
}

// SECTION: Slices -- the fundamental type here
#[derive(Debug)]
#[repr(transparent)] // Because of this we can soundly cast `&{mut }IdxSlice<T>` to `&{mut }[T]`.
pub struct IdxSlice<_Tag: SeqTrait>([_Tag::Type]);

impl<'a, _Tag> From<&'a [_Tag::Type]> for &'a IdxSlice<_Tag>
where
    _Tag: SeqTrait,
{
    fn from(v: &'a [_Tag::Type]) -> &'a IdxSlice<_Tag> {
        unsafe { &*(v as *const [_Tag::Type] as *const IdxSlice<_Tag>) }
    }
}
impl<'a, _Tag> From<&'a mut [_Tag::Type]> for &'a mut IdxSlice<_Tag>
where
    _Tag: SeqTrait,
{
    fn from(v: &'a mut [_Tag::Type]) -> &'a mut IdxSlice<_Tag> {
        unsafe { &mut *(v as *mut [_Tag::Type] as *mut IdxSlice<_Tag>) }
    }
}

impl<Idx, _Tag> Index<Idx> for IdxSlice<_Tag>
where
    _Tag: SeqTrait,
    Idx: IndexType,
    Idx: CanIndex<_Tag>,
{
    type Output = _Tag::Type;
    fn index(&self, idx: Idx) -> &Self::Output {
        &self.0[idx.index()]
    }
}
impl<Idx, _Tag> IndexMut<Idx> for IdxSlice<_Tag>
where
    _Tag: SeqTrait,
    Idx: IndexType,
    Idx: CanIndex<_Tag>,
{
    fn index_mut(&mut self, idx: Idx) -> &mut Self::Output {
        &mut self.0[idx.index()]
    }
}

impl<Idx, _Tag> Index<Range<Idx>> for IdxSlice<_Tag>
where
    _Tag: SeqTrait,
    Idx: IndexType,
    Idx: CanIndex<_Tag>,
{
    type Output = IdxSlice<_Tag>;
    fn index(&self, idx: Range<Idx>) -> &Self::Output {
        self.0[idx.start.index()..idx.end.index()].into()
    }
}

impl<Idx, _Tag> IndexMut<Range<Idx>> for IdxSlice<_Tag>
where
    _Tag: SeqTrait,
    Idx: IndexType,
    Idx: CanIndex<_Tag>,
{
    fn index_mut(&mut self, idx: Range<Idx>) -> &mut Self::Output {
        (&mut self.0[idx.start.index()..idx.end.index()]).into()
    }
}
// FIXME: And so on, for all range types...

// SECTION: Other sequence types
#[derive(Debug)]
pub struct IdxVec<_Tag: SeqTrait>(pub Vec<_Tag::Type>);
impl<_Tag: SeqTrait> From<Vec<_Tag::Type>> for IdxVec<_Tag> {
    fn from(v: Vec<_Tag::Type>) -> IdxVec<_Tag> {
        IdxVec::<_Tag>(v)
    }
}
impl<_Tag: SeqTrait> Deref for IdxVec<_Tag> {
    type Target = IdxSlice<_Tag>;
    fn deref(&self) -> &Self::Target {
        self.0.as_slice().into()
    }
}
impl<_Tag: SeqTrait> DerefMut for IdxVec<_Tag> {
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice().into()
    }
}

// FIXME: other containers

// SECTION: tests
#[cfg(test)]
mod test {
    use crate::*;

    #[rustfmt::skip]
    mod types {
        use crate::*;
        use std::marker::PhantomData;

        #[derive(Clone, Copy, Debug)]
        pub struct Foo();
        impl SeqTrait for Foo { type Type = u32; }
        #[derive(Clone, Copy, Debug)]
        pub struct ST<T>(PhantomData<T>);
        impl<T> SeqTrait for ST<T> { type Type = T; }

        #[derive(Clone, Copy)]
        pub struct X{}
        impl TypeTrait for X { type Type = u32; }
        impl<T> CanIndex<Vec<T>> for X {}
        impl<T> CanIndex<[T]> for X {}
        impl<T> CanIndex<ST<T>> for X {}

        #[derive(Clone, Copy)]
        pub struct Y{}
        impl TypeTrait for Y { type Type = i64; }
        impl CanIndex<Foo> for Y {}
        impl<T> CanIndex<Vec<T>> for Y {}
    }
    use types::*;

    #[test]
    fn test_new_design() {
        let x: Val<X> = Val(0);
        let y: Val<Y> = Val(0);
        let v: Vec<u32> = vec![1, 2, 3, 4, 5];
        let w: &[u32] = &v[2..];
        println!("v[x] = {}", v[x]);
        println!("w[x] = {}", w[x]);
        println!("v[y] = {}", v[y]);

        let v: IdxVec<Foo> = vec![1, 2, 3, 4, 5].into();
        println!("{:?}", v);
        //println!("{:?}", v[x]);
        println!("{:?}", v[y]);

        let v: IdxVec<ST<u32>> = vec![1, 2, 3, 4, 5].into();
        let (i, j): (Val<X>, Val<X>) = (0.into(), 3.into());
        let w = &v[i..j];
        println!("w = {:?}", &w);
        println!("v[x] = {}", v[x]);
        println!("w[x] = {}", w[x]);
        // println!("v[y] = {}", v[y]);

        //assert!(false);
    }
}
