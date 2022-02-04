/// Macro for defining numerical operators on wrapper types.
macro_rules! def_op {
    ($lhs:ident + $rhs:ident => $res:ident) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = $res;
            fn add(self, rhs: $rhs) -> Self::Output {
                $res($res::cast_to_wrapped(self.wrapped()) + $res::cast_to_wrapped(rhs.wrapped()))
            }
        }
    };

    ($lhs:ident += $rhs:ident) => {
        impl std::ops::AddAssign<$rhs> for $lhs {
            fn add_assign(&mut self, rhs: $rhs) {
                self.0 += Self::cast_to_wrapped(rhs.wrapped());
            }
        }
    };

    ($lhs:ident - $rhs:ident => $res:ident) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = $res;
            fn sub(self, rhs: $rhs) -> Self::Output {
                $res($res::cast_to_wrapped(self.wrapped()) - $res::cast_to_wrapped(rhs.wrapped()))
            }
        }
    };

    ($lhs:ident -= $rhs:ident) => {
        impl std::ops::SubAssign<$rhs> for $lhs {
            fn sub_assign(&mut self, rhs: $rhs) {
                self.0 -= Self::cast_to_wrapped(rhs.wrapped());
            }
        }
    };
}

pub(crate) use def_op;
