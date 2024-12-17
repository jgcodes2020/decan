use std::{
    ffi::{c_int, c_void, CStr},
    io,
    sync::Mutex,
};

use libc::{dlclose, dlerror, dlsym, RTLD_GLOBAL, RTLD_LAZY};

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

pub unsafe fn load_library(path: &CStr) -> io::Result<Handle> {
    with_dlerror_lock(|| {
        let handle = libc::dlopen(path.as_ptr(), DEFAULT_FLAGS);
        if handle.is_null() {
            let msg = CStr::from_ptr(dlerror());
            return Err(io::Error::new(
                io::ErrorKind::Other,
                msg.to_string_lossy().into_owned(),
            ));
        }

        Ok(handle)
    })
}

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

pub fn free_library(handle: Handle) {
    if unsafe { dlclose(handle) } != 0 {
        panic!("dlclose() failed!");
    }
}
