#![feature(step_trait)]
mod index;
mod range;
mod rmq;

use index::*;
use range::*;

fn f(range: &Range) {
    use Cases2::*;
    match range.cases2() {
        E => println!("Empty"),
        R(Idx(i), Idx(j)) => println!("Non-empty [{},{})", i, j),
    }
}

fn g(range: &Range) {
    use Cases3::*;
    match range.cases3() {
        E => println!("Empty"),
        S(Idx(i)) => println!("Singleton [{}]", i),
        R(Idx(i), Idx(j)) => println!("Non-empty [{},{})", i, j),
    }
}

fn main() {
    let r1 = range(Idx(0), Idx(0));
    let r2 = range(Idx(0), Idx(1));
    let r3 = range(Idx(0), Idx(2));
    println!("{}", r1.is_empty());
    println!("{}", r2.is_empty());
    println!("RANGE");
    for Idx(i) in Idx(0)..Idx(4) {
        println!("{}", i)
    }
    for Idx(i) in r3.iter() {
        println!("{}", i);
    }
    if let Cases2::R(i, j) = r3.cases2() {
        println!("{},{}", i, j);
    }
    println!("{} {}", r2.contains(&Idx(1)), r3.contains(&Idx(1)));
    f(&r1);
    f(&r2);
    f(&r3);
    g(&r1);
    g(&r2);
    g(&r3);

    let v = vec![1, 2, 3, 4, 3, 1, 1];
    println!("{}", v[Idx(0)]);
    let w = &v[1..3];
    println!("{}", w[Idx(0)]);

    for Idx(i) in Idx(0)..Idx(5) {
        println!("{}", i);
    }

    // FIXME: I want this
    /*
    let (i, j) = (Idx(1), Idx(3));
    let w = &v[Idx(1)..Idx(3)];
    for k in w {
        println!("{}", k);
    }
    */
    // but I can only get this
    let (i, j) = (Idx(1), Idx(3));
    let w = &v[range(i, j)];
    for k in w {
        println!("{}", k);
    }
}
