use super::*;

/// Macro for defining numerical operators on wrapper types.
macro_rules! def_op {
    ($lhs:ident + $rhs:ident => $res:ident) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = $res;
            #[inline]
            fn add(self, rhs: $rhs) -> Self::Output {
                let lhs: <$res as $crate::wrapper::TypeInfo>::WrappedType =
                    $crate::wrapper::WrapperType::wrapped_as(&self);
                let rhs: <$res as $crate::wrapper::TypeInfo>::WrappedType =
                    $crate::wrapper::WrapperType::wrapped_as(&rhs);
                (lhs + rhs).into()
            }
        }
    };

    ($lhs:ident += $rhs:ident) => {
        impl std::ops::AddAssign<$rhs> for $lhs {
            #[inline]
            fn add_assign(&mut self, rhs: $rhs) {
                let rhs: <$lhs as $crate::wrapper::TypeInfo>::WrappedType =
                    $crate::wrapper::WrapperType::wrapped_as(&rhs);
                self.0 += rhs;
            }
        }
    };

    ($lhs:ident - $rhs:ident => $res:ident) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = $res;
            #[inline]
            fn sub(self, rhs: $rhs) -> Self::Output {
                let lhs: <$res as $crate::wrapper::TypeInfo>::WrappedType =
                    $crate::wrapper::WrapperType::wrapped_as(&self);
                let rhs: <$res as $crate::wrapper::TypeInfo>::WrappedType =
                    $crate::wrapper::WrapperType::wrapped_as(&rhs);
                (lhs - rhs).into()
            }
        }
    };

    ($lhs:ident -= $rhs:ident) => {
        impl std::ops::SubAssign<$rhs> for $lhs {
            #[inline]
            fn sub_assign(&mut self, rhs: $rhs) {
                let rhs: <$lhs as $crate::wrapper::TypeInfo>::WrappedType =
                    $crate::wrapper::WrapperType::wrapped_as(&rhs);
                self.0 -= rhs;
            }
        }
    };
}

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
