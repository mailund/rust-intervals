// SECTION: Traits for keeping track of types and type information

/// Implement this to behave like a number
pub trait NumberType {
    type Type: num::NumCast;
    fn value(&self) -> Self::Type;
    #[inline]
    fn value_as<T: num::NumCast>(&self) -> T {
        num::cast::<Self::Type, T>(self.value()).unwrap()
    }
}

/// Trait for types that can be used for indexing
pub trait IndexType {
    fn as_index(&self) -> usize;
}

// IntegerType for primitive types; we will wrap those for specific
// type-safe types. Having the traits for all numbers makes meta-programming
// a lot easier. Numerical types just wrap themselves.
// It would be nicer to implement this as generics but num::PrimInt can be
// extended, so that limits what we are allowed to implement of generics
// based on these traits.
macro_rules! def_wrap_index {
    ($($t:ty),*) => {
        $(
            impl NumberType for $t
            {
                type Type = $t;
                #[inline]
                fn value(&self) -> Self::Type {
                    *self
                }
            }
            // These should return usize since that is the basic
            // type for indexing in Rust's slices and vectors.
            impl IndexType for $t
            {
                #[inline]
                fn as_index(&self) -> usize {
                    *self as usize
                }
            }
        )*
    };
}
def_wrap_index!(usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8);

pub trait TypeTrait {
    type Type: num::PrimInt;
}

#[derive(Clone, Copy, Debug)]
pub struct Val<_Tag>(pub _Tag::Type)
where
    _Tag: TypeTrait,
    _Tag::Type: Copy;

impl<_Tag> std::fmt::Display for Val<_Tag>
where
    _Tag: TypeTrait,
    _Tag::Type: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implementing From just to make it easier to create objects
impl<T, _Tag> From<T> for Val<_Tag>
where
    T: num::PrimInt,
    _Tag: TypeTrait,
{
    fn from(t: T) -> Self {
        Val(num::cast::<T, _Tag::Type>(t).unwrap())
    }
}

// Get an ordering on it
impl<_Tag> std::cmp::PartialEq for Val<_Tag>
where
    _Tag: TypeTrait,
{
    #[inline]
    fn eq(&self, other: &Val<_Tag>) -> bool {
        self.0 == other.0
    }
}
impl<_Tag> std::cmp::PartialOrd for Val<_Tag>
where
    _Tag: TypeTrait,
{
    #[inline]
    fn partial_cmp(&self, other: &Val<_Tag>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<_Tag> NumberType for Val<_Tag>
where
    _Tag: TypeTrait,
{
    type Type = _Tag::Type;
    fn value(&self) -> Self::Type {
        self.0
    }
}

// We can index if the type is an usize
impl<_Tag> IndexType for Val<_Tag>
where
    _Tag: TypeTrait<Type = usize>,
{
    fn as_index(&self) -> usize {
        self.0
    }
}

macro_rules! new_types {
    ($( $name:ident[$type:ty] ),+ ) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name();
            impl $crate::wrapper::TypeTrait for $name {
                type Type = $type;
            }
        )+
    };
}
pub(crate) use new_types;

// SECTION: Index
pub trait CanIndexTag<T: ?Sized> {}

// usize can index everything
impl<T> CanIndexTag<T> for usize {}

// A Val can index if its trait can index
impl<_Tag, T> CanIndexTag<T> for Val<_Tag>
where
    _Tag: TypeTrait,
    _Tag: CanIndexTag<T>,
{
}

impl<_Tag, T> std::ops::Index<Val<_Tag>> for Vec<T>
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

impl<_Tag, T> std::ops::Index<Val<_Tag>> for [T]
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

// SECTION: hack hack with wrapper sequences
use std::ops::{Deref, DerefMut, Index, IndexMut, Range};

pub trait SeqTrait {}

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
// And so on, for all range types...

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
impl<T> CanIndexTag<Vec<T>> for X {}
impl CanIndexTag<[u32]> for X {}

#[derive(Debug)]
struct ST {}
impl SeqTrait for ST {}
impl CanIndexTag<ST> for X {}

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

    let v: IdxVec<ST, u32> = vec![1, 2, 3, 4, 5].into();
    let (i, j): (Val<X>, Val<X>) = (0.into(), 3.into());
    let w = &v[i..j];
    println!("w = {:?}", &w);
    println!("v[x] = {}", v[x]);
    println!("w[x] = {}", w[x]);
    // println!("v[y] = {}", v[y]);
    assert!(false);
}
