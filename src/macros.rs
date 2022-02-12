// helper macros
macro_rules! apply_macro {
    ($macro:ident [ $($args:tt),* ]) => {
        $( $macro!($args); )*
    };
}
macro_rules! apply_base_types {
    ($macro:ident) => {
        apply_macro!( $macro [ u8, i8, u16, i16, u32, i32, u64, i64, usize, isize ] );
    };
}

pub(crate) use apply_base_types;
pub(crate) use apply_macro;
