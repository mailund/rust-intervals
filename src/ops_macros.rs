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


    ( $( $lhs:tt $op:tt $rhs:tt $( => $res:tt  )? );+ ) => {
        $( $crate::def_ops!( @ $lhs $op $rhs $( => $res )? ); )+
    };
}
pub(crate) use def_ops;

/*
macro_rules! def_offset_ops {
    ($offset:ident) => {
        // Offsets and scalars
        def_op!($offset + isize => $offset);
        def_op!(isize + $offset => $offset);
        def_op!($offset += isize);

        def_op!($offset - isize => $offset);
        def_op!(isize - $offset => $offset);
        def_op!($offset -= isize);
    };
}

macro_rules! def_idx_ops {
    ($idx:ident with offset $offset:ident) => {
        // You can add an index and an offset
        def_op!($idx + $offset => $idx);
        def_op!($offset + $idx => $idx);
        def_op!($idx += $offset);

        // You can subtract an index and an offset
        def_op!($idx - $offset => $idx);
        def_op!($idx -= $offset);

        // You can subtract two indices, but you can't add
        // them (adding indices do not usually make sense)
        def_op!($idx - $idx => $offset);

        // You can add scalars to indices.
        def_op!($idx + usize => $idx);
        def_op!(usize + $idx => $idx);
        def_op!($idx += usize);

        // You can subtract scalars from indices.
        def_op!($idx - usize => $idx);
        def_op!(usize - $idx => $idx);
        def_op!($idx -= usize);
    };
}

pub(crate) use def_idx_ops;
pub(crate) use def_offset_ops;
pub(crate) use def_op;

/* FIXME
#[cfg(test)]
mod ops_macros_test {
    crate::index::def_offset!(Offset);
    crate::index::def_idx!(
        Idx with offset Offset
        with sub []
    );
}
*/
*/
