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
                write!(f, stringify!($name))
            }
        }
        // Implement SeqTrait for all types
        impl< $($meta),+ > $crate::SeqTrait for $name< $($meta),+ > {
            type Type = $type;
        }
    };

    ($( $( < $($meta:ident),+ > )? $name:ident[$type:ty] ;)+ ) => {
        $( $crate::new_seq_types!( @ $( < $($meta),+ > )? $name[$type] ); )*
    };
}
pub(crate) use new_seq_types;

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
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    type Output = _Tag::Type;
    fn index(&self, idx: Idx) -> &Self::Output {
        &self.0[idx.value_as::<usize>()]
    }
}
impl<Idx, _Tag> IndexMut<Idx> for IdxSlice<_Tag>
where
    _Tag: SeqTrait,
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    fn index_mut(&mut self, idx: Idx) -> &mut Self::Output {
        &mut self.0[idx.value_as::<usize>()]
    }
}

#[rustfmt::skip]
impl<Idx, _Tag> Index<Range<Idx>> for IdxSlice<_Tag>
where
    _Tag: SeqTrait,
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    type Output = IdxSlice<_Tag>;
    fn index(&self, idx: Range<Idx>) -> &Self::Output {
        self.0[
            idx.start.value_as::<usize>()
            ..
            idx.end.value_as::<usize>()
        ].into()
    }
}

#[rustfmt::skip]
impl<Idx, _Tag> IndexMut<Range<Idx>> for IdxSlice<_Tag>
where
    _Tag: SeqTrait,
    Idx: NumberType,
    Idx: CanIndexTag<_Tag>,
{
    fn index_mut(&mut self, idx: Range<Idx>) -> &mut Self::Output {
        (&mut self.0[
            idx.start.value_as::<usize>()
            ..
            idx.end.value_as::<usize>()
        ]).into()
    }
}
// FIXME: And so on, for all range types...

#[derive(Debug)]
pub struct IdxVec<_Tag: SeqTrait>(pub Vec<_Tag::Type>);

impl<_Tag> From<Vec<_Tag::Type>> for IdxVec<_Tag>
where
    _Tag: SeqTrait,
{
    fn from(v: Vec<_Tag::Type>) -> IdxVec<_Tag> {
        IdxVec::<_Tag>(v)
    }
}

impl<_Tag> Deref for IdxVec<_Tag>
where
    _Tag: SeqTrait,
{
    type Target = IdxSlice<_Tag>;
    fn deref(&self) -> &Self::Target {
        self.0.as_slice().into()
    }
}
impl<_Tag> DerefMut for IdxVec<_Tag>
where
    _Tag: SeqTrait,
{
    fn deref_mut(&mut self) -> &mut Self::Target {
        self.0.as_mut_slice().into()
    }
}

// SECTION: experiments

macro_rules! type_rules {
    ( $( $block:ident : { $($ops:tt)* })* ) => {
        // Dispatching the blocks
        $( type_rules!(@ $block { $($ops)* }); )*
    };
    (@sequences {$($ops:tt)*}) => { new_seq_types!($($ops)*); };
    (@indices {$($ops:tt)*}) => { new_types!($($ops)*); };
    (@operations {$($ops:tt)*}) => { def_ops!($($ops)*); };
}

type_rules! {
    sequences: {
        Foo[u32];
        <T> ST[T];
    }
    indices: {
        X[u32] for [u32], Vec<T> where <T> meta, ST<T> where <T> meta;
        Y[i64] for Foo, Vec<T> where <T> meta;
    }
    operations: {
        [X] + [X] => usize;
        [Y] - [Y] => [Y];
        [Y] += isize;
    }
}

#[test]
fn test_new_design() {
    let x: Val<X> = Val(0);
    let _y: Val<Y> = Val(64);
    let z: Val<X> = Val(16);
    //println!("{} < {} == {}", x, y, x < y);
    println!("{} < {} == {}", x, z, x < z);

    let v: Vec<u32> = vec![1, 2, 3, 4, 5];
    let w: &[u32] = &v[2..];
    println!("v[x] = {}", v[x]);
    println!("w[x] = {}", w[x]);
    //println!("v[y] = {}", v[y]);

    let v: IdxVec<Foo> = vec![1, 2, 3, 4, 5].into();
    println!("{:?}", v);

    let v: IdxVec<ST<u32>> = vec![1, 2, 3, 4, 5].into();
    let (i, j): (Val<X>, Val<X>) = (0.into(), 3.into());
    let w = &v[i..j];
    println!("w = {:?}", &w);
    println!("v[x] = {}", v[x]);
    println!("w[x] = {}", w[x]);
    // println!("v[y] = {}", v[y]);
    //assert!(false);
}
