#![warn(missing_docs)]
#![warn(rustdoc::missing_doc_code_examples)]
#![allow(clippy::excessive_precision)]

//! A Library for Double-Double and Quad-Double Arithmetic.

/// Module for double-double arithmetic.
pub mod f128;

/// Module for quad-double arithmetic.
pub mod f256;

/// Crate-wide utils.
pub mod utils;
