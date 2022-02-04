// Hack to work with wrapped types in macros...
pub trait Wrapper<Wrapped> {
    fn wrapped(&self) -> Wrapped;
}

pub trait NumWrapper<Wrapped>: Wrapper<Wrapped>
where
    Wrapped: num::traits::NumCast,
{
    fn cast_to_wrapped<T: num::traits::NumCast>(val: T) -> Wrapped {
        num::cast(val).unwrap()
    }
}

// Wrap basic numeric types...
impl<T> Wrapper<T> for T
where
    T: num::NumCast + Copy,
{
    fn wrapped(&self) -> T {
        *self
    }
}
impl<T: num::NumCast + Copy> NumWrapper<T> for T {}

pub trait AsIndex<Index> {
    fn as_index(&self) -> Index;
}

macro_rules! def_num_wrapper {
    ($name:ident wrapping $w:ty) => {
        // Define the new type...
        /// A type-safe integer type for use with indices and offsets.
        #[derive(Copy, Clone, Debug)]
        pub struct $name(pub $w);

        // Get an ordering on it
        impl std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                self.0 == other.0
            }
        }
        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        // Implement Wrapper so we can use that in the other macros
        impl Wrapper<$w> for $name {
            fn wrapped(&self) -> $w {
                self.0
            }
        }

        // For wrapped numbers, we also have this guy
        impl NumWrapper<$w> for $name {}

        // We need this one if we define index operators for this time
        impl AsIndex<$w> for $name {
            fn as_index(&self) -> $w {
                self.0
            }
        }
    };
}

macro_rules! def_obj_wrapper {
    ($name:ident wrapping $w:ty) => {
        // Define the new type...
        #[derive(Clone, Debug)] // FIXME: maybe not clone?
        pub struct $name(pub $w);

        impl std::ops::Deref for $name {
            type Target = $w;
            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }
    };

    (mut $name:ident wrapping $w:ty) => {
        def_obj_wrapper!($name wrapping $w);

        impl std::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
}

// Exporting macro to crate
pub(crate) use def_num_wrapper;
pub(crate) use def_obj_wrapper;
