mod interval;
mod rmq;

use interval::*;

fn main() {
    let r1: Interval<u32> = int(0, 0);
    let r2: Interval<u32> = int(0, 1);
    let r3: Interval<u32> = int(0, 2);
    println!("{}, {}, {}", r1, r2, r3);
    for i in r3.range() {
        println!("{}", i);
    }
    let (i, j) = r3.indices();
    println!("{},{}", i, j);
    println!("{} {}", r2.contains(1), r3.contains(1));
}
