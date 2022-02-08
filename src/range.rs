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

// Trait trickery to specify which sequences we can index with a range
pub trait RangeIndexConstraits<Seq, Of> {
    fn index_closed(&self, start: usize, end: usize) -> &[Of];
    fn index_left(&self, start: usize) -> &[Of];
    fn index_right(&self, end: usize) -> &[Of];
    fn index_full(&self) -> &[Of];
}
impl<Seq, Of> RangeIndexConstraits<Seq, Of> for Seq
where
    Seq: std::ops::Index<std::ops::Range<usize>, Output = [Of]>,
    Seq: std::ops::Index<std::ops::RangeFrom<usize>, Output = [Of]>,
    Seq: std::ops::Index<std::ops::RangeTo<usize>, Output = [Of]>,
    Seq: std::ops::Index<std::ops::RangeFull, Output = [Of]>,
{
    fn index_closed(&self, start: usize, end: usize) -> &[Of] {
        &self[start..end]
    }
    fn index_left(&self, start: usize) -> &[Of] {
        &self[start..]
    }
    fn index_right(&self, end: usize) -> &[Of] {
        &self[..end]
    }
    fn index_full(&self) -> &[Of] {
        &self[..]
    }
}

/*
impl<Of> RangeIndexConstraits<[Of], Of> for [Of]
where
    Of: std::marker::Sized,
    [Of]: std::slice::SliceIndex<[Of]>,
{
    fn index_closed<'a>(seq: &'a [Of], start: usize, end: usize) -> &'a [Of] {
        &seq[start..end]
    }
    fn index_left<'a>(seq: &'a [Of], start: usize) -> &'a [Of] {
        &seq[start..]
    }
    fn index_right<'a>(seq: &'a [Of], end: usize) -> &'a [Of] {
        &seq[..end]
    }
    fn index_full<'a>(seq: &'a [Of]) -> &'a [Of] {
        &seq[..]
    }
}
*/

// Generic trickery for getting an index
pub struct RangeIndex<Idx, Seq, Of> {
    pub _idx: std::marker::PhantomData<Idx>,
    pub _seq: std::marker::PhantomData<Seq>,
    pub _of: std::marker::PhantomData<Of>,
}
impl<Idx, Seq, Of> RangeIndex<Idx, Seq, Of>
where
    Idx: IndexInfo<IndexType = usize>,
    Seq: RangeIndexConstraits<Seq, Of>,
{
    #[inline]
    fn range_index<'a>(seq: &'a Seq, r: Range<Idx>) -> &'a [Of] {
        match r {
            Range::Closed(start, end) => &seq.index_closed(start.as_index(), end.as_index()),
            Range::Left(start) => &seq.index_left(start.as_index()),
            Range::Right(end) => &seq.index_right(end.as_index()),
            Range::Full => &seq.index_full(),
        }
    }
}

// I can't implement this for general types U (impl<Idx,U> Index<range<Idx>> for U)
// but I can do it for specific ones...
impl<Idx, T> std::ops::Index<Range<Idx>> for Vec<T>
where
    Idx: IndexInfo<IndexType = usize>,
    Vec<T>: RangeIndexConstraits<Vec<T>, T>,
{
    type Output = <Vec<T> as std::ops::Index<std::ops::Range<usize>>>::Output;
    #[inline]
    fn index(&self, r: Range<Idx>) -> &Self::Output {
        RangeIndex::<Idx, Vec<T>, T>::range_index(&self, r)
    }
}

/* THIS SHIT FUCKING DOESN'T WORK
impl<Idx, T> std::ops::Index<Range<Idx>> for &[T]
where
    Idx: IndexInfo<IndexType = usize>,
    T: std::marker::Sized,
    T: RangeIndexConstraits<&[T], T>,
{
    type Output = <&[T] as std::ops::Index<std::ops::Range<usize>>>::Output;
    #[inline]
    fn index(&self, r: Range<Idx>) -> &Self::Output {
        RangeIndex::<Idx, &[T], T>::range_index(&self, r)
    }
}
*/

#[cfg(test)]
mod range_index_experiments {
    use super::*;
    use crate::*;
    def_idx!(Idx with offset isize with sub [
        meta<>: Vec<u32> => u32
    ]);
    #[test]
    fn test_range_index() {
        let r: Range<Idx> = range(Idx::from(1)..Idx::from(4));
        let v: Vec<u32> = vec![0, 1, 2, 3, 4, 5];
        println!("{:?}", v);
        println!("{:?}", v[Idx::from(1)]);
        let w: &[u32] = &v[r];
        println!("{:?}", w);

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

/*
impl<_Tag, T> std::ops::Index<std::ops::Range<Wrapper<_Tag>>> for Vec<T> {
    type Output = [T];
    fn index(&self, i: std::ops::Range<Wrapper<_Tag>>) -> &Self::Output {
        &self[0..4]
    }
}
*/

/*
// See if we can get Range<T> objects to index when <T> can index
/// Any of the wrapped types should have this.

impl<T> crate::wrapper::TypeInfo for std::ops::Range<T>
where
    T: crate::wrapper::TypeInfo,
{
    type WrappedType = std::ops::Range<T::WrappedType>;
}
impl<T> CanIndex for std::ops::Range<T> where T: crate::wrapper::TypeInfo {}
impl<T> IndexInfo for std::ops::Range<T>
where
    T: crate::wrapper::IndexInfo,
{
    type IndexType = std::ops::Range<T::IndexType>;
    fn as_index(&self) -> Self::IndexType {
        self.start.as_index()..self.end.as_index()
    }
}*/

/*

// Ranges are not quite good enough for our purposes. We want
// immutable ranges that we can manipulate as data objecs.
def_obj_wrapper!(Range wrapping ops::Range<Idx>);

// Implement AsIndex for usize ranges so we can use those for slices
use super::wrapper::AsIndex;
impl AsIndex<std::ops::Range<usize>> for Range {
    fn as_index(&self) -> std::ops::Range<usize> {
        self.start.0..self.end.0
    }
}

// Indexing with our new Range type
def_index!(
    Vec<T>[Range] => [T],
    [T][Range] => [T]
);

// Constructors. Since the embedded range is private, these
// are the only ways to create a Range, and they ensures that
// all ranges are valid, i.e. that end >= start.

/// Creates an interval if i <= j and return Some(-).
/// If j < i, the interval is invalid and we return None
pub fn safe_range(i: Idx, j: Idx) -> Option<Range> {
    if i <= j {
        Some(Range(i..j))
    } else {
        None
    }
}
/// Creates a valid interval or panics if [i,j) is not
/// a valid interval.
pub fn range(i: Idx, j: Idx) -> Range {
    safe_range(i, j).unwrap()
}

// I would love to reuse the underlying std::ops::Range iterator, but
// I haven't figured out how to do that without moving it, and I definitely
// do not want that. So, I implement a custom iterator.
// FIXME: I probably should implment iterators going the other direction as
// well, since I have algorithms where I need to run in the backwards
// direction
pub struct RangeForwardIterator {
    cur: Idx,
    step: Offset,
    end: Idx,
}
impl std::iter::Iterator for RangeForwardIterator {
    type Item = Idx;
    fn next(&mut self) -> Option<Self::Item> {
        if self.cur >= self.end {
            None
        } else {
            // Not worrying about overflow right now...
            self.cur += self.step;
            Some(self.cur - self.step)
        }
    }
}

impl Range {
    pub fn iter(&self) -> RangeForwardIterator {
        RangeForwardIterator {
            cur: self.start,
            step: Offset(1),
            end: self.end,
        }
    }
}

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
