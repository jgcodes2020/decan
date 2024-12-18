//! A no-nonsense dynamic library loading crate.
//! 
//! The primary issue with dynamic library loaders is that they return pointers,
//! which can potentially outlive the library that created them. This crate provides
//! two different mechanisms for borrowing symbols, each with their own benefits and caveats.
//! 
//! This crate is inspired by [libloading](https://github.com/nagisa/rust_libloading) and 
//! [dlopen](https://github.com/szymonwieloch/rust-dlopen).

mod error;
mod traits;

pub mod raw;
pub mod borrow;
pub mod can;
pub mod util;

pub use error::*;
pub use traits::*;

pub use decan_macros::SymbolGroup;