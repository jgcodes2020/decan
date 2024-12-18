#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

use std::{mem::ManuallyDrop, path::Path};

#[cfg(unix)]
use unix as platform;
#[cfg(windows)]
use windows as platform;

pub use platform::{Handle, load_library, get_symbol, free_library};

use crate::{LibraryHandle, LoadError};

/// An owning handle to an open library.
pub struct Library(Handle);

impl Library {
    /// Loads a library from a path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
        Ok(Self(unsafe { load_library(path.as_ref().canonicalize()?.as_os_str())? }))
    }

    /// Wraps a raw library handle in a non-owning reference.
    /// # Safety
    /// The caller is responsible for ensuring that the provided handle
    /// is valid for the lifetime of this object.
    pub unsafe fn wrap_raw(handle: Handle) -> ManuallyDrop<Self> {
        ManuallyDrop::new(Self(handle))
    }
}

impl Drop for Library {
    fn drop(&mut self) {
        free_library(self.0);
    }
}

impl LibraryHandle for Library {
    unsafe fn as_raw(&self) -> self::Handle {
        self.0
    }
}

impl LibraryHandle for ManuallyDrop<Library> {
    unsafe fn as_raw(&self) -> self::Handle {
        self.0
    }
}