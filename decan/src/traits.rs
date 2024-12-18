use std::{ffi::CStr, mem, ptr::NonNull};

use crate::{raw, SymbolError, SymbolGroupError};

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
        raw::get_symbol(lib, name).map_or_else(|err| Err(Into::into(err)), |ptr| Ok(ptr as Self))
    }
}

impl<T: Sized + 'static> sealed::Sealed for *mut T {}
unsafe impl<T: Sized + 'static> Symbol for *mut T {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::get_symbol(lib, name).map_or_else(|err| Err(Into::into(err)), |ptr| Ok(ptr as Self))
    }
}

impl<T: Sized + 'static> sealed::Sealed for NonNull<T> {}
unsafe impl<T: Sized + 'static> Symbol for NonNull<T> {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::get_symbol(lib, name)
            .map_err(Into::into)
            .and_then(|ptr| NonNull::new(ptr as *mut T).ok_or(SymbolError::null_value::<Self>()))
    }
}

macro_rules! impl_symbol_fn {
    ($($types:ident),* $(,)?) => {
        impl<R: 'static, $($types: 'static),*>  sealed::Sealed for Option<extern "C" fn($($types),*) -> R>  {}
        unsafe impl<R: 'static, $($types: 'static),*> Symbol for Option<extern "C" fn($($types),*) -> R> {
            unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
                raw::get_symbol(lib, name).map_or_else(
                    |err| Err(err.into()),
                    |ptr| Ok(mem::transmute::<_, Self>(ptr)),
                )
            }
        }

        impl<R: 'static, $($types: 'static),*>  sealed::Sealed for extern "C" fn($($types),*) -> R  {}
        unsafe impl<R: 'static, $($types: 'static),*> Symbol for extern "C" fn($($types),*) -> R {
            unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
                raw::get_symbol(lib, name)
                    .map_err(|err| err.into())
                    .and_then(|ptr| {
                        mem::transmute::<_, Option<Self>>(ptr).ok_or(SymbolError::null_value::<Self>())
                    })
            }
        }
    };
}

impl_symbol_fn!();
impl_symbol_fn!(T1);
impl_symbol_fn!(T1, T2);
impl_symbol_fn!(T1, T2, T3);
impl_symbol_fn!(T1, T2, T3, T4);
impl_symbol_fn!(T1, T2, T3, T4, T5);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);

#[cfg(feature = "variadic_32")]
mod impl_loadable_fn_32 {
    use super::*;
    impl_symbol_fn!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26, T27
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26, T27, T28
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26, T27, T28, T29
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
    );
    impl_symbol_fn!(
        T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
        T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32
    );
}

/// A group of known symbols that can be loaded together. It is very unsafe to use this on
/// its own; you generally want to create a [`Can`][`crate::can::Can`] containing the data.
/// # Safety
/// All members of a `SymbolGroup` should be [`Symbol`]s, or other [`SymbolGroup`]s.
/// This ensures that they can be safely loaded, unloaded, and referenced without causing UB.
pub unsafe trait SymbolGroup: Sized + Default + 'static {
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