use crate::*;

/// Trait for implementing iteration through i..j ranges for wrapped Val<_Tag>
/// values.
// NB: This requires nightly; the iter::Step trait is unstable.
impl<_Tag> std::iter::Step for Val<_Tag>
where
    _Tag: TypeTrait,
{
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        match (start.value(), end.value()) {
            (i, j) if i > j => None,
            (i, j) => num::cast(j - i),
        }
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        let count = num::cast::<usize, _Tag::Type>(count)?;
        Some(Val::<_Tag>(start.value() + count))
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        let count = num::cast::<usize, _Tag::Type>(count)?;
        Some(Val::<_Tag>(start.value() - count))
    }
}

#[cfg(test)]
mod range_tests {
    use crate::*;

    type_rules! {
        indices: {
            Idx[usize];
        }
    }

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
