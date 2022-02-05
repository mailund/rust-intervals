macro_rules! def_index {
    // E.g. def_idx!(Vec<T>[Idx] => T)
    //               ^^^^^^ -- sequence type
    //                      ^^^ -- index type
    //                              ^ -- return type
    ( $($seq:ty[$idx:ty] => $res:ty),* ) => {
        $(
            impl<T> std::ops::Index<$idx> for $seq {
                type Output = $res;
                fn index(&self, i: $idx) -> &Self::Output {
                    &self[i.as_index()]
                }
            }
            impl<T> std::ops::Index<&$idx> for $seq {
                type Output = $res;
                fn index(&self, i: &$idx) -> &Self::Output {
                    &self[i.as_index()]
                }
            }
            impl<T> std::ops::IndexMut<$idx> for $seq {
                fn index_mut(&mut self, i: $idx) -> &mut Self::Output {
                    &mut self[i.as_index()]
                }
            }
            impl<T> std::ops::IndexMut<&$idx> for $seq {
                fn index_mut(&mut self, i: &$idx) -> &mut Self::Output {
                    &mut self[i.as_index()]
                }
            }
        )*
    };
}

pub(crate) use def_index;
