#![feature(step_trait)]

// Base wrapper mechanism and type system
mod wrapper;
#[allow(unused_imports)]
use wrapper::*;

// Definining arithmetic operators on wrapped types
mod ops_macros;
#[allow(unused_imports)]
use ops_macros::*;

// Code for treating wrapped objects with indexing
mod index;
#[allow(unused_imports)]
use index::*;

// Code for wrapping sequences
mod sequences;
#[allow(unused_imports)]
use sequences::*;

mod range;
#[allow(unused_imports)]
use range::*;

//mod rmq;

/// The DSL for specifying types and the legal operations
#[allow(unused_macros)]
macro_rules! type_rules {
    ( $( $block:ident : { $($ops:tt)* })* ) => {
        // Dispatching the blocks
        $( $crate::type_rules!(@ $block { $($ops)* }); )*
    };
    (@sequences  {$($ops:tt)*}) => { $crate::new_seq_types!($($ops)*); };
    (@indices    {$($ops:tt)*}) => { $crate::new_index_types!($($ops)*); };
    (@operations {$($ops:tt)*}) => { $crate::def_ops!($($ops)*); };
}
#[allow(unused_macros)]
#[allow(unused_imports)]
pub(crate) use type_rules;
