use std::{any, ffi::CString, io};

/// Error that occurs when loading a [`Loadable`].
#[derive(Debug, thiserror::Error)]
pub enum SymbolError {
    /// An error occurred in the OS's dynamic library loader.
    #[error("OS error ({0})")]
    Os(#[source] io::Error),
    /// A type expects a non-null value, but got a null value.
    #[error("Symbols of type {0} cannot contain a null value")]
    NullValue(&'static str)
}

impl SymbolError {
    pub(crate) fn null_value<T: ?Sized>() -> Self {
        Self::NullValue(any::type_name::<T>())
    }

    pub fn in_group<S: Into<Box<str>>>(self, name: S) -> SymbolGroupError {
        SymbolGroupError {
            name: name.into(),
            inner: self
        }
    }
}

impl From<io::Error> for SymbolError {
    fn from(error: io::Error) -> Self {
        Self::Os(error)
    }
}

#[derive(Debug, thiserror::Error)]
#[error("Error loading `{name}`: {inner}")]
pub struct SymbolGroupError {
    name: Box<str>,
    #[source] inner: SymbolError
}

impl SymbolGroupError {
    pub fn name(&self) -> &str {
        &self.name
    }
}