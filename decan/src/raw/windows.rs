use std::{
    ffi::{c_void, CStr, OsStr},
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

use crate::LoadError;

pub type Handle = HMODULE;

pub unsafe fn load_library(path: &OsStr) -> Result<Handle, LoadError> {
    // collect to wchar_t
    let wstr: Vec<u16> = path.encode_wide().chain(iter::once(0)).collect();

    LoadLibraryW(PCWSTR::from_raw(wstr.as_ptr())).map_err(|err| to_io_error(err).into())
}

pub unsafe fn get_symbol(handle: Handle, symbol: &CStr) -> io::Result<*mut c_void> {
    let symbol = GetProcAddress(handle, PCSTR::from_raw(symbol.as_ptr() as *const u8));
    if symbol.is_none() {
        return Err(io::Error::last_os_error());
    }
    Ok(mem::transmute::<_, *mut c_void>(symbol))
}

pub fn free_library(handle: Handle) {
    unsafe { FreeLibrary(handle).expect("FreeLibrary failed!") }
}

fn to_io_error(err: WinError) -> io::Error {
    io::Error::from_raw_os_error(err.code().0)
}
