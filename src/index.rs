macro_rules! def_offset {
    ($offset:ident) => {
        // Offsets wrap isize
        $crate::wrapper::def_wrapped!($offset[isize]);
        // Arithmetic operations
        $crate::ops_macros::def_offset_ops!($offset);
    };
}

// FIXME: this guy can't handle missing meta vars
macro_rules! def_idx {
    ($idx:ident
        with offset $offset:ident
        with sub [ $( meta < $($meta:ident),* >: $seq:ty => $res:ty),* ]) => {
        // Indices wrap usize
        $crate::wrapper::def_wrapped!($idx[usize]);
        // Arithmetic operations
        $crate::ops_macros::def_idx_ops!($idx with offset $offset);
        // Indexing
        $($crate::index_macros::def_index!(meta< $($meta),* >: $seq[$idx] => $res);)*
    };
}

pub(crate) use def_idx;
pub(crate) use def_offset;

#[cfg(test)]
mod index_tests {
    use crate::*;
    // Get a generic offset we can use for all purposes
    // If you don't want it, don't import it
    def_offset!(Offset);

    // Get a generic index we can use for all purposes.
    // It will operate with Offset
    // If you don't want it, don't import it
    def_idx!(
        Idx
        with offset Offset
        with sub [
            meta<T>: Vec<T> => T,
            meta<>: [u32] => u32,
            meta<>: [usize] => usize
        ]
    );

    #[test]
    fn test_basic_index_arithmetic() {
        let i: Idx = 0.into();
        let k: Idx = 6.into();
        let j: Idx = 12.into();

        assert_eq!(i + 1, Idx::from(1));
        assert_eq!(i + 6, k);

        let l: Offset = 6.into();
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
            assert_eq!(v[i], v[Idx::from(i)]);
        }
        for i in 0..w.len() {
            w[Idx::from(i)] = 2 * i;
            assert_eq!(v[i], v[Idx::from(i)]);
        }

        test_indexing_slices(3, &v[1..4], &mut w[1..4]);
    }

    fn test_indexing_slices(n: usize, v: &[usize], w: &mut [usize]) {
        for i in 0..n {
            assert_eq!(v[i], v[Idx::from(i)]);
        }
        for i in Idx::from(0)..Idx::from(n) {
            w[i] = i.wrapped() + 2;
            assert_eq!(v[i], v[Idx::from(i)]);
        }
    }
}
