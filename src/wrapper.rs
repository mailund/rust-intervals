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

pub struct Val<T: TypeTrait>(pub T::Type);

impl<_Tag> std::fmt::Display for Val<_Tag>
where
    _Tag: TypeTrait,
    _Tag::Type: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
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

impl<_Tag> IndexType for Val<_Tag>
where
    _Tag: TypeTrait<Type = usize>,
{
    fn as_index(&self) -> usize {
        self.0
    }
}

macro_rules! new_type {
    ($name:ident[$type:ty]) => {
        pub struct $name();
        impl $crate::wrapper::TypeTrait for $name {
            type Type = $type;
        }
    };
}

new_type!(X[u32]);
new_type!(Y[i64]);

fn narko() {
    let x: Val<X> = Val(32);
    let y: Val<Y> = Val(64);
    let z: Val<X> = Val(16);
    //println!("{} < {} == {}", x, y, x < y);
    println!("{} < {} == {}", x, z, x < z);
}

// SECTION: Now the wrapper type
/*
#[derive(Debug, Clone, Copy)]
pub struct Wrapper<_Tag>(pub _Tag::WrappedType)
where
    _Tag: TypeInfo;

impl<_Tag> TypeInfo for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    type WrappedType = _Tag::WrappedType;
}
impl<_Tag> WrapperType for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    #[inline]
    fn wrapped(&self) -> _Tag::WrappedType {
        self.0
    }
}

impl<_Tag> IndexType for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    #[inline]
    fn as_index(&self) -> usize {
        num::cast::<_Tag::WrappedType, usize>(self.0).unwrap()
    }
}

impl<_Tag> std::fmt::Display for Wrapper<_Tag>
where
    _Tag: TypeInfo,
    _Tag::WrappedType: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Get an ordering on it
impl<_Tag> std::cmp::PartialEq for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    #[inline]
    fn eq(&self, other: &Wrapper<_Tag>) -> bool {
        self.0 == other.0
    }
}
impl<_Tag> std::cmp::PartialOrd for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    #[inline]
    fn partial_cmp(&self, other: &Wrapper<_Tag>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T, _Tag> From<T> for Wrapper<_Tag>
where
    T: num::PrimInt,
    _Tag: TypeInfo,
{
    #[inline]
    fn from(t: T) -> Self {
        Wrapper::<_Tag>(num::cast::<T, _Tag::WrappedType>(t).unwrap())
    }
}

// This bit requires the paste crate.
// It defines a new type and a wrapper for it. The type is used
// as a tag to make the type unique, but the main functionality is in
// Wrapper. The wrapper just needs to know about TypeInfo and then
// it will handle the rest
macro_rules! def_wrapped {
    ($name:ident[$wrapped:ty]) => {
        paste::paste! {
            #[derive(Debug, Clone, Copy)]
            pub struct [<_ $name tag>]();
            impl crate::wrapper::TypeInfo for [<_ $name tag>] {
                type WrappedType = $wrapped;
            }
            pub type $name = crate::wrapper::Wrapper<[<_ $name tag>]>;
        }
    };
}
pub(crate) use def_wrapped;
*/
