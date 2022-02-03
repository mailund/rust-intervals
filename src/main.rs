mod index;
mod interval;
mod rmq;

use index::*;
use interval::*;

fn f(ij: &Interval) {
    use Cases2::*;
    match ij.cases2() {
        Empty => println!("Empty"),
        Range(Idx(i), Idx(j)) => println!("Non-empty [{},{})", i, j),
    }
}

fn g(ij: &Interval) {
    use Cases3::*;
    match ij.cases3() {
        Empty => println!("Empty"),
        Singleton(Idx(i)) => println!("Singleton [{}]", i),
        Range(Idx(i), Idx(j)) => println!("Non-empty [{},{})", i, j),
    }
}

fn main() {
    let r1: Interval = i(Idx(0), Idx(0));
    let r2: Interval = i(Idx(0), Idx(1));
    let r3: Interval = i(Idx(0), Idx(2));
    println!("{}, {}, {}", r1, r2, r3);
    println!("{}", r1.is_empty());
    println!("{}", r2.is_empty());
    println!("ITER");
    for Idx(i) in r3.iter() {
        println!("{}", i);
    }
    let (i, j) = r3.indices();
    println!("{},{}", i, j);
    println!("{} {}", r2.contains(Idx(1)), r3.contains(Idx(1)));
    f(&r1);
    f(&r2);
    f(&r3);
    g(&r1);
    g(&r2);
    g(&r3);
}
