// Any of the wrapped types should have this.
pub trait TypeInfo: Copy {
    type WrappedType: Copy + num::PrimInt;
}

// A few places, this is useful for meta-programming. Mostly because I can't
// get the From<> trait to work half the time...
pub trait WrapInfo: TypeInfo {
    fn wrapped(&self) -> Self::WrappedType;
    fn wrapped_as<T: num::PrimInt + Copy>(&self) -> T {
        num::cast::<Self::WrappedType, T>(self.wrapped()).unwrap()
    }
}

// Type info for primitive types; we will wrap those for specific
// type-safe types. Having the traits for all numbers makes meta-programming
// a lot easier. Numerical types just wrap themselves.
impl<T> TypeInfo for T
where
    T: Copy + num::PrimInt,
{
    type WrappedType = T;
}
impl<T> WrapInfo for T
where
    T: Copy + num::PrimInt,
{
    fn wrapped(&self) -> Self::WrappedType {
        *self
    }
}

#[derive(Debug, Clone, Copy)]
pub struct Wrapper<_Tag>(pub _Tag::WrappedType)
where
    _Tag: TypeInfo;

impl<_Tag> TypeInfo for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    type WrappedType = _Tag::WrappedType;
}
impl<_Tag> WrapInfo for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    fn wrapped(&self) -> _Tag::WrappedType {
        self.0
    }
}

impl<_Tag> std::fmt::Display for Wrapper<_Tag>
where
    _Tag: TypeInfo,
    _Tag::WrappedType: std::fmt::Display,
{
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

// Get an ordering on it
impl<_Tag> std::cmp::PartialEq for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    fn eq(&self, other: &Wrapper<_Tag>) -> bool {
        self.0 == other.0
    }
}
impl<_Tag> std::cmp::PartialOrd for Wrapper<_Tag>
where
    _Tag: TypeInfo,
{
    fn partial_cmp(&self, other: &Wrapper<_Tag>) -> Option<std::cmp::Ordering> {
        self.0.partial_cmp(&other.0)
    }
}

impl<T, _Tag> From<T> for Wrapper<_Tag>
where
    T: num::PrimInt,
    _Tag: TypeInfo,
{
    fn from(t: T) -> Self {
        Wrapper::<_Tag>(num::cast::<T, _Tag::WrappedType>(t).unwrap())
    }
}

macro_rules! def_op {
    ($lhs:ident + $rhs:ident => $res:ident) => {
        impl std::ops::Add<$rhs> for $lhs {
            type Output = $res;
            fn add(self, rhs: $rhs) -> Self::Output {
                let lhs: <$res as TypeInfo>::WrappedType = self.wrapped_as();
                let rhs: <$res as TypeInfo>::WrappedType = rhs.wrapped_as();
                (lhs + rhs).into()
            }
        }
    };

    ($lhs:ident - $rhs:ident => $res:ident) => {
        impl std::ops::Sub<$rhs> for $lhs {
            type Output = $res;
            fn sub(self, rhs: $rhs) -> Self::Output {
                let lhs: <$res as TypeInfo>::WrappedType = self.wrapped_as();
                let rhs: <$res as TypeInfo>::WrappedType = rhs.wrapped_as();
                (lhs - rhs).into()
            }
        }
    };
}

impl<_Tag> std::iter::Step for Wrapper<_Tag>
where
    _Tag: TypeInfo,
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

// This bit requires the paste crate
use paste::paste;
macro_rules! def_wrapped {
    ($name:ident[$wrapped:ty]) => {
        paste! {
            #[derive(Debug, Clone, Copy)]
            pub struct [<_ $name tag>]();
            impl TypeInfo for [<_ $name tag>] {
                type WrappedType = $wrapped;
            }
            pub type $name = Wrapper<[<_ $name tag>]>;
        }
    };
}

def_wrapped!(I[usize]); // Index with usize
def_wrapped!(O[isize]); // Offset with isize

// Some appropriate operators
def_op!(I + O => I);
def_op!(I - O => I);
def_op!(I - I => O);
def_op!(I + usize => I);

#[test]
fn test_wrapper() {
    let i: I = I::from(0);
    let j: I = 10.into();
    let k: O = j - i;
    println!("{} {} {}", i, j, i + k);
    println!("arithmetic with index and offset: {}", i + k);
    println!("arithmetic with index and usize: {}", i + 12);
    println!("{}", i + k);
    for i in I::from(0)..I::from(10) {
        println!("i = {}", i);
    }
    for k in O::from(-10)..O::from(10) {
        println!("k = {}", k);
    }
    assert!(false);
}
