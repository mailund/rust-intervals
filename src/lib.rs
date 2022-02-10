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

//mod index_macros;

//mod range;
//mod rmq;
