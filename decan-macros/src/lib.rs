use proc_macro::TokenStream;
use syn::parse_macro_input;

mod derive_symbol_group;

#[proc_macro_derive(SymbolGroup, attributes(symbol, subgroup))]
pub fn derive_symbol_group(item: TokenStream) -> TokenStream {
    let item = parse_macro_input!(item as syn::DeriveInput);

    derive_symbol_group::generate(item)
        .unwrap_or_else(syn::Error::into_compile_error)
        .into()
}
