use super::index::*;

pub struct Interval(Idx, Idx);

/// Creates an interval if i <= j and return Some(-).
/// If j < i, the interval is invalid and we return None
pub fn safe_i(i: Idx, j: Idx) -> Option<Interval> {
    match (i, j) {
        (i, j) if i <= j => Some(Interval(i, j)),
        _ => None,
    }
}
/// Creates a valid interval or panics if [i,j) is not
/// a valid interval.
pub fn i(i: Idx, j: Idx) -> Interval {
    safe_i(i, j).unwrap()
}

// Just so we can print the buggers
impl std::fmt::Display for Interval {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self.indices() {
            (Idx(i), Idx(j)) if i == j => write!(f, "[]"),
            (Idx(i), Idx(j)) if j == i + 1 => write!(f, "[{}]", i),
            (Idx(i), Idx(j)) => write!(f, "[{},{})", i, j),
        }
    }
}

impl Interval {
    /// Is this interval empty?
    pub fn is_empty(&self) -> bool {
        self.0 == self.1
    }
    /// Extract the indices from an interval. You lose the type
    /// information but you get the original numbers back instead.
    pub fn indices(&self) -> (Idx, Idx) {
        (self.0, self.1)
    }
    /// Check if index k is in the interval.
    pub fn contains(&self, k: Idx) -> bool {
        return self.0 <= k && k < self.1;
    }
}

pub struct IntervalIter {
    cur: Idx,
    end: Idx,
}
impl Iterator for IntervalIter {
    type Item = Idx;
    fn next(&mut self) -> Option<Idx> {
        if self.cur == self.end {
            None
        } else {
            let cur = self.cur;
            self.cur += 1;
            Some(cur)
        }
    }
}
impl Interval {
    pub fn iter(&self) -> IntervalIter {
        IntervalIter {
            cur: self.0,
            end: self.1,
        }
    }
}

/// Representation of an interval that separates the two
/// cases of an interval: empty and non-empty
#[derive(Copy, Clone)]
pub enum Cases2 {
    Empty,
    Range(Idx, Idx),
}

/// Representation of an interval that separates the three
/// cases of an interval: empty, singleton, and larger range
#[derive(Copy, Clone)]
pub enum Cases3 {
    Empty,
    Singleton(Idx),
    Range(Idx, Idx),
}

impl Interval {
    /// Get two different cases for an interval: empty and non-empty.
    pub fn cases2(&self) -> Cases2 {
        use Cases2::*;
        match (self.0, self.1) {
            (i, j) if i == j => Empty,
            (i, j) => Range(i, j),
        }
    }

    /// Get three different cases for an interval: empty,
    /// singleton and larger range.
    pub fn cases3(&self) -> Cases3 {
        use Cases3::*;
        match (self.0, self.1) {
            (i, j) if i == j => Empty,
            (i, j) if j == i + 1 => Singleton(i),
            (i, j) => Range(i, j),
        }
    }
}
