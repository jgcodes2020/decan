use std::{ops::Deref, path::Path};

use crate::{raw::Library, LibraryHandle, LoadOrSymbolGroupError, SymbolGroup, SymbolGroupError};

/// A library together with a set of symbols. You could probably call it a DLL in a can.
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
        let handle = Library::load(path)?;
        let symbols = G::load(handle.as_raw())?;
        Ok(Self { handle, symbols })
    }
}

impl<H: LibraryHandle, G: SymbolGroup> LibraryHandle for Can<H, G> {
    unsafe fn as_raw(&self) -> crate::raw::Handle {
        self.handle.as_raw()
    }
}

impl<H: LibraryHandle, G: SymbolGroup> Deref for Can<H, G> {
    type Target = G;

    fn deref(&self) -> &Self::Target {
        &self.symbols
    }
}