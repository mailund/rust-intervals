use super::index::*;
use std::ops;

// Trait for implementing iteration through i..j ranges for Idx.
// NB: This requires nightly; the iter::Step trait is unstable.
impl std::iter::Step for Idx {
    fn steps_between(start: &Self, end: &Self) -> Option<usize> {
        match (start, end) {
            (Idx(i), Idx(j)) if i > j => None,
            (Idx(i), Idx(j)) => Some(j - i),
        }
    }
    fn forward_checked(start: Self, count: usize) -> Option<Self> {
        Some(start + count) // Ignoring overflow here...
    }
    fn backward_checked(start: Self, count: usize) -> Option<Self> {
        if start < Idx(count) {
            None
        } else {
            Some(start - count)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ranges() {
        for Idx(i) in Idx(0)..Idx(5) {
            println!("{}", i);
        }
    }
}

// Ranges are not quite good enough for our purposes. We want
// immutable ranges that we can manipulate as data objecs.
/// Wrapper type for better ranges.
// FIXME: Figure out how to get Copy for a fucking ops::Range!!!
#[derive(Clone, Debug)]
pub struct Range(ops::Range<Idx>);

// Deref gives us access to the inner workings of T without
// going through the zero'th index in the wrapper.
impl ops::Deref for Range {
    type Target = ops::Range<Idx>;
    fn deref(&self) -> &Self::Target {
        &self.0
    }
}
// We don't want the mutable interface (DerefMut) since
// this type of Range should be immutable.

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
            // FIXME: not worrying about overflow right now...
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

// Indexing with our new Range type
impl<T> ops::Index<Range> for Vec<T> {
    type Output = [T];
    fn index(self: &Vec<T>, r: Range) -> &Self::Output {
        &self[r.start.0..r.end.0]
    }
}
impl<T> ops::IndexMut<Range> for Vec<T> {
    fn index_mut(self: &mut Vec<T>, r: Range) -> &mut Self::Output {
        &mut self[r.start.0..r.end.0]
    }
}
impl<T> ops::Index<&Range> for Vec<T> {
    type Output = [T];
    fn index(self: &Vec<T>, r: &Range) -> &Self::Output {
        &self[r.start.0..r.end.0]
    }
}
impl<T> ops::IndexMut<&Range> for Vec<T> {
    fn index_mut(self: &mut Vec<T>, r: &Range) -> &mut Self::Output {
        &mut self[r.start.0..r.end.0]
    }
}
impl<T> ops::Index<Range> for [T] {
    type Output = [T];
    fn index<'a>(self: &'a [T], r: Range) -> &'a Self::Output {
        &self[r.start.0..r.end.0]
    }
}
impl<T> ops::IndexMut<Range> for [T] {
    fn index_mut<'a>(self: &'a mut [T], r: Range) -> &'a mut Self::Output {
        &mut self[r.start.0..r.end.0]
    }
}
impl<T> ops::Index<&Range> for [T] {
    type Output = [T];
    fn index<'a>(self: &'a [T], r: &Range) -> &'a Self::Output {
        &self[r.start.0..r.end.0]
    }
}
impl<T> ops::IndexMut<&Range> for [T] {
    fn index_mut<'a>(self: &'a mut [T], r: &Range) -> &'a mut Self::Output {
        &mut self[r.start.0..r.end.0]
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
