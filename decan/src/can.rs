//! Implementation of *owned symbols* based on [`dlopen`](https://github.com/szymonwieloch/rust-dlopen).
//! 
//! Creating a [`Can`] associates a set of symbols with a library handle. The library handle is freed
//! simultaneously to the symbols.
//! 
//! While this is memory-safe, it creates a more permanent association between the library and its symbols.
//! If you only need to load symbols temporarily, you may want to use the [`borrow`][`crate::borrow`] module.

use std::{mem::ManuallyDrop, ops::Deref, path::Path};

use crate::{raw::{self, Library}, LibraryHandle, LoadOrSymbolGroupError, SymbolGroup, SymbolGroupError};

/// A library together with a set of symbols. The symbols do not outlive the library
/// because they die with it.
pub struct Can<H: LibraryHandle, G: SymbolGroup> {
    handle: H,
    symbols: G,
}

impl<H: LibraryHandle, G: SymbolGroup> Can<H, G> {
    /// Creates a can using an existing handle.
    /// # Safety
    /// The caller is responsible for ensuring that the symbols specified in `G`
    /// match exported library symbols with the correct type.
    pub unsafe fn with_handle(handle: H) -> Result<Self, SymbolGroupError> {
        let symbols = G::load(handle.as_raw())?;
        Ok(Self { handle, symbols })
    }
}

impl<G: SymbolGroup> Can<Library, G> {
    /// Loads a can from a specified path.
    /// # Safety
    /// The caller is responsible for ensuring that the symbols specified in `G`
    /// match exported library symbols with the correct type.
    pub unsafe fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadOrSymbolGroupError> {
        Self::with_handle(Library::load(path)?).map_err(Into::into)
    }
}

impl<G: SymbolGroup> Can<ManuallyDrop<Library>, G> {
    /// Wraps an existing library handle in a non-owning reference.
    /// # Safety
    /// The caller is responsible for ensuring that the symbols specified in `G`
    /// match exported library symbols with the correct type.
    pub unsafe fn wrap_raw(raw_handle: raw::Handle) -> Result<Self, SymbolGroupError> {
        Self::with_handle(Library::wrap_raw(raw_handle))
    }
}

impl<H: LibraryHandle, G: SymbolGroup> LibraryHandle for Can<H, G> {
    unsafe fn as_raw(&self) -> crate::raw::Handle {
        self.handle.as_raw()
    }
}

impl<H: LibraryHandle, G: SymbolGroup> Deref for Can<H, G> {
    type Target = G;

    /// Dereferences to the underlying symbol group.
    fn deref(&self) -> &Self::Target {
        &self.symbols
    }
}