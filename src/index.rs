use super::ops_macros::*;
use super::wrapper::*;

def_wrapper!(Idx, usize);
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

// You can add an index and an offset
def_add!(Idx, Offset, Idx);
def_add!(Offset, Idx, Idx);
def_add_assign!(Idx, Offset);

// You can subtract an index and an offset
def_sub!(Idx, Offset, Idx);
def_sub!(Offset, Idx, Idx);
def_sub_assign!(Idx, Offset);

// You can subtract two indices, but you can't add
// them (adding indices do not usually make sense)
def_sub!(Idx, Idx, Offset);

// You can add scalars to the two types.
def_add!(Idx, usize, Idx);
def_add!(usize, Idx, Idx);
def_add_assign!(Idx, usize);
def_add!(Offset, isize, Offset);
def_add!(isize, Offset, Offset);
def_add_assign!(Offset, isize);

// You can subtract scalars from the two types.
def_sub!(Idx, usize, Idx);
def_sub!(usize, Idx, Idx);
def_sub_assign!(Idx, usize);
def_sub!(Offset, isize, Offset);
def_sub!(isize, Offset, Offset);
def_sub_assign!(Offset, isize);

macro_rules! def_index {
    ($t:ty) => {
        impl<T> std::ops::Index<$t> for Vec<T> {
            type Output = T;
            fn index(self: &Vec<T>, i: $t) -> &T {
                &self[i.wrapped()]
            }
        }
        impl<T> std::ops::IndexMut<$t> for Vec<T> {
            fn index_mut(&mut self, i: $t) -> &mut Self::Output {
                &mut self[i.wrapped()]
            }
        }
        impl<T> std::ops::Index<$t> for [T] {
            type Output = T;
            fn index<'a>(self: &'a [T], i: $t) -> &'a T {
                &self[i.wrapped()]
            }
        }
        impl<T> std::ops::IndexMut<$t> for [T] {
            fn index_mut<'a>(&'a mut self, i: $t) -> &'a mut Self::Output {
                &mut self[i.wrapped()]
            }
        }
    };
}
def_index!(Idx);

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
