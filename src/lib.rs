#![feature(step_trait)]

// Base wrapper mechanism and type system
mod wrapper;
use wrapper::*;

// Definining arithmetic operators on wrapped types
mod ops_macros;
use ops_macros::*;

// Code for treating wrapped objects with indexing
mod index;
use index::*;

// Code for wrapping sequences
mod sequences;
use sequences::*;

//mod range;
//mod rmq;

/// The DSL for specifying types and the legal operations
macro_rules! type_rules {
    ( $( $block:ident : { $($ops:tt)* })* ) => {
        // Dispatching the blocks
        $( $crate::type_rules!(@ $block { $($ops)* }); )*
    };
    (@sequences  {$($ops:tt)*}) => { $crate::new_seq_types!($($ops)*); };
    (@indices    {$($ops:tt)*}) => { $crate::new_index_types!($($ops)*); };
    (@operations {$($ops:tt)*}) => { $crate::def_ops!($($ops)*); };
}
pub(crate) use type_rules;
