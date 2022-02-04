macro_rules! def_index {
    // a type alone is indexing to scalars
    ($t:ty) => {
        def_index!(__ $t => T);
    };
    // a type followed by [] is indexing slices
    ($t:ty[]) => {
        def_index!(__ $t => [T]);
    };

    // This is the main branch; we only need the others
    // to dispatch to the right one
    (__ $t:ty => $outtype:ty) => {
        impl<T> std::ops::Index<$t> for Vec<T> {
            type Output = $outtype;
            fn index(self: &Vec<T>, i: $t) -> &Self::Output {
                &self[i.wrapped()]
            }
        }
        impl<T> std::ops::IndexMut<$t> for Vec<T> {
            fn index_mut(&mut self, i: $t) -> &mut Self::Output {
                &mut self[i.wrapped()]
            }
        }
        impl<T> std::ops::Index<$t> for [T] {
            type Output = $outtype;
            fn index<'a>(self: &'a [T], i: $t) -> &'a Self::Output {
                &self[i.wrapped()]
            }
        }
        impl<T> std::ops::IndexMut<$t> for [T] {
            fn index_mut<'a>(&'a mut self, i: $t) -> &'a mut Self::Output {
                &mut self[i.wrapped()]
            }
        }

        impl<T> std::ops::Index<&$t> for Vec<T> {
            type Output = $outtype;
            fn index(self: &Vec<T>, i: &$t) -> &Self::Output {
                &self[i.wrapped()]
            }
        }
        impl<T> std::ops::IndexMut<&$t> for Vec<T> {
            fn index_mut(&mut self, i: &$t) -> &mut Self::Output {
                &mut self[i.wrapped()]
            }
        }
        impl<T> std::ops::Index<&$t> for [T] {
            type Output = $outtype;
            fn index<'a>(self: &'a [T], i: &$t) -> &'a Self::Output {
                &self[i.wrapped()]
            }
        }
        impl<T> std::ops::IndexMut<&$t> for [T] {
            fn index_mut<'a>(&'a mut self, i: &$t) -> &'a mut Self::Output {
                &mut self[i.wrapped()]
            }
        }
    };
}

pub(crate) use def_index;
