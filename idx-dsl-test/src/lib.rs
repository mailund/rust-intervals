use idx_dsl::{seq_type, idx_type};

seq_type!(
    type<T> Foo<T>
);

idx_type!(
    type Foo[usize]
    Foo - Foo => Bar
);


#[test]
fn it_works() {
    let result = 2 + 2;
    assert_eq!(result, 4);
}
