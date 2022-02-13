#![feature(step_trait)]

mod macros;

// Base wrapper mechanism and type system
mod wrapper;
#[allow(unused_imports)]
use wrapper::*;

// Definining arithmetic operators on wrapped types
mod ops;
#[allow(unused_imports)]
use ops::*;

// Code for treating wrapped objects with indexing
mod index;
#[allow(unused_imports)]
use index::*;

// Code for wrapping sequences
mod sequences;
#[allow(unused_imports)]
use sequences::*;

// Handling ranges of new types (with some rust-induced limits)
mod range;
#[allow(unused_imports)]
use range::*;

//mod rmq;
