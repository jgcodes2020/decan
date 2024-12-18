use proc_macro::TokenStream;
use syn::parse_macro_input;

mod derive_symbol_group;

/// Implements [`decan::SymbolGroup`]. Example usage:
/// ```rust
/// use decan::SymbolGroup;
/// use std::ptr::NonNull;
/// 
/// #[derive(SymbolGroup)]
/// struct MyLibrary {
///     static_array: NonNull<u32>,
///     #[symbol = "special_function"]
///     function: extern "C" fn(u32) -> u32,
///     #[subgroup]
///     extra: Option<MyLibraryOptional>
/// }
/// 
/// #[derive(SymbolGroup)]
/// struct MyLibraryOptional {
///     static_array_2: NonNull<u32>
/// }
/// ```
/// ## Features
/// - Functions will be loaded by field name by default. To override this,
///   add `#[symbol = "entry_point"]` to specify an entry point.
/// - `SymbolGroup`s can contain other `SymbolGroup`s. Annotate these
///   members with `#[subgroup]` to inform the macro.
#[proc_macro_derive(SymbolGroup, attributes(symbol, subgroup))]
pub fn derive_symbol_group(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::DeriveInput);

    derive_symbol_group::generate(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
