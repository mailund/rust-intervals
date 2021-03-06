#![feature(step_trait)] // for generated range iteration

use idx_dsl::{def_index, idx_type, offset_type, seq_type};

#[seq_type]
type SA<u32>;

#[offset_type]
type Offset = isize;

#[idx_type(base_ops, offset = Offset)]
type XIdx = u32;

#[idx_type(offset = Offset, base_ops)]
type SaIdx = i32;

def_index!(SaIdx for SA);

#[test]
fn it_works() {
    let x = XIdx(42);
    assert!(x.0 == 42);
    let result = 2 + 2;
    assert_eq!(result, 4);

    let y = XIdx(13);
    let off = x - y;
    assert_eq!(off, Offset(42 - 13));
    let off2 = 2 * off;
    assert_eq!(off2, Offset(2 * (42 - 13)));

    for i in XIdx(0)..XIdx(10) {
        let XIdx(j) = i;
        println!("{} {}", i, j);
    }

    use idx_types::type_traits::IndexType;
    assert_eq!(SaIdx(0).index(10), 0);
    assert_eq!(SaIdx(-1).index(10), 9);

    let _x: SA = vec![1, 2, 3].into();

    // FIXME println!("{}", x[0]);
}
