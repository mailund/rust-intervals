pub mod type_traits {
    // Re-exporting these for code generation
    pub use num::{cast as ncast, NumCast};

    pub trait CastType {
        type Type: NumCast;
        fn cast<T: NumCast>(self) -> T;
    }

    impl<T: NumCast> CastType for T {
        type Type = T;
        #[inline]
        fn cast<U: NumCast>(self) -> U {
            ncast::<Self::Type, U>(self).unwrap()
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
}
