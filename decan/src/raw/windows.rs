use std::{
    ffi::{c_void, CStr, NulError, OsStr},
    io, iter, mem,
    os::windows::ffi::OsStrExt,
};

use windows::{
    core::{Error as WinError, PCSTR, PCWSTR},
    Win32::{
        Foundation::{FreeLibrary, HMODULE},
        System::LibraryLoader::{GetProcAddress, LoadLibraryW},
    },
};

use crate::{LoadError, WideNulError};

/// The Windows dynamic library handle, `HMODULE`.
pub type Handle = HMODULE;

/// Loads a library from a path. 
/// 
/// This is equivalent to:
/// ```c
/// #include <windows.h>
/// LoadLibraryW(path);
/// ```
/// with additional error checking.
pub unsafe fn load_library(path: &OsStr) -> Result<Handle, LoadError> {
    // collect to wchar_t
    let wstr: Vec<u16> = path.encode_wide().collect();
    if let Some(pos) = wstr.iter().position(|&b| b == 0) {
        return Err(WideNulError(pos, wstr).into());
    }
    LoadLibraryW(PCWSTR::from_raw(wstr.as_ptr())).map_err(|err| to_io_error(err).into())
}

/// Gets a symbol from a path. 
/// 
/// This is equivalent to:
/// ```c
/// #include <windows.h>
/// (void*) GetProcAddress(handle, symbol);
/// ```
/// with additional error checking.
pub unsafe fn get_symbol(handle: Handle, symbol: &CStr) -> io::Result<*mut c_void> {
    let symbol = GetProcAddress(handle, PCSTR::from_raw(symbol.as_ptr() as *const u8));
    if symbol.is_none() {
        return Err(io::Error::last_os_error());
    }
    Ok(mem::transmute::<_, *mut c_void>(symbol))
}

/// Closes a library. 
/// 
/// This is equivalent to:
/// ```c
/// #include <windows.h>
/// FreeLibrary(handle);
/// ```
/// with additional error checking.
pub fn free_library(handle: Handle) {
    unsafe { FreeLibrary(handle).expect("FreeLibrary failed!") }
}

fn to_io_error(err: WinError) -> io::Error {
    io::Error::from_raw_os_error(err.code().0)
}

pub unsafe fn get_address_info(ptr: *const c_void) -> Option<AddressInfo> {
    None
}
