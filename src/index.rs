/// A type-safe index. The underlying type is usize
/// since all of Rust's indices want that.
#[derive(Copy, Clone, Debug)]
pub struct Idx(pub usize);

/// A type-safe offset. The underlying type is isize
/// just because it matches the usize in Idx.
#[derive(Copy, Clone, Debug)]
pub struct Offset(pub isize);

// So we can print the buggers
impl std::fmt::Display for Idx {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "[{}]", self.0)
    }
}
impl std::fmt::Display for Offset {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "±[{}]", self.0)
    }
}

// We should be able to compare indices and offsets
// (but not between each other as that doesn't make sense)
impl std::cmp::PartialEq for Idx {
    fn eq(&self, other: &Idx) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::PartialOrd for Idx {
    fn partial_cmp(&self, other: &Idx) -> Option<std::cmp::Ordering> {
        let (Idx(i), Idx(j)) = (*self, *other);
        i.partial_cmp(&j)
    }
}
impl std::cmp::PartialEq for Offset {
    fn eq(&self, other: &Offset) -> bool {
        self.0 == other.0
    }
}
impl std::cmp::PartialOrd for Offset {
    fn partial_cmp(&self, other: &Offset) -> Option<std::cmp::Ordering> {
        let (Offset(k), Offset(l)) = (*self, *other);
        k.partial_cmp(&l)
    }
}

// You can add an index and an offset
impl std::ops::Add<Offset> for Idx {
    type Output = Idx;
    fn add(self, rhs: Offset) -> Self::Output {
        let Idx(i) = self;
        let Offset(k) = rhs;
        Idx(i + k as usize)
    }
}
impl std::ops::AddAssign<Offset> for Idx {
    fn add_assign(&mut self, rhs: Offset) {
        let Offset(k) = rhs;
        self.0 += k as usize;
    }
}
impl std::ops::Add<Idx> for Offset {
    type Output = Idx;
    fn add(self, rhs: Idx) -> Self::Output {
        let Offset(k) = self;
        let Idx(i) = rhs;
        Idx(i + k as usize)
    }
}

// You can subtract an index and an offset
impl std::ops::Sub<Offset> for Idx {
    type Output = Idx;
    fn sub(self, rhs: Offset) -> Self::Output {
        let Idx(i) = self;
        let Offset(k) = rhs;
        Idx(i - k as usize)
    }
}
impl std::ops::SubAssign<Offset> for Idx {
    fn sub_assign(&mut self, rhs: Offset) {
        let Offset(k) = rhs;
        self.0 -= k as usize;
    }
}
impl std::ops::Sub<Idx> for Offset {
    type Output = Idx;
    fn sub(self, rhs: Idx) -> Self::Output {
        let Offset(k) = self;
        let Idx(i) = rhs;
        Idx(i + k as usize)
    }
}

// You can subtract two indices, but you can't add
// them (adding indices do not usually make sense)
impl std::ops::Sub<Idx> for Idx {
    type Output = Offset;
    fn sub(self, rhs: Idx) -> Self::Output {
        let Idx(i) = self;
        let Idx(j) = rhs;
        Offset((i + j) as isize)
    }
}

// You can add scalars to the two types.
// I can't allow the scalar on the left-hand side, so this
// will have to do.
impl std::ops::Add<usize> for Idx {
    type Output = Idx;
    fn add(self, rhs: usize) -> Self::Output {
        let Idx(i) = self;
        Idx(i + rhs)
    }
}
impl std::ops::AddAssign<usize> for Idx {
    fn add_assign(&mut self, rhs: usize) {
        self.0 += rhs;
    }
}
impl std::ops::Add<isize> for Offset {
    type Output = Offset;
    fn add(self, rhs: isize) -> Self::Output {
        let Offset(k) = self;
        Offset(k + rhs)
    }
}
impl std::ops::AddAssign<isize> for Offset {
    fn add_assign(&mut self, rhs: isize) {
        self.0 += rhs;
    }
}

// You can subtract scalars from the two types.
// I can't allow the scalar on the left-hand side, so this
// will have to do.
impl std::ops::Sub<usize> for Idx {
    type Output = Idx;
    fn sub(self, rhs: usize) -> Self::Output {
        let Idx(i) = self;
        Idx(i - rhs)
    }
}
impl std::ops::SubAssign<usize> for Idx {
    fn sub_assign(&mut self, rhs: usize) {
        self.0 -= rhs;
    }
}
impl std::ops::Sub<isize> for Offset {
    type Output = Offset;
    fn sub(self, rhs: isize) -> Self::Output {
        let Offset(k) = self;
        Offset(k - rhs)
    }
}
impl std::ops::SubAssign<isize> for Offset {
    fn sub_assign(&mut self, rhs: isize) {
        self.0 -= rhs;
    }
}

// FIXME: I doubt this will work...
impl<T> std::ops::Index<Idx> for Vec<T> {
    type Output = T;
    fn index(&self, idx: Idx) -> &T {
        let Idx(i) = idx;
        &self[i]
    }
}
/*
impl<T> std::slice::SliceIndex<Idx> {

}
*/
