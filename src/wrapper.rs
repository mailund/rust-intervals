// SECTION: Traits for keeping track of types and type information

use super::*;

use num::NumCast;
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::fmt;

/// Implement this to behave like a number
pub trait NumberType {
    type Type: NumCast;
    fn value(&self) -> Self::Type;
    #[inline]
    fn value_as<T: NumCast>(&self) -> T {
        num::cast::<Self::Type, T>(self.value()).unwrap()
    }
}

// IntegerType for primitive types; we will wrap those for specific
// type-safe types. Having the traits for all numbers makes meta-programming
// a lot easier. Numerical types just wrap themselves.
// It would be nicer to implement this as generics but num::PrimInt can be
// extended, so that limits what we are allowed to implement of generics
// based on these traits.
macro_rules! def_wrap_primitive {
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
        )*
    };
}
def_wrap_primitive!(usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8);

/// Trait that must be satisfied for wrapped types.
pub trait NumType: NumCast + Copy + PartialEq + PartialOrd {}
// Hack to make a NumType out of all that satisfy the constraints.
// This just make NumType a short-hand for the required traits
impl<T> NumType for T where T: NumCast + Copy + PartialEq + PartialOrd {}

/// This is the trait that defines new types. The wrapper Val handles all
/// the functionality, but this trait is used as a tag to distinguish
/// between conceptually different types.
pub trait TypeTrait {
    type Type: NumType;
}

#[derive(Clone, Copy, Debug)]
pub struct Val<_Tag>(pub _Tag::Type)
where
    _Tag: TypeTrait;

impl<_Tag> fmt::Display for Val<_Tag>
where
    _Tag: TypeTrait,
    _Tag::Type: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Implementing From just to make it easier to create objects
impl<_Tag, T> From<T> for Val<_Tag>
where
    _Tag: TypeTrait,
    T: NumCast,
{
    fn from(t: T) -> Self {
        Val(num::cast::<T, _Tag::Type>(t).unwrap())
    }
}

// Get an ordering on it
impl<_Tag> PartialEq for Val<_Tag>
where
    _Tag: TypeTrait,
{
    #[inline]
    fn eq(&self, other: &Val<_Tag>) -> bool {
        self.0 == other.0
    }
}
impl<_Tag> PartialOrd for Val<_Tag>
where
    _Tag: TypeTrait,
{
    #[inline]
    fn partial_cmp(&self, other: &Val<_Tag>) -> Option<Ordering> {
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

#[allow(unused_macros)]
#[allow(unused_imports)]
macro_rules! new_index_types {
    ($(
        $name:ident[$type:ty]
        $(for
            $( $seq:ty $( where < $($meta:ident),+ > meta)? ),+
        )?
     ;)+
    ) => {
        $(
            #[derive(Clone, Copy, Debug)]
            pub struct $name();
            impl $crate::wrapper::TypeTrait for $name {
                type Type = $type;
            }
            $(
                $(
                    impl$( < $($meta),+ >)? CanIndexTag<$seq> for $name {}
                )+
            )?
        )+
    };
}
pub(crate) use new_index_types;

#[cfg(test)]
mod test {
    use super::*;
    type_rules! {
        indices: {
            I[u32];
            J[usize];
        }
        operations: {
            [I] - [I] => [J];
            [J] += usize;
        }
    }

    #[test]
    fn test_creating_types() {
        let i: Val<I> = 0.into();
        let j: Val<I> = 10.into();
        let mut k: Val<J> = 10.into();
        assert_eq!(k, j - i);
        k += 5;
        assert_eq!(k, 15.into());
    }
}
