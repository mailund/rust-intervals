use super::*;

use std::ops::{Deref, DerefMut, Index, IndexMut, Range};

/// Trait that different types of sequences must implement.
/// The generic parameter T is a hack so we can work with both
/// generic and concrete underlying types.
pub trait SeqTrait {
    type Type; // The type a sequence is a sequence of
}

macro_rules! new_seq_types {
    // Basic concrete type
    ( @ $name:ident[$type:ty] ) => {
        #[derive(Clone, Copy, Debug)]
        pub struct $name();
        impl $crate::SeqTrait for $name {
            type Type = $type;
        }
    };

    // Generic type
    ( @wrap_phantom@ $( $name:ident),+ ) => {
        $( std::marker::PhantomData<$name> ),+
    };
    ( @ < $($meta:ident),+ > $name:ident[$type:ty] ) => {
        // Define the sequence trait type
        pub struct $name<$($meta),+>(
           $crate::new_seq_types!( @wrap_phantom@ $($meta),+ )
        );
        impl< $($meta),+ > Clone for $name< $($meta),+ > {
            fn clone(&self) -> Self {
                $name( $( std::marker::PhantomData::<$meta> ),+ )
            }
        }
        impl< $($meta),+ > std::marker::Copy for $name< $($meta),+ > {}
        impl< $($meta),+ > std::fmt::Debug for $name< $($meta),+ > {
            fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> Result<(), std::fmt::Error> {
                write!(f, stringify!($name));
                Ok(())
            }
        }
        // Implement SeqTrait for all types
        impl< $($meta),+ > $crate::SeqTrait for $name< $($meta),+ > {
            type Type = $type;
        }
    };

    ($( $( < $($meta:ident),+ > )? $name:ident[$type:ty] ),+ ) => {
        $( $crate::new_seq_types!( @ $( < $($meta),+ > )? $name[$type] ); )*
    };
}
pub(crate) use new_seq_types;

#[derive(Debug)]
#[repr(transparent)] // Because of this we can soundly cast `&{mut }IdxSlice<T>` to `&{mut }[T]`.
pub struct IdxSlice<_Tag: SeqTrait, T>(std::marker::PhantomData<_Tag>, [T]);

impl<'a, _Tag, T> From<&'a [T]> for &'a IdxSlice<_Tag, T>
where
    _Tag: SeqTrait,
{
    fn from(v: &'a [T]) -> &'a IdxSlice<_Tag, T> {
        unsafe { &*(v as *const [T] as *const IdxSlice<_Tag, T>) }
    }
}
impl<'a, _Tag, T> From<&'a mut [T]> for &'a mut IdxSlice<_Tag, T>
where
    _Tag: SeqTrait,
{
    fn from(v: &'a mut [T]) -> &'a mut IdxSlice<_Tag, T> {
        unsafe { &mut *(v as *mut [T] as *mut IdxSlice<_Tag, T>) }
    }
}

impl<Idx, _Tag, T> Index<Idx> for IdxSlice<_Tag, T>
where
    _Tag: SeqTrait,
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    type Output = T;
    fn index(&self, idx: Idx) -> &Self::Output {
        &self.1[idx.value_as::<usize>()]
    }
}
impl<Idx, _Tag, T> IndexMut<Idx> for IdxSlice<_Tag, T>
where
    _Tag: SeqTrait,
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    fn index_mut(&mut self, idx: Idx) -> &mut Self::Output {
        &mut self.1[idx.value_as::<usize>()]
    }
}

impl<Idx, _Tag, T> Index<Range<Idx>> for IdxSlice<_Tag, T>
where
    _Tag: SeqTrait,
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    type Output = IdxSlice<_Tag, T>;
    fn index(&self, idx: Range<Idx>) -> &Self::Output {
        self.1[idx.start.value_as::<usize>()..idx.end.value_as::<usize>()].into()
    }
}
impl<Idx, _Tag, T> IndexMut<Range<Idx>> for IdxSlice<_Tag, T>
where
    _Tag: SeqTrait,
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    fn index_mut(&mut self, idx: Range<Idx>) -> &mut Self::Output {
        (&mut self.1[idx.start.value_as::<usize>()..idx.end.value_as::<usize>()]).into()
    }
}
// FIXME: And so on, for all range types...

#[derive(Debug)]
pub struct IdxVec<_Tag: SeqTrait, T>(pub Vec<T>, std::marker::PhantomData<_Tag>);

impl<_Tag, T> From<Vec<T>> for IdxVec<_Tag, T>
where
    _Tag: SeqTrait,
{
    fn from(v: Vec<T>) -> IdxVec<_Tag, T> {
        IdxVec::<_Tag, T>(v, std::marker::PhantomData)
    }
}

impl<_Tag, T> Deref for IdxVec<_Tag, T>
where
    _Tag: SeqTrait,
{
    type Target = IdxSlice<_Tag, T>;
    fn deref(&self) -> &Self::Target {
        self.0.as_slice().into()
    }
}
impl<_Tag, T> DerefMut for IdxVec<_Tag, T>
where
    _Tag: SeqTrait,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice().into()
    }
}

// SECTION: experiments

new_types!(X[u32], Y[i64]);
super::def_ops! {
    [X] + [X] => usize;
    [Y] - [Y] => [Y];
    [Y] += isize
}
new_seq_types! {
    Foo[u32],
    <T> ST[T]
}

impl<T> CanIndexTag<Vec<T>> for X {}
impl CanIndexTag<[u32]> for X {}
impl<T> CanIndexTag<ST<T>> for X {}


#[test]
fn narko() {
    let x: Val<X> = Val(0);
    let y: Val<Y> = Val(64);
    let z: Val<X> = Val(16);
    //println!("{} < {} == {}", x, y, x < y);
    println!("{} < {} == {}", x, z, x < z);

    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    let w: &[u32] = &v[2..];
    println!("v[x] = {}", v[x]);
    println!("w[x] = {}", w[x]);
    // println!("v[y] = {}", v[y]);

    let v: IdxVec<Foo, u32> = vec![1, 2, 3, 4, 5].into();
    println!("{:?}", v);

    let v: IdxVec<ST<u32>, u32> = vec![1, 2, 3, 4, 5].into();
    let (i, j): (Val<X>, Val<X>) = (0.into(), 3.into());
    let w = &v[i..j];
    println!("w = {:?}", &w);
    println!("v[x] = {}", v[x]);
    println!("w[x] = {}", w[x]);
    // println!("v[y] = {}", v[y]);
    assert!(false);
}
