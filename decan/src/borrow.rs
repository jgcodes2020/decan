//! Implementation of *borrowed symbols* based on [`libloading`](https://github.com/nagisa/rust_libloading).
//! 
//! Calling [`LibraryBorrowExt::borrow_symbol`] or [`LibraryBorrowExt::borrow_group`] creates a reference object
//! whose lifetime cannot exceed the library it was borrowed from.
//! 
//! While this is memory-safe, it prevents the library handle from being moved while there are symbols referencing it.
//! If you need to store a library and symbols in a structure, consider using a [`Can`][crate::can::Can] instead.

use std::{ffi::CStr, marker::PhantomData, ops::Deref};

use crate::{LibraryHandle, Symbol, SymbolError, SymbolGroup, SymbolGroupError};

/// Extension trait for borrowing symbols from a library handle.
pub trait LibraryBorrowExt: LibraryHandle {
    /// Borrows a symbol from a library.
    /// # Safety
    /// The caller is responsible for ensuring that the type `T`
    /// matches the exported library symbol for `name`.
    /// 
    /// The caller must also take care to avoid aliasing a single memory location
    /// through multiple mutable references, as this is considered UB in Rust.
    unsafe fn borrow_symbol<'a, T: Symbol>(
        &'a self,
        name: &CStr,
    ) -> Result<SymbolRef<'a, T>, SymbolError> {
        let data = T::load_from(self.as_raw(), name)?;
        Ok(SymbolRef {
            data,
            marker: PhantomData,
        })
    }

    /// Borrows a symbol group from a library.
    /// # Safety
    /// The caller is responsible for ensuring that the type `T`
    /// matches the exported library symbol for `name`.
    /// 
    /// The caller must also take care to avoid aliasing a single memory location
    /// through multiple mutable references, as this is considered UB in Rust.
    unsafe fn borrow_group<'a, T: SymbolGroup>(
        &'a self,
    ) -> Result<SymbolGroupRef<'a, T>, SymbolGroupError> {
        let group = T::load(self.as_raw())?;
        Ok(SymbolGroupRef {
            group,
            marker: PhantomData,
        })
    }
}

impl<T> LibraryBorrowExt for T where T: LibraryHandle {}

/// A borrowed reference to a symbol.
pub struct SymbolRef<'a, T: Symbol> {
    data: T,
    marker: PhantomData<&'a ()>,
}

impl<'a, T: Symbol> Deref for SymbolRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.data
    }
}

/// A borrowed reference to a symbol group.
pub struct SymbolGroupRef<'a, T: SymbolGroup> {
    group: T,
    marker: PhantomData<&'a ()>,
}

impl<'a, T: SymbolGroup> Deref for SymbolGroupRef<'a, T> {
    type Target = T;

    fn deref(&self) -> &Self::Target {
        &self.group
    }
}
