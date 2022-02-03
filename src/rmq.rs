use super::index::*;

/// Tests if x is a power of two, x=2^k.
pub fn power_of_two(x: usize) -> bool {
    (x == 0) || ((x & (x - 1)) == 0)
}

/// Type for powers of two, 2^k. Contains k, but wrapped in
/// a type so we don't confuse log-space with linear space.
#[derive(Debug, Clone, Copy)]
pub struct Pow(pub usize);

impl std::cmp::PartialEq for Pow {
    #[inline]
    fn eq(&self, other: &Pow) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::PartialOrd for Pow {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let Pow(i) = *self;
        let Pow(j) = *other;
        Some(i.cmp(&j))
    }
}

impl std::fmt::Display for Pow {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "2^{}", self.0)
    }
}

impl Pow {
    /// for a power Pow(k) get 2^k.
    #[inline]
    pub fn value(&self) -> usize {
        1 << self.0
    }
}

/// Get k such that 2**k is j rounded down to the
/// nearest power of 2.
/// j=1=2^0 => 0
/// j=2=2^1 => 1
/// j=3=2^1+1 => 1
/// j=4=2^2 => 2
/// and so on.
pub fn log2_down(j: usize) -> Pow {
    assert!(j != 0); // not defined for zero

    // Rounded down means finding the index of the first
    // 1 in the bit-pattern. If j = 00010101110
    // then 00010000000 (only first bit) is the closest
    // power of two, and we want the position of that bit.
    // j.leading_zeros() counts the number of leading zeros
    // and we get the index by subtracting this
    // from the total number of bits minus one.
    Pow((usize::BITS - j.leading_zeros() - 1) as usize)
    // usize::BITS and j.leading_zeros() will be u32, so
    // we cast the result back to usize.
}

/// We always have to add one to the exponent, because in log-space
/// we are working with 1-indexed (0-indexed in log-space) values,
/// so to have a table that can handle maximum value k, we need k+1
/// entires. That is what this function gives us.
pub fn log_table_size(n: usize) -> Pow {
    let Pow(k) = log2_down(n);
    Pow(k + 1)
}

/// For n, get (rounded up) log2(n).
pub fn log2_up(n: usize) -> Pow {
    // log_table_size(n) with n=2^k+m will always give us 2^{k+1},
    // whether m is zero or not. We want 2^{k+1} when m > 0 and 2^k
    // when m is zero, i.e. when n is a power of two.
    // So we should subtract one from the exponent if n is a power of two.
    let Pow(k) = log_table_size(n);
    Pow(k - power_of_two(n) as usize)
}

/// Type for indices into blocks. This is just a wrapper for an index,
/// but helps us distinguish between when an index should be considered into
/// a array of blocks and when it is not. Just a little type safety.
#[derive(Debug, Clone, Copy)]
pub struct BlockIdx(pub usize);

impl std::cmp::PartialEq for BlockIdx {
    #[inline]
    fn eq(&self, other: &BlockIdx) -> bool {
        self.0 == other.0
    }
}

impl std::cmp::PartialOrd for BlockIdx {
    fn partial_cmp(&self, other: &Self) -> Option<std::cmp::Ordering> {
        let BlockIdx(i) = *self;
        let BlockIdx(j) = *other;
        Some(i.cmp(&j))
    }
}

impl std::fmt::Display for BlockIdx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}

/// For n and block size bs, compute (r,r*n) where r
/// is n/bs rounded down. That is, r is n divided by bs
/// rounded down, and r*bs is n adjusted downwards to the
/// closest multiple of bs.
pub fn round_down(n: usize, bs: usize) -> (BlockIdx, usize) {
    let r = n / bs;
    (BlockIdx(r), r * bs)
}

/// For n and block size bs, compute (r,r*n) where r
/// is n/bs rounded up. That is, r is n divided by bs
/// rounded down, and r*bs is n adjusted upwards to the
/// closest multiple of bs.
pub fn round_up(n: usize, bs: usize) -> (BlockIdx, usize) {
    let r = (n + bs - 1) / bs;
    (BlockIdx(r), r * bs)
}

#[cfg(test)]
mod tests_math {
    use super::*;
    use more_asserts::*;

    #[test]
    fn test_power_of_two() {
        assert!(power_of_two(0));
        assert!(power_of_two(1));
        assert!(power_of_two(2));
        assert!(!power_of_two(3));
        assert!(power_of_two(4));
        assert!(!power_of_two(5));
        assert!(!power_of_two(6));
        assert!(!power_of_two(7));
        assert!(power_of_two(8));
        assert!(!power_of_two(9));
    }

    #[test]
    fn test_log2_down() {
        assert_eq!(Pow(0), log2_down(1));
        assert_eq!(Pow(1), log2_down(2));
        assert_eq!(Pow(1), log2_down(3));
        assert_eq!(Pow(2), log2_down(4));
        assert_eq!(Pow(2), log2_down(5));
        assert_eq!(Pow(2), log2_down(6));
        assert_eq!(Pow(2), log2_down(7));
        assert_eq!(Pow(3), log2_down(8));
        assert_eq!(Pow(3), log2_down(9));
        for i in 1..100 {
            let k = log2_down(i);
            assert_le!(k.value(), i);
            if power_of_two(i) {
                assert_eq!(i, k.value());
            }
        }
    }

    #[test]
    fn test_log2_up() {
        assert_eq!(Pow(0), log2_up(1));
        assert_eq!(Pow(1), log2_up(2));
        assert_eq!(Pow(2), log2_up(3));
        assert_eq!(Pow(2), log2_up(4));
        assert_eq!(Pow(3), log2_up(5));
        assert_eq!(Pow(3), log2_up(6));
        assert_eq!(Pow(3), log2_up(7));
        assert_eq!(Pow(3), log2_up(8));
        assert_eq!(Pow(4), log2_up(9));
        for i in 1..100 {
            let k = log2_up(i);
            assert_le!(i, k.value());
            if i > 1 && power_of_two(i) {
                assert_eq!(i, k.value());
            }
        }
        for k in 2..10 {
            let i = 1 << k;
            assert_eq!(log2_up(i), Pow(k));
            assert_eq!(log2_down(i), Pow(k));
        }
    }

    #[test]
    fn test_log_table_size() {
        assert_eq!(Pow(1), log_table_size(1));
        assert_eq!(Pow(2), log_table_size(2));
        assert_eq!(Pow(2), log_table_size(3));
        assert_eq!(Pow(3), log_table_size(4));
        assert_eq!(Pow(3), log_table_size(5));
        assert_eq!(Pow(3), log_table_size(6));
        assert_eq!(Pow(3), log_table_size(7));
        assert_eq!(Pow(4), log_table_size(8));
        assert_eq!(Pow(4), log_table_size(9));
    }

    #[test]
    fn test_round() {
        let bs = 4;
        assert_eq!((BlockIdx(0), 0), round_down(0, bs));
        assert_eq!((BlockIdx(0), 0), round_up(0, bs));
        assert_eq!((BlockIdx(0), 0), round_down(1, bs));
        assert_eq!((BlockIdx(1), 4), round_up(1, bs));

        assert_eq!((BlockIdx(0), 0), round_down(2, bs));
        assert_eq!((BlockIdx(1), 4), round_up(2, bs));

        assert_eq!((BlockIdx(0), 0), round_down(3, bs));
        assert_eq!((BlockIdx(1), 4), round_up(3, bs));

        assert_eq!((BlockIdx(1), 4), round_down(4, bs));
        assert_eq!((BlockIdx(1), 4), round_up(4, bs));

        assert_eq!((BlockIdx(1), 4), round_down(5, bs));
        assert_eq!((BlockIdx(2), 8), round_up(5, bs));

        assert_eq!((BlockIdx(1), 4), round_down(6, bs));
        assert_eq!((BlockIdx(2), 8), round_up(6, bs));
    }
}

/// From range [i,j), get values (k,j-2^k) where k is the offset
/// into the TwoD table to look up the value for [i,i+2^k) and [j-2^k,j)
/// from which we can get the RMQ.
pub fn adjusted_index(i: usize, j: usize) -> (Pow, usize) {
    let k = log2_down(j - i);
    (k, j - k.value())
}

/// A rather simple 2D array made from vectors of vectors.
/// There are better solutions, but I can implement those later
/// with the same interface.
pub struct TwoD {
    table: Vec<Vec<usize>>,
}

impl TwoD {
    pub fn new(n: usize) -> TwoD {
        let Pow(logn) = log_table_size(n);
        let table = vec![vec![0; logn]; n];
        TwoD { table }
    }
}

impl std::ops::Index<(usize, Pow)> for TwoD {
    type Output = usize;
    fn index(&self, index: (usize, Pow)) -> &Self::Output {
        match index {
            (i, Pow(k)) => &self.table[i][k],
        }
    }
}

impl std::ops::IndexMut<(usize, Pow)> for TwoD {
    fn index_mut(&mut self, index: (usize, Pow)) -> &mut Self::Output {
        match index {
            (i, Pow(k)) => &mut self.table[i][k],
        }
    }
}

impl std::fmt::Display for TwoD {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        for row in &self.table {
            for val in row {
                let _ = write!(f, "{} ", val);
            }
            let _ = write!(f, "\n");
        }
        Ok(())
    }
}

#[cfg(test)]
mod tests_2d_table {
    use super::*;

    #[test]
    fn test_adjusted_index() {
        // [0,0) is undefined; j must be larger than i
        // [0,1) => offset=1=2^0, k=0, ii=1-2^k=0
        let (Pow(k), ii) = adjusted_index(0, 1);
        assert_eq!(k, 0);
        assert_eq!(ii, 0);

        // [0,2) => offset=2=2^1, k=1, ii=2-2^1=0
        let (Pow(k), ii) = adjusted_index(0, 2);
        assert_eq!(k, 1);
        assert_eq!(ii, 0);

        // [0,3) => offset=2, k=1 -- second offset; then ii=1
        let (Pow(k), ii) = adjusted_index(0, 3);
        assert_eq!(k, 1);
        assert_eq!(ii, 1);

        // [0,4) => offset=4=2^2, k=2, ii=4-4=0
        let (Pow(k), ii) = adjusted_index(0, 4);
        assert_eq!(k, 2);
        assert_eq!(ii, 0);

        let (Pow(k), ii) = adjusted_index(0, 5);
        assert_eq!(k, 2);
        assert_eq!(ii, 1);

        let (Pow(k), ii) = adjusted_index(0, 6);
        assert_eq!(k, 2);
        assert_eq!(ii, 2);

        let (Pow(k), ii) = adjusted_index(0, 7);
        assert_eq!(k, 2);
        assert_eq!(ii, 3);

        let (Pow(k), ii) = adjusted_index(0, 8);
        assert_eq!(k, 3);
        assert_eq!(ii, 0);

        let (Pow(k), ii) = adjusted_index(1, 8);
        assert_eq!(k, 2);
        assert_eq!(ii, 4);

        let (Pow(k), ii) = adjusted_index(1, 9);
        assert_eq!(k, 3);
        assert_eq!(ii, 1);
    }

    #[test]
    fn test_2d() {
        let n = 5;
        let mut tbl = TwoD::new(n);
        println!("{}", tbl);

        for i in 0..n {
            for j in i + 1..n + 1 {
                let (k, _) = adjusted_index(i, j);
                assert_eq!(0, tbl[(i, k)]);
            }
        }

        for i in 0..n {
            for j in i + 1..n + 1 {
                // This is just saving the largest matching j
                // in the entry those js should go to
                let (k, _) = adjusted_index(i, j);
                println!("({},{}) to offset {}", i, j, k);
                tbl[(i, k)] = j;
            }
        }
        println!("{}", tbl);
        for i in 0..n {
            for j in i + 1..n + 1 {
                let (k, _) = adjusted_index(i, j);
                println!("({},{}): {} <? {}", i, j, j, tbl[(i, k)]);
                assert!(j <= tbl[(i, k)]);
            }
        }
    }
}

/// Takes (a,b,op(a,b)) and lift it
macro_rules! lift_binop {
    ($a:ident, $b:ident, $expr:expr) => {
        match (&$a, &$b) {
            (&None, &None) => None,
            (&Some(_), &None) => $a,
            (&None, &Some(_)) => $b,
            (&Some($a), &Some($b)) => Some($expr),
        }
    };
}

pub trait Min {
    fn min(a: Self, b: Self) -> Self;
}

#[inline]
pub fn min<T: Min>(a: T, b: T) -> T {
    T::min(a, b)
}

#[inline]
pub fn min3<T: Min>(a: T, b: T, c: T) -> T {
    min(min(a, b), c)
}

// Lift min to Option<T>
impl<T> Min for Option<T>
where
    T: Min + Copy,
{
    #[inline]
    fn min(a: Self, b: Self) -> Self {
        lift_binop!(a, b, T::min(a, b))
    }
}

use super::index::*;

/// A point is an index with the corresponding value
#[derive(Clone, Copy)]
pub struct Point(pub Idx, pub u32);

impl Point {
    #[inline]
    pub fn idx(&self) -> Idx {
        self.0
    }
    #[inline]
    pub fn val(&self) -> u32 {
        self.1
    }

    pub fn new(i: Idx, x: &[u32]) -> Point {
        Point(i, x[i.0]) // FIXME
    }
}

impl Min for Point {
    #[inline]
    fn min(p1: Point, p2: Point) -> Point {
        if p1.idx() > p2.idx() {
            // min is symmetric, giving preference to the smallest index,
            // so if p2 has the smallest index, we flip the points.
            return Self::min(p2, p1);
        }
        // Pick the smallest value, but in case of ties, pick p1.
        match p1.val() <= p2.val() {
            true => p1,
            false => p2,
        }
    }
}

impl std::fmt::Display for Point {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "Point({},{})", self.0, self.1)
    }
}

/// RMQ table that tabulates all [i,i+2^k] ranges (there are O(n log n)),
/// form which we can get the RMQ from the table by splitting [i,j) into
/// two, [i,2^k) and [j-2^k,j) (where k is the largest such k). We can get
/// the RMQ from those two intervals with a table lookup in O(1) and then
/// pick the one of those with the smallest value.
/// The result is O(n log n) preprocessing and O(1) lookup.
pub struct PowerRMQImpl {
    lcp: Vec<u32>,
    tbl: TwoD,
}

impl PowerRMQImpl {
    fn point(&self, i: Idx) -> Point {
        Point(i, self.lcp[i])
    }
    fn new(lcp: Vec<u32>) -> PowerRMQImpl {
        let n = lcp.len();

        // When tbl is a TwoD table, interpret tbl[i,Pow(k)] as containing
        // values (at powers of two) in the range [i,i+2^k).
        let Pow(logn) = log_table_size(n);
        let mut tbl = TwoD::new(n);

        // Base case: intervals [i,i+1) = [i,i+2^0).
        for i in 0..n {
            tbl[(i, Pow(0))] = i;
        }

        // Dynamic programming construction of tables of increasing length.
        // We have O(log n) runs of the outer loop and O(n) of the inner,
        // so the total time is O(n log n).
        for k in 1..logn {
            for i in 0..(n - Pow(k - 1).value()) {
                // Interval [i,i+2^k) = [i,i+2^{k-1}) [i+2^{k-1},(i+2^{k-1})+2^{k-1})
                let left = Point::new(Idx(tbl[(i, Pow(k - 1))]), &lcp);
                let right = Point::new(Idx(tbl[(i + Pow(k - 1).value(), Pow(k - 1))]), &lcp);
                let Point(Idx(m), _) = min(left, right); // FIXME
                tbl[(i, Pow(k))] = m;
            }
        }
        PowerRMQImpl { lcp, tbl }
    }
    fn len(&self) -> usize {
        self.lcp.len()
    }
    fn rmq(&self, i: usize, j: usize) -> Point {
        // Work out k so [i,2^k) and [j-2^k,j) are overlapping (and are not overlapping)
        // anything outside of [i,j). Then use the table to get the index with the smallest
        // lcp in those intervals, and pick the smaller of the two (with the first index
        // in case of a tie). All in O(1).
        let (p, ii) = adjusted_index(i, j);
        min(
            self.point(Idx(self.tbl[(i, p)])),
            self.point(Idx(self.tbl[(ii, p)])),
        )
    }
}

use super::interval::*;

/// Finds the left-most index with the smallest value in x.
/// Returns the index of the left-most minimal value and the
/// minimal value. If [i,j) is not a valid interval, you ged
/// None.
pub fn smallest_in_range(x: &[u32], ij: Interval) -> Option<Point> {
    use super::interval::Cases2::*;
    match ij.cases2() {
        Empty => None,
        Range(Idx(i), Idx(j)) => {
            let y = &x[i..j];
            let min_val = y.iter().min()?;
            let pos = i + y.iter().position(|a| a == min_val)?;
            Some(Point(Idx(pos), *min_val))
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    fn check_min_in_interval(lcp: &[u32], ij: Interval) {
        use Cases2::*;
        if let Range(Idx(i), Idx(j)) = ij.cases2() {
            let Point(Idx(k), _) = smallest_in_range(lcp, range(Idx(i), Idx(j))).unwrap();
            assert!(i <= k);
            assert!(k < j);

            let v = lcp[k];
            for l in i..k {
                assert!(lcp[l] > v);
            }
            for l in k + 1..j {
                assert!(lcp[l] >= v)
            }
        }
    }

    fn check_min(lcp: &[u32]) {
        for i in 0..lcp.len() {
            for j in i + 1..lcp.len() + 1 {
                check_min_in_interval(lcp, range(Idx(i), Idx(j)))
            }
        }
    }
    fn check_rmq() {
        // Not power of two
        let v = vec![2, 1, 2, 5, 3, 6, 1, 3, 7, 4];
        check_min(&v);
        // Power of two
        let v = vec![2, 1, 2, 5, 3, 6, 1, 3, 7, 4, 2, 6, 3, 4, 7, 9];
        check_min(&v);
        // Not power of two
        let v = vec![2, 1, 2, 0, 2, 1, 3, 7, 4];
        check_min(&v);
        // Power of two
        let v = vec![2, 1, 2, 5, 3, 6, 1, 3];
        check_min(&v);
    }

    #[test]
    fn test_rmq_power() {
        // First a few checks of the Power specific table...
        // can we handle the diagonal (base case of the dynamic programming),
        // and can we handle the cases where we only look up in the table?
        let v = vec![2, 1, 2, 5, 3, 6, 1, 3, 7, 4, 1, 2, 4, 5, 6, 7];
        let rmqa = PowerRMQImpl::new(v.clone());
        println!("{:?}", &v);
        println!("{}", rmqa.tbl);
        println!("{}", rmqa.rmq(0, rmqa.len()));

        // Checking diagonal
        for i in 0..v.len() {
            assert_eq!(Idx(i), rmqa.rmq(i, i + 1).idx());
        }

        // Checking powers
        for i in 0..v.len() {
            for k in [0, 1, 2, 3] {
                let j = i + (1 << k);
                if j > v.len() {
                    continue;
                }
                let i1 = smallest_in_range(&v, range(Idx(i), Idx(j))).unwrap().idx();
                let i2 = rmqa.rmq(i, j).idx();
                println!(
                    "[{},{}): {}, {}, [offset={}] {:?}",
                    i,
                    j,
                    i1,
                    i2,
                    i,
                    &v[i..j]
                );
                assert_eq!(i1, i2);
            }
        }
    }
}
