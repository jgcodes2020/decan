use std::{ffi::CStr, mem, ptr::NonNull};

use crate::{raw, util, SymbolError, SymbolGroupError};

mod sealed {
    pub trait Sealed {}
}

/// Trait for types that can be loaded from a dynamic library.
pub unsafe trait Symbol: sealed::Sealed + Sized + 'static {
    /// Loads a symbol with the given name from the specified library.
    /// # Safety
    /// The caller is responsible for ensuring:
    /// - the resulting pointer does not outlive the library owning it. 
    /// - the pointer's type matches that of the exported library symbol.
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError>;
}

impl<T: Sized + 'static> sealed::Sealed for *const T {}
unsafe impl<T: Sized + 'static> Symbol for *const T {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::platform::get_symbol(lib, name).map_or_else(|err| Err(Into::into(err)), |ptr| Ok(ptr as Self))
    }
}

impl<T: Sized + 'static> sealed::Sealed for *mut T {}
unsafe impl<T: Sized + 'static> Symbol for *mut T {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::platform::get_symbol(lib, name).map_or_else(|err| Err(Into::into(err)), |ptr| Ok(ptr as Self))
    }
}

impl<T: Sized + 'static> sealed::Sealed for NonNull<T> {}
unsafe impl<T: Sized + 'static> Symbol for NonNull<T> {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::platform::get_symbol(lib, name)
            .map_err(Into::into)
            .and_then(|ptr| NonNull::new(ptr as *mut T).ok_or(SymbolError::null_value::<Self>()))
    }
}

macro_rules! impl_symbol_fn {
    ($($types:ident),* $(,)?) => {
        impl<R: 'static, $($types: 'static),*>  sealed::Sealed for Option<extern "C" fn($($types),*) -> R>  {}
        unsafe impl<R: 'static, $($types: 'static),*> Symbol for Option<extern "C" fn($($types),*) -> R> {
            unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
                raw::platform::get_symbol(lib, name).map_or_else(
                    |err| Err(err.into()),
                    |ptr| Ok(mem::transmute::<_, Self>(ptr)),
                )
            }
        }

        impl<R: 'static, $($types: 'static),*>  sealed::Sealed for extern "C" fn($($types),*) -> R  {}
        unsafe impl<R: 'static, $($types: 'static),*> Symbol for extern "C" fn($($types),*) -> R {
            unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
                raw::platform::get_symbol(lib, name)
                    .map_err(|err| err.into())
                    .and_then(|ptr| {
                        mem::transmute::<_, Option<Self>>(ptr).ok_or(SymbolError::null_value::<Self>())
                    })
            }
        }

        impl<R: 'static, $($types: 'static),*>  sealed::Sealed for Option<unsafe extern "C" fn($($types),*) -> R>  {}
        unsafe impl<R: 'static, $($types: 'static),*> Symbol for Option<unsafe extern "C" fn($($types),*) -> R> {
            unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
                raw::platform::get_symbol(lib, name).map_or_else(
                    |err| Err(err.into()),
                    |ptr| Ok(mem::transmute::<_, Self>(ptr)),
                )
            }
        }

        impl<R: 'static, $($types: 'static),*>  sealed::Sealed for unsafe extern "C" fn($($types),*) -> R  {}
        unsafe impl<R: 'static, $($types: 'static),*> Symbol for unsafe extern "C" fn($($types),*) -> R {
            unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
                raw::platform::get_symbol(lib, name)
                    .map_err(|err| err.into())
                    .and_then(|ptr| {
                        mem::transmute::<_, Option<Self>>(ptr).ok_or(SymbolError::null_value::<Self>())
                    })
            }
        }
    };
}

util::variadic_0_16!(impl_symbol_fn);
#[cfg(feature = "variadic_32")]
util::variadic_17_32!(impl_symbol_fn);

/// A group of known symbols that can be loaded together.
/// 
/// Using `SymbolGroup`s directly makes it relatively easy for the symbols
/// to outlive the library handle. In general, symbol groups should either 
/// be [borrowed][crate::borrow::LibraryBorrowExt::borrow_group] or
/// [canned][crate::can::Can].
/// # Safety
/// All members of a `SymbolGroup` should be [`Symbol`]s, or other [`SymbolGroup`]s.
/// This ensures that they can be safely loaded, unloaded, and referenced without causing UB.
pub unsafe trait SymbolGroup: Sized + 'static {
    /// Loads the symbol group from the provided library handle.
    /// # Safety
    /// This function expects the type signatures provided by this `SymbolGroup` to
    /// match those of the exported library symbols they are loading.
    unsafe fn load(handle: raw::Handle) -> Result<Self, SymbolGroupError>;
}

unsafe impl<G: SymbolGroup> SymbolGroup for Option<G> {
    /// Loads the symbol group from the library handle. If it fails to load,
    /// it will simply return `None`.
    unsafe fn load(handle: raw::Handle) -> Result<Self, SymbolGroupError> {
        Ok(G::load(handle).ok())
    }
}

/// An object that contains a library handle. Library handles can have
/// symbols borrowed temporarily from them.
pub trait LibraryHandle {
    /// Obtains the raw library handle.
    /// # Safety
    /// The caller should not free the library while it is in use.
    unsafe fn as_raw(&self) -> raw::Handle;
}