// SECTION: Traits for keeping track of types and type information

/// Any of the wrapped types should have this.
/// You define new types that implement it to define new
/// wrapped types.
pub trait TypeInfo {
    type WrappedType: num::PrimInt;
}

/// Trait for types that can be used for indexing
pub trait IndexType {
    fn as_index(&self) -> usize;
}

/// A few places, this is useful for meta-programming. Mostly because I can't
/// get the From<> trait to work half the time...
pub trait WrapperType: TypeInfo {
    fn wrapped(&self) -> Self::WrappedType;
    #[inline]
    fn wrapped_as<T: num::PrimInt>(&self) -> T {
        num::cast::<Self::WrappedType, T>(self.wrapped()).unwrap()
    }
}

/// For meta-programming. Implemented for types that can index
/// the type Seq
pub trait CanIndex<Seq: ?Sized> {}

// Type info for primitive types; we will wrap those for specific
// type-safe types. Having the traits for all numbers makes meta-programming
// a lot easier. Numerical types just wrap themselves.
// It would be nicer to implement this as generics but num::PrimInt can be
// extended, so that limits what we are allowed to implement of generics
// based on these traits.
macro_rules! def_wrap_index {
    ($($t:ty),*) => {
        $(
            impl TypeInfo for $t
            {
                type WrappedType = $t;
            }
            impl WrapperType for $t
            {
                #[inline]
                fn wrapped(&self) -> Self::WrappedType {
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
            // Generally assuming that any basic integer
            // can be used to index, once the IndexInfo trait
            // is defined
            impl<Seq> CanIndex<Seq> for $t { }
        )*
    };
}
def_wrap_index!(usize, isize, u128, i128, u64, i64, u32, i32, u16, i16, u8, i8);

// SECTION: Now the wrapper type
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
