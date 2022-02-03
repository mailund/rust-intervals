/// Representation of an interval that separates the three
/// cases of an interval into separate constructors to make
/// manipulation of intervals type-safe.
#[derive(Copy, Clone)]
pub enum Interval<T> {
    Empty(T), // empty interval starting at a given position
    Singleton(T),
    Int(T, T),
}
use Interval::*;

/// If you are not sure if [i,j) is a valid interval, use this
/// function to create it. You get None if j < i and Some(-)
/// interval otherwise.
pub fn safe_int<T>(i: T, j: T) -> Option<Interval<T>>
where
    T: num::Num + std::cmp::PartialOrd + Copy,
{
    match (i, j) {
        (i, j) if i == j => Some(Empty(i)),
        (i, j) if j == i + T::one() => Some(Singleton(i)),
        (i, j) if i < j => Some(Int(i, j)),
        _ => None,
    }
}

/// When you know you have a valid interval, you can create
/// it with this function. It will panic! if the interval
/// isn't valid after all, so be careful.
pub fn int<T>(i: T, j: T) -> Interval<T>
where
    T: num::Num + std::cmp::PartialOrd + Copy,
{
    safe_int(i, j).unwrap()
}

// Just so we can print the buggers
impl<T> std::fmt::Display for Interval<T>
where
    T: std::fmt::Display + Copy,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            &Int(i, j) => write!(f, "[{},{})", i, j),
            &Singleton(i) => write!(f, "[{}]", i),
            &Empty(i) => write!(f, "]{}[", i),
        }
    }
}

impl<T> Interval<T>
where
    T: num::Num + std::cmp::PartialOrd + Copy,
{
    /// Extract the indices from an interval. You lose the type
    /// information but you get the original numbers back instead.
    pub fn indices(&self) -> (T, T) {
        match self {
            &Int(i, j) => (i, j),
            &Singleton(i) => (i, i + T::one()),
            &Empty(i) => (i, i),
        }
    }
    /// Get the interval as a range; useful for running through
    /// the indices in an interval
    pub fn range(&self) -> std::ops::Range<T> {
        match self {
            &Int(i, j) => i..j,
            &Singleton(i) => i..i + T::one(),
            &Empty(i) => i..i,
        }
    }
    /// Check if index k is in the interval.
    pub fn contains(&self, k: T) -> bool {
        let (i, j) = self.indices();
        return i <= k && k < j;
    }
}
