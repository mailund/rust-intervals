use idx_dsl::{idx_type, seq_type};

#[seq_type]
type SA = [XIdx];

#[seq_type]
type Str;

#[idx_type] // FIXME: make an offset type
type Offset = isize;

#[idx_type(base_ops, offset = Offset)]
type XIdx = u32;

#[idx_type(offset = Offset, base_ops)]
type SaIdx = u32;

#[idx_type(offset = Offset)]
type XX = u32;

#[idx_type(base_ops)]
type YY = u32;

#[test]
fn it_works() {
    let x = XIdx(42);
    assert!(x.0 == 42);
    let result = 2 + 2;
    assert_eq!(result, 4);

    let y = XIdx(13);
    let off = x - y;
    assert_eq!(off, Offset(42 - 13));
}
