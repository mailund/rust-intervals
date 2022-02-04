// Hack because we don't have typeof
trait Wrapper<Wrapped>
where
    Wrapped: num::traits::NumCast,
{
    fn cast_to_wrapped<T: num::traits::NumCast>(val: T) -> Wrapped {
        num::cast(val).unwrap()
    }
    fn wrapped(&self) -> Wrapped;
}

impl<T: num::NumCast + Copy> Wrapper<T> for T {
    fn wrapped(&self) -> T {
        *self
    }
}

// FIXME: figure out how to make #[derive()] for these
macro_rules! def_wrapper {
    ($name:ident,$w:ty) => {
        // Define the new type...
        #[derive(Copy, Clone, Debug)]
        pub struct $name(pub $w);

        // Get an ordering on it
        impl std::cmp::PartialEq for $name {
            fn eq(&self, other: &$name) -> bool {
                self.0 == other.0
            }
        }
        impl std::cmp::PartialOrd for $name {
            fn partial_cmp(&self, other: &$name) -> Option<std::cmp::Ordering> {
                self.0.partial_cmp(&other.0)
            }
        }

        // Implement Wrapper so we can use that in the other macros
        impl Wrapper<$w> for $name {
            fn wrapped(&self) -> $w {
                self.0
            }
        }
    };
}

/// A type-safe index. The underlying type is usize
/// since all of Rust's indices want that.
def_wrapper!(Idx, usize);

/// A type-safe offset. The underlying type is isize
/// just because it matches the usize in Idx.
def_wrapper!(Offset, isize);

// So we can print the buggers
impl std::fmt::Display for Idx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "±[{}]", self.0)
    }
}

macro_rules! def_add {
    ($lhs:ty, $rhs:ty, $res:ident) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = $res;
            fn add(self, rhs: $rhs) -> Self::Output {
                $res($res::cast_to_wrapped(self.wrapped()) + $res::cast_to_wrapped(rhs.wrapped()))
            }
        }
    };
}

// You can add an index and an offset
def_add!(Idx, Offset, Idx);
def_add!(Offset, Idx, Idx);
impl std::ops::AddAssign<Offset> for Idx {
    fn add_assign(&mut self, rhs: Offset) {
        let Offset(k) = rhs;
        self.0 += k as usize;
    }
}

// You can subtract an index and an offset
impl std::ops::Sub<Offset> for Idx {
    type Output = Idx;
    fn sub(self, rhs: Offset) -> Self::Output {
        let Idx(i) = self;
        let Offset(k) = rhs;
        Idx(i - k as usize)
    }
}
impl std::ops::SubAssign<Offset> for Idx {
    fn sub_assign(&mut self, rhs: Offset) {
        let Offset(k) = rhs;
        self.0 -= k as usize;
    }
}
impl std::ops::Sub<Idx> for Offset {
    type Output = Idx;
    fn sub(self, rhs: Idx) -> Self::Output {
        let Offset(k) = self;
        let Idx(i) = rhs;
        Idx(i + k as usize)
    }
}

// You can subtract two indices, but you can't add
// them (adding indices do not usually make sense)
impl std::ops::Sub<Idx> for Idx {
    type Output = Offset;
    fn sub(self, rhs: Idx) -> Self::Output {
        let Idx(i) = self;
        let Idx(j) = rhs;
        Offset((i + j) as isize)
    }
}

// You can add scalars to the two types.
// I can't allow the scalar on the left-hand side, so this
// will have to do.
def_add!(Idx, usize, Idx);

impl std::ops::AddAssign<usize> for Idx {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
def_add!(Offset, isize, Offset);

impl std::ops::AddAssign<isize> for Offset {
    fn add_assign(&mut self, rhs: isize) {
        self.0 += rhs;
    }
}

// You can subtract scalars from the two types.
// I can't allow the scalar on the left-hand side, so this
// will have to do.
impl std::ops::Sub<usize> for Idx {
    type Output = Idx;
    fn sub(self, rhs: usize) -> Self::Output {
        let Idx(i) = self;
        Idx(i - rhs)
    }
}
impl std::ops::SubAssign<usize> for Idx {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}
impl std::ops::Sub<isize> for Offset {
    type Output = Offset;
    fn sub(self, rhs: isize) -> Self::Output {
        let Offset(k) = self;
        Offset(k - rhs)
    }
}
impl std::ops::SubAssign<isize> for Offset {
    fn sub_assign(&mut self, rhs: isize) {
        self.0 -= rhs;
    }
}

// FIXME: fix the i.0 here to make it more general
macro_rules! def_index {
    ($t:ty) => {
        impl<T> std::ops::Index<$t> for Vec<T> {
            type Output = T;
            fn index(self: &Vec<T>, i: $t) -> &T {
                &self[i.0]
            }
        }
        impl<T> std::ops::IndexMut<$t> for Vec<T> {
            fn index_mut(&mut self, i: $t) -> &mut Self::Output {
                &mut self[i.0]
            }
        }
        impl<T> std::ops::Index<$t> for [T] {
            type Output = T;
            fn index<'a>(self: &'a [T], i: $t) -> &'a T {
                &self[i.0]
            }
        }
        impl<T> std::ops::IndexMut<$t> for [T] {
            fn index_mut<'a>(&'a mut self, i: $t) -> &'a mut Self::Output {
                &mut self[i.0]
            }
        }
    };
}
def_index!(Idx);

// We want to be able to actually index with Idx
/*
impl<T> std::ops::Index<Idx> for Vec<T> {
    type Output = T;
    fn index(self: &Vec<T>, i: Idx) -> &T {
        &self[i.0]
    }
}
impl<T> std::ops::IndexMut<Idx> for Vec<T> {
    fn index_mut(&mut self, i: Idx) -> &mut Self::Output {
        &mut self[i.0]
    }
}
impl<T> std::ops::Index<Idx> for [T] {
    type Output = T;
    fn index<'a>(self: &'a [T], i: Idx) -> &'a T {
        &self[i.0]
    }
}
impl<T> std::ops::IndexMut<Idx> for [T] {
    fn index_mut<'a>(&'a mut self, i: Idx) -> &'a mut Self::Output {
        &mut self[i.0]
    }
}*/

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_basic_index_arithmetic() {
        let i = Idx(0);
        let k = Idx(6);
        let j = Idx(12);

        assert_eq!(i + 1, Idx(1));
        assert_eq!(i + 6, k);

        let l = Offset(6);
        assert_eq!(i - k, l);
        assert_eq!(i + l, k);
        assert_eq!(j - l, k);

        // Type error: i + j;
    }

    #[test]
    fn test_indexing_vectors() {
        let v: Vec<usize> = vec![1, 2, 3, 4];
        let mut w: Vec<usize> = vec![4, 3, 2, 1];

        for i in 0..v.len() {
            assert_eq!(v[i], v[Idx(i)]);
        }
        for i in 0..w.len() {
            w[Idx(i)] = 2 * i;
            assert_eq!(v[i], v[Idx(i)]);
        }

        test_indexing_slices(3, &v[1..4], &mut w[1..4]);
    }

    fn test_indexing_slices(n: usize, v: &[usize], w: &mut [usize]) {
        for i in 0..n {
            assert_eq!(v[i], v[Idx(i)]);
        }
        for i in 0..n {
            w[Idx(i)] = 2 * i;
            assert_eq!(v[i], v[Idx(i)]);
        }
    }
}
