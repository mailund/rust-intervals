use idx_dsl::{def_ops, idx_type, seq_type};

#[seq_type]
type SA = [XIdx];

#[seq_type]
type Str;

#[idx_type]
type Offset = u32; // FIXME

#[idx_type]
type XIdx = u32;

#[idx_type]
type SaIdx = u32;

def_ops! {
    XIdx - XIdx => Offset,
    XIdx + Offset => XIdx,
    Offset + XIdx => XIdx,
    XIdx += Offset,
    XIdx -= Offset
}

#[test]
fn it_works() {
    let x = XIdx(42);
    assert!(x.0 == 42);
    let result = 2 + 2;
    assert_eq!(result, 4);
}
