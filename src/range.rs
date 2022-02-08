use crate::*;

// Trait for implementing iteration through i..j ranges for Idx.
// NB: This requires nightly; the iter::Step trait is unstable.
impl<_Tag> std::iter::Step for Wrapper<_Tag>
where
    _Tag: TypeInfo + Clone,
{
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        match (start.wrapped(), end.wrapped()) {
            (i, j) if i > j => None,
            (i, j) => num::cast(j - i),
        }
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        let count = num::cast::<usize, _Tag::WrappedType>(count)?;
        Some(Wrapper::<_Tag>(start.wrapped() + count))
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        let count = num::cast::<usize, _Tag::WrappedType>(count)?;
        Some(Wrapper::<_Tag>(start.wrapped() - count))
    }
}

#[cfg(test)]
mod range_tests {
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
        with sub []
    );

    #[test]
    fn test_ranges() {
        for i in Idx::from(0)..Idx::from(5) {
            println!("{}", i);
        }
    }
}

#[cfg(test)]
mod experiments {
    use crate::*;
    def_offset!(Offset);
    def_idx!(Idx with offset Offset
             with sub []
    );
}

/// Wrapper of Range that we can work with within Rust's type system
#[derive(Clone, Copy)]
pub enum Range<Idx> {
    Closed(Idx, Idx), // start..end
    Left(Idx),        // start..
    Right(Idx),       // ..end
    Full,             // ..
}

impl<Idx> std::fmt::Display for Range<Idx>
where
    Idx: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            Range::Closed(start, end) => write!(f, "{}..{}", start, end),
            Range::Left(start) => write!(f, "{}..", start),
            Range::Right(end) => write!(f, "..{}", end),
            Range::Full => write!(f, ".."),
        }
    }
}

pub trait GenRange<R, Idx> {
    fn range(r: R) -> Range<Idx>;
}
impl<Idx> GenRange<std::ops::Range<Idx>, Idx> for std::ops::Range<Idx> {
    fn range(r: std::ops::Range<Idx>) -> Range<Idx> {
        Range::Closed(r.start, r.end)
    }
}
impl<Idx> GenRange<std::ops::RangeFrom<Idx>, Idx> for std::ops::RangeFrom<Idx> {
    fn range(r: std::ops::RangeFrom<Idx>) -> Range<Idx> {
        Range::Left(r.start)
    }
}
impl<Idx> GenRange<std::ops::RangeTo<Idx>, Idx> for std::ops::RangeTo<Idx> {
    fn range(r: std::ops::RangeTo<Idx>) -> Range<Idx> {
        Range::Right(r.end)
    }
}
impl<Idx> GenRange<std::ops::RangeFull, Idx> for std::ops::RangeFull {
    fn range(_r: std::ops::RangeFull) -> Range<Idx> {
        Range::Full
    }
}

pub fn range<Idx, R: GenRange<R, Idx>>(r: R) -> Range<Idx> {
    R::range(r)
}

#[cfg(test)]
mod range_experiments {
    use super::*;
    use crate::*;
    def_idx!(Idx with offset isize with sub []);

    #[test]
    fn range_constructor() {
        let (i, j): (Idx, Idx) = (Idx::from(0), Idx::from(10));
        match range(i..j) {
            Range::Closed(left, right) => {
                assert_eq!(i, left);
                assert_eq!(j, right);
            }
            _ => {
                assert!(false);
            }
        };
        match range(i..) {
            Range::Left(left) => {
                assert_eq!(i, left);
            }
            _ => {
                assert!(false);
            }
        };
        match range(..j) {
            Range::Right(right) => {
                assert_eq!(j, right);
            }
            _ => {
                assert!(false);
            }
        };
        match range(..) {
            Range::<Idx>::Full => {}
            _ => {
                assert!(false);
            }
        };
        println!(
            "{} {}, {}, {}",
            range(i..j),
            range(i..),
            range(..j),
            range(..) as Range<Idx>
        );
        // for printing assert!(false);
    }
}

impl<Idx, T> std::ops::Index<Range<Idx>> for Vec<T>
where
    Idx: IndexType,
    Idx: CanIndex<Vec<T>>,
{
    type Output = [T];
    fn index(&self, r: Range<Idx>) -> &Self::Output {
        match r {
            Range::Closed(start, end) => &self[start.as_index()..end.as_index()],
            Range::Left(start) => &self[start.as_index()..],
            Range::Right(end) => &self[..end.as_index()],
            Range::Full => &self[..],
        }
    }
}
impl<Idx, T> std::ops::Index<Range<Idx>> for [T]
where
    Idx: IndexType,
    Idx: CanIndex<[T]>,
{
    type Output = [T];
    fn index(&self, r: Range<Idx>) -> &Self::Output {
        match r {
            Range::Closed(start, end) => &self[start.as_index()..end.as_index()],
            Range::Left(start) => &self[start.as_index()..],
            Range::Right(end) => &self[..end.as_index()],
            Range::Full => &self[..],
        }
    }
}

#[cfg(test)]
mod range_index_sequences {
    use super::*;
    use crate::*;
    def_idx!(Idx with offset isize with sub [
        meta<>: Vec<u32> => u32,
        meta<T>: [T] => T
    ]);
    #[test]
    fn test_range_index() {
        let r: Range<Idx> = range(Idx::from(1)..Idx::from(4));
        let v: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
        println!("{:?}", v);
        println!("{:?}", v[Idx::from(1)]);
        let w: &[u32] = &v[r];
        println!("{:?}", w);
        println!("{:?}", &w[range(Idx::from(0)..)]);

        /* not legal subscript
        let v: Vec<i32> = vec![0, 1, 2, 3, 4, 5];
        println!("{:?}", v);
        println!("{:?}", v[Idx::from(1)]);
        let w: &[i32] = &v[r];
        println!("{:?}", w);
        assert!(false);
        */
    }
}

// I would love to reuse the underlying std::ops::Range iterator, but
// I haven't figured out how to do that without moving it, and I definitely
// do not want that. So, I implement a custom iterator.
// FIXME: I probably should implment iterators going the other direction as
// well, since I have algorithms where I need to run in the backwards
// direction

pub struct RangeForwardIterator<Idx> {
    cur: Idx,
    step: usize,
    end: Idx,
}
impl<Idx> std::iter::Iterator for RangeForwardIterator<Idx>
where
    Idx: Copy,
    Idx: std::cmp::PartialOrd,
    Idx: std::ops::AddAssign<usize>,
{
    type Item = Idx;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            return None;
        }
        let cur = self.cur;
        self.cur += self.step;
        Some(cur)
    }
}

impl<Idx> Range<Idx>
where
    Idx: Copy,
{
    pub fn safe_iter(&self) -> Option<RangeForwardIterator<Idx>> {
        match &self {
            &Range::Closed(start, end) => Some(RangeForwardIterator {
                cur: *start,
                step: 1,
                end: *end,
            }),
            _ => None,
        }
    }
    pub fn iter(&self) -> RangeForwardIterator<Idx> {
        self.safe_iter().unwrap()
    }
}

#[cfg(test)]
mod range_iterator_tests {
    use super::*;

    def_idx!(Idx with offset isize with sub []);

    #[test]
    fn test_iterator() {
        let i = Idx::from(0);
        let j = Idx::from(10);
        let r = range(i..j);
        for (k, l) in r.iter().enumerate() {
            println!("{}", k);
            assert_eq!(k, l.wrapped());
        }
    }
}

/*
/// Representation of an interval that separates the two
/// cases of an interval: empty and non-empty
#[derive(Copy, Clone)]
pub enum Cases2 {
    /// Empty interval
    E,
    /// Non-empty range
    R(Idx, Idx),
}

/// Representation of an interval that separates the three
/// cases of an interval: empty, singleton, and larger range
#[derive(Copy, Clone)]
pub enum Cases3 {
    /// Empty interval
    E,
    /// Singleton: interval with a single element
    S(Idx),
    /// Range larger than one
    R(Idx, Idx),
}

impl Range {
    /// Get two different cases for an interval: empty and non-empty.
    pub fn cases2(&self) -> Cases2 {
        use Cases2::*;
        match (self.start, self.end) {
            (i, j) if i >= j => E,
            (i, j) => R(i, j),
        }
    }

    /// Get three different cases for an interval: empty,
    /// singleton and larger range.
    pub fn cases3(&self) -> Cases3 {
        use Cases3::*;
        match (self.start, self.end) {
            (i, j) if i >= j => E,
            (i, j) if j == i + 1 => S(i),
            (i, j) => R(i, j),
        }
    }
}
*/
