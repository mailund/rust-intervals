pub mod type_traits {
    pub use num::{cast, NumCast};

    pub trait CastType {
        type Type: NumCast;
        fn cast<T: NumCast>(self) -> T;
    }

    impl<T: NumCast> CastType for T {
        type Type = T;
        fn cast<U: NumCast>(self) -> U {
            cast::<Self::Type, U>(self).unwrap()
        }
    }
}
