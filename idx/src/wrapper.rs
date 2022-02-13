// SECTION: Traits for keeping track of types and type information

use super::macros::*;
use num::{cast, Num, NumCast};
use std::cmp::{Ordering, PartialEq, PartialOrd};
use std::convert::From;
use std::fmt;


/// Trait that must be satisfied for wrapped types.
pub trait NumType: Num + NumCast + Clone + Copy + PartialEq + PartialOrd {
    fn cast<T: NumType>(self) -> T { cast::<Self, T>(self).unwrap() }
}

// NumType is implemented by all types all that satisfy the constraints.
// This just make NumType a short-hand for the required traits
impl<T> NumType for T where T: Num + NumCast + Clone + Copy + PartialEq + PartialOrd {}


/// This is the trait that defines new types. The wrapper Val handles all
/// the functionality, but this trait is used as a tag to distinguish
/// between conceptually different types.
#[rustfmt::skip]
pub trait TypeTrait: Clone { type Type: NumType; }

// The primitive types should also be type traits...
#[rustfmt::skip]
macro_rules! base_trait {
    ( $t:ty ) => { impl TypeTrait for $t { type Type = $t; }  };
}
apply_base_types!(base_trait);

/// Wrapper type.
#[derive(Clone, Copy, Debug)]
pub struct Val<_Tag: TypeTrait>(pub _Tag::Type);

// Expose the type trait directly from the Val wrapper
impl<_Tag: TypeTrait> TypeTrait for Val<_Tag> {
    type Type = _Tag::Type;
}

// Type dispatch for casting. FIXME: must be a simple way
#[rustfmt::skip]
mod val_casting {
    use super::*;
    pub trait WrapperTrait {
        type InType: NumType;
        fn wrap(val: Self::InType) -> Self;
    }
    impl<T: NumType> WrapperTrait for T {
        type InType = T;
        fn wrap(val: Self::InType) -> Self { val }
    }
    impl<_Tag: TypeTrait> WrapperTrait for Val<_Tag> {
        type InType = _Tag::Type;
        fn wrap(val: Self::InType) -> Self { Val::<_Tag>(val) }
    }
}
use val_casting::*;

impl<_Tag> Val<_Tag>
where
    _Tag: TypeTrait,
{
    #[allow(dead_code)]
    #[inline]
    pub fn cast<W: WrapperTrait>(self) -> W {
        W::wrap(self.0.cast::<W::InType>())
    }
}

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
impl<_Tag, T: NumCast> From<T> for Val<_Tag>
where
    _Tag: TypeTrait,
{
    // FIXME: This panics if you cannot cast the input value. The TryFrom trait seems better
    // but I cannot convince Rust to let me implement it. It says that Into is already implemented
    // (behind my back, I might add), so there already is a TryFrom. Fuck it. Just don't make
    // Val values with overflow if you want to survive.
    #[inline]
    fn from(t: T) -> Self {
        let val = cast::<T, _Tag::Type>(t).unwrap();
        Val(val)
    }
}

// Get an ordering on it
impl<_Tag> PartialEq for Val<_Tag>
where
    _Tag: TypeTrait,
{
    #[rustfmt::skip]
    #[inline]
    fn eq(&self, other: &Val<_Tag>) -> bool { self.0 == other.0 }
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

#[cfg(test)]
mod test {
    #[warn(unused_imports)]
    use super::*;

    #[rustfmt::skip]
    mod new_types {
        use crate::TypeTrait;

        #[derive(Clone, Copy)]
        pub struct I {}
        impl TypeTrait for I { type Type = usize; }

        #[derive(Clone, Copy)]
        pub struct J {}
        impl TypeTrait for J { type Type = i32; }
    }
    use new_types::*;

    #[test]
    fn test_creating_types() {
        let i: Val<I> = 0.into();
        let j: Val<I> = 10.into();
        let k: Val<J> = 10.into();
        assert_eq!(k.cast::<usize>(), j.0 - i.0);
        let _: usize = i.cast();
        let _: Val<J> = i.cast();
    }
}
