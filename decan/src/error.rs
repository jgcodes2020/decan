use std::{any, ffi::NulError, io};

/// An error that occurs when loading a dynamic library.
#[derive(Debug, thiserror::Error)]
pub enum LoadError {
    /// An error occurred in the operating system API while loading the library.
    #[error("OS error ({0})")]
    Os(#[source] io::Error),
    /// An error occurred converting the path to a C string. This only occurs
    /// if the provided path contains null characters, which are invalid on most systems.
    #[error("Failed to create C string from path ({0})")]
    CStr(#[source] NulError),
}

impl From<io::Error> for LoadError {
    fn from(value: io::Error) -> Self {
        Self::Os(value)
    }
}

impl From<NulError> for LoadError {
    fn from(value: NulError) -> Self {
        Self::CStr(value)
    }
}

/// Error that occurs when loading a [`Symbol`][crate::Symbol].
#[derive(Debug, thiserror::Error)]
pub enum SymbolError {
    /// An error occurred in the OS's dynamic library loader.
    #[error("OS error ({0})")]
    Os(#[source] io::Error),
    /// A type expects a non-null value, but got a null value.
    #[error("Type {0} expects a non-null value")]
    NullValue(&'static str)
}

impl SymbolError {
    pub(crate) fn null_value<T: ?Sized>() -> Self {
        Self::NullValue(any::type_name::<T>())
    }

    /// Combines this symbol error 
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

/// Error that occurs when loading a [`SymbolGroup`][crate::SymbolGroup].
#[derive(Debug, thiserror::Error)]
#[error("Error loading `{name}`: {inner}")]
pub struct SymbolGroupError {
    name: Box<str>,
    #[source] inner: SymbolError
}

impl SymbolGroupError {
    /// The name of the symbol that failed to load.
    pub fn name(&self) -> &str {
        &self.name
    }

    /// The error that occurred when loading the symbol.
    pub fn inner(&self) -> &SymbolError {
        &self.inner
    }
}

/// Either a [`LoadError`] or a [`SymbolGroupError`]. 
/// 
/// For now, this only occurs when calling [`can::Can::load`][crate::can::Can::load], 
/// as this is the only call that both loads the library and symbols.
#[derive(Debug, thiserror::Error)]
pub enum LoadOrSymbolGroupError {
    /// An OS error occurred when loading the library.
    #[error("Library loading failed: {0}")]
    Library(#[source] LoadError),
    /// One of the symbols in the symbol group failed to load.
    #[error("Symbol loading failed: {0}")]
    Symbol(#[source] SymbolGroupError),
}

impl From<LoadError> for LoadOrSymbolGroupError {
    fn from(value: LoadError) -> Self {
        Self::Library(value)
    }
}
impl From<SymbolGroupError> for LoadOrSymbolGroupError {
    fn from(value: SymbolGroupError) -> Self {
        Self::Symbol(value)
    }
}