//! Unix implementation of dynamic library loading.
//!
//! This uses the POSIX `dlopen()`/`dlsym()`/`dlclose()` APIs.

use std::{
    ffi::{c_int, c_void, CStr, CString, OsStr},
    io,
    os::unix::ffi::OsStrExt,
    path::{Path, PathBuf},
    ptr::{null, null_mut, NonNull},
    sync::Mutex,
};

use libc::{dlclose, dlerror, dlsym, RTLD_GLOBAL, RTLD_LAZY};

use crate::LoadError;

use super::AddressInfo;

/// The Unix dynamic library handle, `*mut c_void`.
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

/// Loads a library from a path.
///
/// This is equivalent to:
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
            return Err(
                io::Error::new(io::ErrorKind::Other, msg.to_string_lossy().into_owned()).into(),
            );
        }

        Ok(handle)
    })
}

/// Gets a symbol from a path.
///
/// This is equivalent to:
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

/// Closes a library.
///
/// This is equivalent to:
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

pub unsafe fn get_address_info(ptr: *const c_void) -> Option<AddressInfo> {
    let mut info = libc::Dl_info {
        dli_fname: null(),
        dli_fbase: null_mut(),
        dli_sname: null(),
        dli_saddr: null_mut(),
    };
    if libc::dladdr(ptr, &mut info) != 0 {
        let lib_path =
            Path::new(OsStr::from_bytes(CStr::from_ptr(info.dli_fname).to_bytes())).to_owned();
        let lib_addr = NonNull::new(info.dli_fbase).unwrap();

        let sym_name = if !info.dli_sname.is_null() {
            Some(CStr::from_ptr(info.dli_sname).to_owned())
        } else {
            None
        };
        let sym_addr = NonNull::new(info.dli_saddr);

        Some(AddressInfo { lib_path, lib_addr, sym_name, sym_addr })
    } else {
        None
    }
}
