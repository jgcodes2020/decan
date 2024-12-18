use std::{
    ffi::{c_int, c_void, CStr, CString, OsStr}, io, os::unix::ffi::OsStrExt, sync::Mutex
};

use libc::{dlclose, dlerror, dlsym, RTLD_GLOBAL, RTLD_LAZY};

use crate::LoadError;

pub type Handle = *mut c_void;

#[cfg(not(any(target_os = "linux", target_os = "macos")))]
fn with_dlerror_lock<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    static DL_MUTEX: Mutex<()> = Mutex::new(());

    let _lock = DL_MUTEX.lock().unwrap();
    f()
}

#[cfg(any(target_os = "linux", target_os = "macos"))]
fn with_dlerror_lock<T, F>(f: F) -> T
where
    F: FnOnce() -> T,
{
    f()
}

const DEFAULT_FLAGS: c_int = RTLD_GLOBAL | RTLD_LAZY;

/// Loads a library from a path. This is equivalent to:
/// ```c
/// #include <dlfcn.h>
/// dlopen(path, RTLD_GLOBAL | RTLD_LAZY);
/// ```
/// with additional error checking.
pub unsafe fn load_library(path: &OsStr) -> Result<*mut c_void, LoadError> {
    let path = CString::new(path.as_bytes())?;

    with_dlerror_lock(|| {
        let handle = libc::dlopen(path.as_ptr(), DEFAULT_FLAGS);
        if handle.is_null() {
            let msg = CStr::from_ptr(dlerror());
            return Err(io::Error::new(
                io::ErrorKind::Other,
                msg.to_string_lossy().into_owned(),
            ).into());
        }

        Ok(handle)
    })
}

/// Gets a symbol from a path. This is equivalent to:
/// ```c
/// #include <dlfcn.h>
/// dlsym(handle, symbol);
/// ```
/// with additional error checking.
pub unsafe fn get_symbol(handle: Handle, symbol: &CStr) -> io::Result<*mut c_void> {
    with_dlerror_lock(|| {
        let _ = dlerror();
        let symbol = dlsym(handle, symbol.as_ptr());

        if symbol.is_null() {
            let msg = dlerror();
            if !msg.is_null() {
                return Err(io::Error::new(
                    io::ErrorKind::Other,
                    CStr::from_ptr(msg).to_string_lossy().into_owned(),
                ));
            }
        }

        Ok(symbol)
    })
}

/// Closes a library. This is equivalent to:
/// ```c
/// #include <dlfcn.h>
/// dlclose(handle);
/// ```
/// with additional error checking.
pub fn free_library(handle: Handle) {
    if unsafe { dlclose(handle) } != 0 {
        panic!("dlclose() failed!");
    }
}
