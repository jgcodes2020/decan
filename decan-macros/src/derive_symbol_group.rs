use std::{ffi::CString, iter, sync::LazyLock};

use proc_macro::Ident;
use proc_macro2::{Literal, TokenStream};
use quote::{quote, quote_spanned};
use syn::spanned::Spanned;

pub(crate) fn generate(input: syn::DeriveInput) -> syn::Result<TokenStream> {
    if !input.generics.params.is_empty() {
        return Err(syn::Error::new_spanned(
            &input.generics,
            "derive(SymbolGroup) cannot be used with generics",
        ));
    }
    
    let (ident, fields) = match &input.data {
        syn::Data::Struct(syn::DataStruct {
            fields: syn::Fields::Named(fields),
            ..
        }) => (&input.ident, &fields.named),
        syn::Data::Struct(data_struct) => {
            return Err(syn::Error::new_spanned(
                &data_struct.fields,
                "derive(SymbolGroup) cannot be implemented for tuple or unit structs",
            ));
        }
        syn::Data::Enum(data_enum) => {
            return Err(syn::Error::new_spanned(
                &data_enum.enum_token,
                "derive(SymbolGroup) cannot be implemented for enums",
            ))
        }
        syn::Data::Union(data_union) => {
            return Err(syn::Error::new_spanned(
                &data_union.union_token,
                "derive(SymbolGroup) cannot be implemented for unions",
            ))
        }
    };

    let fields_info: syn::Result<Vec<FieldInfo>> =
        fields.iter().map(extract_field_info).collect();
    let fields_info = fields_info?;

    let fields_gen: Vec<TokenStream> = iter::zip(fields.iter(), fields_info.iter()).map(generate_field).collect();

    Ok(quote! {
        #[automatically_derived]
        unsafe impl ::decan::SymbolGroup for #ident {
            unsafe fn load(handle: ::decan::raw::Handle) -> Result<Self, ::decan::SymbolGroupError> {
                Ok(Self {
                    #(#fields_gen),*
                })
            }
        }
    })
}

fn generate_field((field, field_info): (&syn::Field, &FieldInfo)) -> TokenStream {
    let ident = field.ident.as_ref().unwrap();
    let ty = &field.ty;
    let span = field.span();
    match field_info {
        FieldInfo::Symbol { str_token, cstr_token } => {
            quote_spanned! { span=>
                #ident: <#ty as ::decan::Symbol>::load_from(handle, #cstr_token)
                    .map_err(|err| err.in_group(#str_token))?
            }
        },
        FieldInfo::Subgroup => {
            quote_spanned! { span=>
                #ident: <#ty as ::decan::SymbolGroup>::load(handle)?
            }
        },
    }
}

enum FieldInfo {
    Symbol {
        str_token: Literal,
        cstr_token: Literal,
    },
    Subgroup,
}

fn extract_field_info(field: &syn::Field) -> syn::Result<FieldInfo> {
    let mut current_info: Option<FieldInfo> = None;
    for attr in &field.attrs {
        if attr.path().is_ident("symbol") {
            if current_info.is_some() {
                return Err(syn::Error::new_spanned(
                    attr.path(),
                    "Only one of #[symbol] and #[subgroup] can be applied to a member",
                ));
            }
            let kv_pair = attr.meta.require_name_value()?;
            let name_lit_str = match &kv_pair.value {
                syn::Expr::Lit(syn::ExprLit {
                    lit: syn::Lit::Str(lit_str),
                    ..
                }) => lit_str,
                expr => {
                    return Err(syn::Error::new_spanned(
                        expr, "\
                        #[symbol] expects the format: #[symbol = \"symbol_name\"] where \"symbol_name\" \
                        is a valid C string",
                    ))
                }
            };
            let name_cstr = CString::new(name_lit_str.value()).map_err(|err| {
                let pos = err.nul_position();
                syn::Error::new_spanned(
                    &name_lit_str,
                    format!(
                        "\
                        unexpected null char in #[symbol] name at position {}\n\
                        hint: symbol name must be a valid C string and cannot contain null characters",
                        pos
                    ),
                )
            })?;
            let name_lit_cstr = Literal::c_string(&name_cstr);
            current_info = Some(FieldInfo::Symbol {
                str_token: name_lit_str.token(),
                cstr_token: name_lit_cstr,
            });
        } else if attr.path().is_ident("subgroup") {
            if current_info.is_some() {
                return Err(syn::Error::new_spanned(
                    attr.path(),
                    "Only one of #[symbol] and #[subgroup] can be applied to a member",
                ));
            }
            attr.meta.require_path_only()?;
            current_info = Some(FieldInfo::Subgroup);
        }
    }
    let info = current_info.unwrap_or_else(|| {
        let name_str = field.ident.as_ref().unwrap().to_string();
        let name_cstr = CString::new(name_str.clone()).expect(
            "valid identifiers shouldn't contain null characters",
        );
        FieldInfo::Symbol { str_token: Literal::string(&name_str), cstr_token: Literal::c_string(&name_cstr) }
    });
    Ok(info)
}
