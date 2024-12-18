# Decan

*[insert picture of half-open can]*

A no-nonsense dynamic library loading crate. 

Takes inspiration both from [libloading](https://github.com/nagisa/rust_libloading/)
and [dlopen](https://github.com/szymonwieloch/rust-dlopen).

## Example usage
```rs
use std::{path::{Path, PathBuf}, process::Command};

use decan::{can::Can, SymbolGroup};

#[derive(SymbolGroup)]
pub struct DecanTestlib {
    pub print_message: extern "C" fn(),
    pub square_int: Option<extern "C" fn(i32) -> i32>,
}

#[test]
fn main() {
    let can = unsafe { Can::<_, DecanTestlib>::load("libdecan_testlib.so").unwrap() };

    (can.print_message)();
    assert_eq!((can.square_int.unwrap())(2), 4);
}
```