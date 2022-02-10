/// Macro for defining numerical operators on wrapper types.
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
pub(crate) use def_ops;
