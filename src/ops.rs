use super::macros::*;
use crate::*;

use num::cast;
use std::ops::{Add, Index};

#[rustfmt::skip]
mod ops_traits {
    pub trait CanAdd<Rhs> { type Res; } // self + Rhs => Res
}
use ops_traits::*;

mod generated_ops {
    use super::*;
    // Generic + that should be constrained based on traits

    // Generic on wrapped operands
    impl<Rhs, Lhs> Add<Val<Rhs>> for Val<Lhs>
    where
        Rhs: TypeTrait,
        Lhs: TypeTrait,
        Lhs: CanAdd<Rhs>,
        <Lhs as CanAdd<Rhs>>::Res: TypeTrait,
        <Lhs as CanAdd<Rhs>>::Res: From<<<Lhs as CanAdd<Rhs>>::Res as TypeTrait>::Type>,
    {
        type Output = <Lhs as CanAdd<Rhs>>::Res;
        fn add(self, rhs: Val<Rhs>) -> Self::Output {
            let lhs = cast::<<Lhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(self.0)
                .unwrap();
            let rhs =
                cast::<<Rhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(rhs.0).unwrap();
            (lhs + rhs).into()
        }
    }

    // base types. We have to generate these with macros since the orphan rule
    // won't let us do it with generics.
    macro_rules! base_op {
        ($($t: ty),+) => {
            $(
                impl<Lhs> Add<$t> for Val<Lhs>
                where
                    Lhs: TypeTrait,
                    Lhs: CanAdd<$t>,
                    <Lhs as CanAdd<$t>>::Res: TypeTrait,
                    <Lhs as CanAdd<$t>>::Res: From<<<Lhs as CanAdd<$t>>::Res as TypeTrait>::Type>,
                {
                    type Output = <Lhs as CanAdd<$t>>::Res;
                    fn add(self, rhs: $t) -> Self::Output {
                        let lhs = cast::<<Lhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(self.0).unwrap();
                        let rhs = cast::<$t, <Self::Output as TypeTrait>::Type>(rhs).unwrap();
                        (lhs + rhs).into()
                    }
                }

                impl<Rhs> Add<Val<Rhs>> for $t
                where
                    Rhs: TypeTrait,
                    Self: TypeTrait,
                    Self: CanAdd<Rhs>,
                    <Self as CanAdd<Rhs>>::Res: TypeTrait,
                    <Self as CanAdd<Rhs>>::Res: From<<<Self as CanAdd<Rhs>>::Res as TypeTrait>::Type>,
                {
                    type Output = <Self as CanAdd<Rhs>>::Res;
                    fn add(self, rhs: Val<Rhs>) -> Self::Output {
                        let lhs = cast::<$t, <Self::Output as TypeTrait>::Type>(self).unwrap();
                        let rhs = cast::<<Rhs as TypeTrait>::Type, <Self::Output as TypeTrait>::Type>(rhs.0).unwrap();
                        (lhs + rhs).into()
                    }
                }
            )+
        };
    }
    apply_base_types!(base_op);
}

// Generic index implementation. (The real situation is a bit more
// complicated because I need to specify which types each index type
// is allowed to index, but this is the gist of it)
impl<_Tag, T> Index<Val<_Tag>> for Vec<T>
where
    _Tag: TypeTrait<Type = usize>, // only for usize to simplify the example
{
    type Output = T;
    fn index(&self, i: Val<_Tag>) -> &Self::Output {
        &self[i.0] // hardwired cast for example purposes
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
        impl TypeTrait  for T1 { type Type = usize;      }
        impl CanAdd<T1> for T1 { type Res = Val<T1>; } // T1 + T1 => T1
        impl CanAdd<T2> for T1 { type Res = Val<T2>; } // T1 + T2 => T2

        #[derive(Clone, Copy)]
        pub struct T2 {}
        impl TypeTrait   for T2  { type Type = usize; }
        impl CanAdd<T2>  for T2  { type Res = usize;  } // T2 + T2 => usize
        impl CanAdd<u32> for T2  { type Res = usize;  } // T2 + u32 => usize
        impl CanAdd<T2>  for u32 { type Res = usize;  } // u32 + u32 => usize
    }
    use types::{T1, T2};

    #[test]
    fn test_ops() {
        let v: Vec<u32> = vec![1, 2, 3, 4, 5];
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
        /* We haven't defined T2 + T1, so this isn't possible
        let ji = j + i;
        println!("{}", ji);
        */

        let _ = j + 12u32;
        let _ = 12u32 + j;

        println!("{}", v[i]);
    }
}

/*
/// Macro for defining numerical operators on wrapper types.
#[allow(unused_macros)]
macro_rules! def_ops {

        // [T] is a wrapped type
        ( @ wrap [$id:ident] ) => { Val<$id> };
        // raw identifier is just kept as it is
        ( @ wrap $id:ident) => { $id };

        // lhs + rhs => res
        ( @ $lhs:tt + $rhs:tt => $res:tt ) => {
            impl std::ops::Add<$crate::def_ops!(@ wrap $rhs)>
                for $crate::def_ops!(@ wrap$lhs)
            {
                type Output = $crate::def_ops!(@ wrap$res);
                #[inline]
                fn add(self, rhs: $crate::def_ops!(@ wrap $rhs)) -> Self::Output {
                    let lhs: <$crate::def_ops!(@ wrap $res) as $crate::wrapper::NumberType>::Type =
                        $crate::wrapper::NumberType::value_as(&self);
                    let rhs: <$crate::def_ops!(@ wrap $res) as $crate::wrapper::NumberType>::Type =
                        $crate::wrapper::NumberType::value_as(&rhs);
                    (lhs + rhs).into()
                }
            }
        };

        // lhs += rhs
        ( @ $lhs:tt += $rhs:tt ) => {
        impl std::ops::AddAssign<$crate::def_ops!(@ wrap $rhs)>
            for $crate::def_ops!(@ wrap$lhs)
        {
            #[inline]
            fn add_assign(&mut self, rhs: $crate::def_ops!(@ wrap $rhs)) {
                let rhs: <$crate::def_ops!(@ wrap $lhs) as $crate::wrapper::NumberType>::Type =
                    $crate::wrapper::NumberType::value_as(&rhs);
                self.0 += rhs;
            }
        }
    };

    // lhs - rhs => res
    ( @ $lhs:tt - $rhs:tt => $res:tt ) => {
        impl std::ops::Sub<$crate::def_ops!(@ wrap $rhs)> for $crate::def_ops!(@ wrap $lhs)
        {
            type Output = $crate::def_ops!(@ wrap $res);
            #[inline]
            fn sub(self, rhs: $crate::def_ops!(@ wrap $rhs)) -> Self::Output {
                let lhs: <$crate::def_ops!(@ wrap $res) as $crate::wrapper::NumberType>::Type =
                    $crate::wrapper::NumberType::value_as(&self);
                let rhs: <$crate::def_ops!(@ wrap $res) as $crate::wrapper::NumberType>::Type =
                    $crate::wrapper::NumberType::value_as(&rhs);
                (lhs - rhs).into()
            }
        }
    };


    ( $( $lhs:tt $op:tt $rhs:tt $( => $res:tt  )? ;)+ ) => {
        $( $crate::def_ops!( @ $lhs $op $rhs $( => $res )? ); )+
    };
}
#[allow(unused_imports)]
pub(crate) use def_ops;
*/
