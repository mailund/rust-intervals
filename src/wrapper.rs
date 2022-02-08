/// Any of the wrapped types should have this.
pub trait TypeInfo {
    type WrappedType: num::PrimInt;
}

/// A few places, this is useful for meta-programming. Mostly because I can't
/// get the From<> trait to work half the time...
pub trait WrapInfo: TypeInfo {
    fn wrapped(&self) -> Self::WrappedType;
    #[inline]
    fn wrapped_as<T: num::PrimInt>(&self) -> T {
        num::cast::<Self::WrappedType, T>(self.wrapped()).unwrap()
    }
}

/// Convinience function for getting the underlying integer from a
/// wrapper. This is slightly easier to use in macros.
#[inline]
pub fn wrapped<T: num::PrimInt>(wrapper: &impl WrapInfo) -> T {
    wrapper.wrapped_as()
}

/// Hack to get the type of a TypeInfo implementation. You can get it
/// as Type<T>::Type where T: TypeInfo.
pub trait TypeTrait {
    type Type;
}
pub struct Type<T: TypeInfo> {
    marker: std::marker::PhantomData<T>,
}
impl<T: TypeInfo> TypeTrait for Type<T> {
    type Type = <T as TypeInfo>::WrappedType;
}

/// Trait for types that can be used for indexing
pub trait IndexInfo {
    type IndexType;
    fn as_index(&self) -> Self::IndexType;
}

/// For meta-programming. If type Sub implements CanIndex<Seq>
/// it means that you can index Seq[Sub] => To.
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
            impl WrapInfo for $t
            {
                #[inline]
                fn wrapped(&self) -> Self::WrappedType {
                    *self
                }
            }
            // These should return usize since that is the basic
            // type for indexing in Rust's slices and vectors.
            impl IndexInfo for $t
            {
                type IndexType = usize;
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
impl<_Tag> WrapInfo for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    #[inline]
    fn wrapped(&self) -> _Tag::WrappedType {
        self.0
    }
}
// When using a wrapped object for indexing, we have a general solution
// as long as the wrapped type is something we can index. We can still specialise
// a wrapper for an indexing type if we wrap one thing and index with another.
impl<_Tag> IndexInfo for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    type IndexType = usize;
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

// This bit requires the paste crate
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
