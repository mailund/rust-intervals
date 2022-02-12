use crate::*;

/// Trait for implementing iteration through i..j ranges for wrapped Val<_Tag>
/// values.
// NB: This requires nightly; the iter::Step trait is unstable.
impl<_Tag> std::iter::Step for Val<_Tag>
where
    _Tag: TypeTrait,
{
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        match (start.0, end.0) {
            (i, j) if i > j => None,
            (i, j) => num::cast(j - i),
        }
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        let count = num::cast::<usize, _Tag::Type>(count)?;
        Some(Val::<_Tag>(start.0 + count))
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        let count = num::cast::<usize, _Tag::Type>(count)?;
        Some(Val::<_Tag>(start.0 - count))
    }
}

#[cfg(test)]
mod range_tests {
    use crate::*;

    #[rustfmt::skip]
    mod new_types {
        use crate::TypeTrait;

        #[derive(Clone, Copy)]
        pub struct Idx {}
        impl TypeTrait for Idx { type Type = usize; }
    }
    use new_types::*;

    #[test]
    fn test_ranges() {
        let i: Val<Idx> = 0.into();
        let j: Val<Idx> = 10.into();
        for k in i..j {
            let kk: Val<Idx> = k;
            println!("{}", kk);
        }
    }
}
