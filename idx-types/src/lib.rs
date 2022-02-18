pub mod type_traits {
    // Re-exporting these for code generation
    pub use num::{cast as ncast, NumCast};

    /// Trait for numerical-like objects we can cast between.
    pub trait CastType {
        type Type: NumCast;
        fn cast<T: NumCast>(&self) -> T;
    }

    impl<T: NumCast + Copy> CastType for T {
        type Type = T;
        #[inline]
        fn cast<U: NumCast>(&self) -> U {
            ncast::<Self::Type, U>(*self).unwrap()
        }
    }

    /// Casts from one CastType to the underlying type of another.
    ///
    /// For meta-programming, it can be more convinient to use
    /// a function than a method, since we can constrait the
    /// type that way, rather than using <type as CastType>
    /// stuff.
    #[inline]
    pub fn cast_underlying<From: CastType, To: CastType>(from: From) -> To::Type {
        from.cast::<To::Type>()
    }

    /// Casts from one CastType to another.
    ///
    /// For meta-programming, it can be more convinient to use
    /// a function than a method, since we can constrait the
    /// type that way, rather than using <type as CastType>
    /// stuff.
    #[inline]
    pub fn cast<From: CastType, To: CastType>(from: From) -> To
    where
        To: std::convert::From<To::Type>,
    {
        cast_underlying::<From, To>(from).into()
    }

    /// Trait for objects we can use to index into a range.
    /// Unsigned values are just their own index, but signed
    /// can also index from the right using negative numbers.
    pub trait IndexType {
        /// Index into a sequence of length n. The method doesn't
        /// have to check for bounds, but for signed values, n can be
        /// used to index from the right.
        fn index(&self, n: usize) -> usize;
    }

    macro_rules! gen_unsigned {
        ($($t:ty),*) => {
            $(
                impl IndexType for $t {
                    #[inline]
                    fn index(&self, _n: usize) -> usize {
                        self.cast()
                    }
                }
            )*
        }
    }
    gen_unsigned!(u8, u16, u32, u64, usize);

    macro_rules! gen_signed {
        ($($t:ty),*) => {
            $(
                impl IndexType for $t {
                    #[inline]
                    fn index(&self, n: usize) -> usize {
                        let mut res: Self = *self;
                        if res < 0 { // if negative, index from the right
                            res += n.cast::<Self>()
                        }
                        res.cast()
                    }
                }
            )*
        }
    }
    gen_signed!(i8, i16, i32, i64, isize);
}
