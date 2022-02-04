use super::index_macros::*;
use super::ops_macros::*;
use super::wrapper::*;

def_num_wrapper!(Idx wrapping usize);
def_num_wrapper!(Offset wrapping isize);

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
def_op!(Idx + Offset => Idx);
def_op!(Offset + Idx => Idx);
def_op!(Idx += Offset);

// You can subtract an index and an offset
def_op!(Idx - Offset => Idx);
def_op!(Idx -= Offset);

// You can subtract two indices, but you can't add
// them (adding indices do not usually make sense)
def_op!(Idx - Idx => Offset);

// You can add scalars to the two types.
def_op!(Idx + usize => Idx);
def_op!(usize + Idx => Idx);
def_op!(Idx += usize);
def_op!(Offset + isize => Offset);
def_op!(isize + Offset => Offset);
def_op!(Offset += isize);

// You can subtract scalars from the two types.
def_op!(Idx - usize => Idx);
def_op!(usize - Idx => Idx);
def_op!(Idx -= usize);
def_op!(Offset - isize => Offset);
def_op!(isize - Offset => Offset);
def_op!(Offset -= isize);

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
        assert_eq!(j - k, l);
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
