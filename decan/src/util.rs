//! Convenience utilities for writing FFI bindings.

use std::ptr::NonNull;

macro_rules! variadic_0_16 {
    ($macro:path) => {
        $macro!();
        $macro!(T1);
        $macro!(T1, T2);
        $macro!(T1, T2, T3);
        $macro!(T1, T2, T3, T4);
        $macro!(T1, T2, T3, T4, T5);
        $macro!(T1, T2, T3, T4, T5, T6);
        $macro!(T1, T2, T3, T4, T5, T6, T7);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15);
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16);
    };
}

#[allow(unused_macros)]
macro_rules! variadic_17_32 {
    ($macro:path) => {
        use super::*;
        $macro!(T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17);
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25, T26
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25, T26, T27
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25, T26, T27, T28
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25, T26, T27, T28, T29
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25, T26, T27, T28, T29, T30
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31
        );
        $macro!(
            T1, T2, T3, T4, T5, T6, T7, T8, T9, T10, T11, T12, T13, T14, T15, T16, T17, T18, T19, T20,
            T21, T22, T23, T24, T25, T26, T27, T28, T29, T30, T31, T32
        );
    };
}

#[allow(unused_imports)]
pub(crate) use {variadic_0_16, variadic_17_32};

/// Utility trait for obtaining a non-nullable version of an FFI type.
/// 
/// This is intended to be used in conjunction with [`bindgen`](https://github.com/rust-lang/rust-bindgen),
/// as `bindgen` generates optional types by default.
pub trait HasNonNull {
    /// The corresponding non-nullable type.
    type NonNull;
}

impl<T> HasNonNull for *mut T {
    type NonNull = NonNull<T>;
}

macro_rules! impl_has_non_null_fn {
    ($($types:ident),*) => {
        impl<R, $($types),*> HasNonNull for Option<extern "C" fn($($types),*) -> R> {
            type NonNull = extern "C" fn($($types),*) -> R;
        }
        impl<R, $($types),*> HasNonNull for Option<unsafe extern "C" fn($($types),*) -> R> {
            type NonNull = unsafe extern "C" fn($($types),*) -> R;
        }
    };
}

variadic_0_16!(impl_has_non_null_fn);
#[cfg(feature = "variadic_32")]
variadic_17_32!(impl_has_non_null_fn);

/// Convenience macro for using [`HasNonNull::NonNull`].
#[macro_export]
macro_rules! non_null {
    ($type:ty) => {
        <$type as ::decan::util::HasNonNull>::NonNull
    };
}