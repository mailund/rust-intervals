macro_rules! def_index {
    // E.g. def_idx!(meta<T>: Vec<T>[Idx] => T)
    //                    ^ -- generic variables
    //                        ^^^^^^ -- sequence type
    //                               ^^^ -- index type
    //                                       ^ -- return type
    ( meta< $($meta:ident),* >: $seq:ty[$idx:ty] => $res:ty ) => {
        impl<$($meta),*> $crate::wrapper::CanIndex<$seq> for $idx {}
        impl<$($meta),*> std::ops::Index<$idx> for $seq {
            type Output = $res;
            #[inline]
            fn index(&self, i: $idx) -> &Self::Output {
                &self[i.as_index()]
            }
        }
        impl<$($meta),*> std::ops::Index<&$idx> for $seq {
            type Output = $res;
            #[inline]
            fn index(&self, i: &$idx) -> &Self::Output {
                &self[i.as_index()]
            }
        }
        impl<$($meta),*> std::ops::IndexMut<$idx> for $seq {
            #[inline]
            fn index_mut(&mut self, i: $idx) -> &mut Self::Output {
                &mut self[i.as_index()]
            }
        }
        impl<$($meta),*> std::ops::IndexMut<&$idx> for $seq {
            #[inline]
            fn index_mut(&mut self, i: &$idx) -> &mut Self::Output {
                &mut self[i.as_index()]
            }
        }
    };

    ( $seq:ty[$idx:ty] => $res:ty ) => {
        def_index!(: $seq[$idx] => $res);
    };
}

pub(crate) use def_index;
