use idx_dsl::{seq_type, idx_type, def_ops};

seq_type!(
    SAIdx:  SA<XIdx>
    StrIdx: Str
);

idx_type!(
    type Foo<usize>
);

def_ops!(
    Foo - Foo => Bar,
    Foo + Bar => Foo,
    Bar + Foo => Foo,
    Foo += Bar
);


#[test]
fn it_works() {
    let x = Foo(42);
    assert!(x.0 == 42);
    let result = 2 + 2;
    assert_eq!(result, 4);
}
