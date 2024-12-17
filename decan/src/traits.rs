use std::{ffi::CStr, ptr::NonNull, mem};

use crate::{raw, SymbolError, SymbolGroupError};

/// Trait for types that can be loaded from a dynamic library.
pub unsafe trait Symbol: Sized + 'static {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError>;
}

unsafe impl<T: Sized + 'static> Symbol for *const T {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::get_symbol(lib, name).map_or_else(|err| Err(Into::into(err)), |ptr| Ok(ptr as Self))
    }
}

unsafe impl<T: Sized + 'static> Symbol for *mut T {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::get_symbol(lib, name).map_or_else(|err| Err(Into::into(err)), |ptr| Ok(ptr as Self))
    }
}

unsafe impl<T: Sized + 'static> Symbol for NonNull<T> {
    unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
        raw::get_symbol(lib, name)
            .map_err(Into::into)
            .and_then(|ptr| NonNull::new(ptr as *mut T).ok_or(SymbolError::null_value::<Self>()))
    }
}

macro_rules! impl_symbol_fn {
    ($($types:ident),* $(,)?) => {
        unsafe impl<R: 'static, $($types: 'static),*> Symbol for Option<extern "C" fn($($types),*) -> R> {
            unsafe fn load_from(lib: raw::Handle, name: &CStr) -> Result<Self, SymbolError> {
                raw::get_symbol(lib, name).map_or_else(
                    |err| Err(err.into()),
                    |ptr| Ok(mem::transmute::<_, Self>(ptr)),
                )
            }
        }

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

pub unsafe trait SymbolGroup: Sized + 'static {
    unsafe fn load(handle: raw::Handle) -> Result<Self, SymbolGroupError>;
}