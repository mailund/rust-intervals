use crate::*;

#[rustfmt::skip]
mod ops_traits {
    pub trait CanAdd<Rhs> { type Res; } // self + Rhs => Res
    pub trait CanSub<Rhs> { type Res; } // self - Rhs => Res
    pub trait CanMul<Rhs> { type Res; } // self * Rhs => Res
    pub trait CanDiv<Rhs> { type Res; } // self / Rhs => Res
}
use ops_traits::*;

mod generated_ops {
    use std::ops::{Add, Div, Mul, Sub};

    use super::*;
    // Generic on wrapped operands
    macro_rules! gen_generic_op {
        ($Trait:ident, $method:ident, $condition:ident, $op:tt) => {
            impl<Rhs, Lhs> $Trait<Val<Rhs>> for Val<Lhs>
            where
                Rhs: TypeTrait,
                Lhs: TypeTrait,
                Lhs: $condition<Rhs>,
                <Lhs as $condition<Rhs>>::Res: TypeTrait,
                <Lhs as $condition<Rhs>>::Res: From<<<Lhs as $condition<Rhs>>::Res as TypeTrait>::Type>,
            {
                type Output = <Lhs as $condition<Rhs>>::Res;
                fn $method(self, rhs: Val<Rhs>) -> Self::Output {
                    let lhs =
                        num::cast::<<Lhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(self.0)
                            .unwrap();
                    let rhs =
                        num::cast::<<Rhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(rhs.0)
                            .unwrap();
                    (lhs $op rhs).into()
                }
            }
        };
    }
    gen_generic_op!(Add, add, CanAdd, +);
    gen_generic_op!(Sub, sub, CanSub, -);
    gen_generic_op!(Mul, mul, CanMul, *);
    gen_generic_op!(Div, div, CanDiv, -);

    // base types. We have to generate these with macros since the orphan rule
    // won't let us do it with generics.
    macro_rules! base_op {
        ($Trait:ident, $method:ident, $condition:ident, $op:tt, $t:ty) => {
            impl<Lhs> $Trait<$t> for Val<Lhs>
            where
                Lhs: TypeTrait,
                Lhs: $condition<$t>,
                <Lhs as $condition<$t>>::Res: TypeTrait,
                <Lhs as $condition<$t>>::Res: From<<<Lhs as $condition<$t>>::Res as TypeTrait>::Type>,
            {
                type Output = <Lhs as $condition<$t>>::Res;
                fn $method(self, rhs: $t) -> Self::Output {
                    let lhs =
                        num::cast::<<Lhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(self.0)
                            .unwrap();
                    let rhs = num::cast::<$t, <Self::Output as TypeTrait>::Type>(rhs).unwrap();
                    (lhs + rhs).into()
                }
            }
            impl<Rhs> $Trait<Val<Rhs>> for $t
            where
                Rhs: TypeTrait,
                Self: TypeTrait,
                Self: $condition<Rhs>,
                <Self as $condition<Rhs>>::Res: TypeTrait,
                <Self as $condition<Rhs>>::Res: From<<<Self as $condition<Rhs>>::Res as TypeTrait>::Type>,
            {
                type Output = <Self as $condition<Rhs>>::Res;
                fn $method(self, rhs: Val<Rhs>) -> Self::Output {
                    let lhs = num::cast::<$t, <Self::Output as TypeTrait>::Type>(self).unwrap();
                    let rhs =
                        num::cast::<<Rhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(rhs.0)
                            .unwrap();
                    (lhs $op rhs).into()
                }
            }
        };
    }
    mod base_macros {
        use crate::macros::{apply_base_types, apply_macro};
        use crate::ops::ops_traits::*;
        use crate::{TypeTrait, Val};
        use std::ops::{Add, Div, Mul, Sub};
        macro_rules! add_base  { ($t:ty) => { base_op!(Add, add, CanAdd, +, $t); }; }
        macro_rules! sub_base  { ($t:ty) => { base_op!(Sub, sub, CanSub, -, $t); }; }
        macro_rules! mul_base  { ($t:ty) => { base_op!(Mul, mul, CanMul, *, $t); }; }
        macro_rules! div_base  { ($t:ty) => { base_op!(Div, div, CanDiv, /, $t); }; }
        #[rustfmt::skip]
        macro_rules! gen_bases {
            ($op:ident) => { apply_base_types!($op); };
        }
        apply_macro!(gen_bases [add_base, sub_base, mul_base, div_base]);
    }
}

#[cfg(test)]
mod test_ops {
    use super::*;

    // Defining types and traits
    #[rustfmt::skip]
    mod types {
        use super::*;
    
        #[derive(Clone, Copy)]
        pub struct T1 {}
        impl TypeTrait   for T1  { type Type = usize;  }
        impl CanAdd<T1>  for T1  { type Res = Val<T1>; } // T1 + T1 => T1
        impl CanAdd<T2>  for T1  { type Res = Val<T2>; } // T1 + T2 => T2
        impl CanSub<T1>  for T1  { type Res = Val<T2>; } // T1 - T1 => T2
        impl CanSub<i32> for T1  { type Res = Val<T2>; } // T1 - i32 => T2
        impl CanSub<T1>  for i32 { type Res = Val<T2>; } // i32 - T1 => T2

        #[derive(Clone, Copy)]
        pub struct T2 {}
        impl TypeTrait   for T2  { type Type = isize;   }
        impl CanAdd<T2>  for T2  { type Res  = usize;   } // T2 + T2 => usize
        impl CanAdd<u32> for T2  { type Res  = usize;   } // T2 + u32 => usize
        impl CanAdd<T2>  for u32 { type Res  = usize;   } // u32 + T2 => usize
        impl CanMul<i32> for T2  { type Res  = Val<T2>; } // T2 * i32 => T2
        impl CanMul<T2>  for i32 { type Res  = Val<T2>; } // i32 * T2 => T2
    }
    use types::{T1, T2};

    #[test]
    fn test_ops() {
        let i: Val<T1> = 1.into();
        let j: Val<T2> = 3.into();

        // i and j are incompatible types
        // We need to explicitly define operations on them.
        // This is exactly what we want
        println!("{}", i);
        println!("{}", j);
        let ii: Val<T1> = i + i;
        println!("{}", ii);
        let ij: Val<T2> = i + j;
        println!("{}", ij);
        let jj: usize = j + j;
        println!("{}", jj);
        let _: usize = j + 12u32;
        let _: usize = 12u32 + j;

        let _: Val<T2> = i - 12i32;
        let _: Val<T2> = 12i32 - i;

        let _: Val<T2> = 42i32.into();
        let _: Val<T2> = j * 2i32;
        let _: Val<T2> = 2i32 * j;
    }
}
