use super::*;

/// Macro for defining numerical operators on wrapper types.
macro_rules! def_op {
    ($lhs:ident + $rhs:ident => $res:ident) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = $res;
            fn add(self, rhs: $rhs) -> Self::Output {
                let lhs: <$res as TypeInfo>::WrappedType = self.wrapped_as();
                let rhs: <$res as TypeInfo>::WrappedType = rhs.wrapped_as();
                (lhs + rhs).into()
            }
        }
    };

    ($lhs:ident += $rhs:ident) => {
        impl std::ops::AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                let rhs: <$lhs as TypeInfo>::WrappedType = rhs.wrapped_as();
                self.0 += rhs;
            }
        }
    };

    ($lhs:ident - $rhs:ident => $res:ident) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = $res;
            fn sub(self, rhs: $rhs) -> Self::Output {
                let lhs: <$res as TypeInfo>::WrappedType = self.wrapped_as();
                let rhs: <$res as TypeInfo>::WrappedType = rhs.wrapped_as();
                (lhs - rhs).into()
            }
        }
    };

    ($lhs:ident -= $rhs:ident) => {
        impl std::ops::SubAssign<$rhs> for $lhs {
            fn sub_assign(&mut self, rhs: $rhs) {
                let rhs: <$lhs as TypeInfo>::WrappedType = rhs.wrapped_as();
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
