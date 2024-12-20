//! Functions for working directly with raw library handles.

#[cfg(unix)]
pub mod unix;
#[cfg(windows)]
pub mod windows;

use std::{ffi::{c_void, CString}, mem::ManuallyDrop, path::{Path, PathBuf}, ptr::NonNull};

/// Alias to the current platform module.
#[cfg(unix)]
pub use unix as platform;
/// Alias to the current platform module.
#[cfg(windows)]
pub use windows as platform;

use crate::{LibraryHandle, LoadError};

/// The platform library handle. This maps to `void*` on Unix-likes and `HMODULE` on Windows.
pub type Handle = platform::Handle;

/// A struct containing info about a pointer address.
pub struct AddressInfo {
    pub lib_path: PathBuf,
    pub lib_addr: NonNull<c_void>,

    pub sym_name: Option<CString>,
    pub sym_addr: Option<NonNull<c_void>>,
}


/// An handle to an open library which frees it when dropped.
/// 
/// To prevent the handle from being dropped, use [`ManuallyDrop`].
pub struct Library(Handle);

impl Library {
    /// Loads a library from a path.
    pub fn load<P: AsRef<Path>>(path: P) -> Result<Self, LoadError> {
        Ok(Self(unsafe { platform::load_library(path.as_ref().canonicalize()?.as_os_str())? }))
    }

    /// Wraps a raw library handle in a non-owning reference.
    /// # Safety
    /// The caller is responsible for ensuring that the provided handle
    /// is valid for the lifetime of this object.
    pub unsafe fn wrap_raw(handle: Handle) -> ManuallyDrop<Self> {
        ManuallyDrop::new(Self(handle))
    }
}

unsafe impl Send for Library {}
unsafe impl Sync for Library {}

impl Drop for Library {
    fn drop(&mut self) {
        platform::free_library(self.0);
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