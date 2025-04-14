//! FFI related macros

use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::ToTokens;
use syn::{
    parse_macro_input, parse_quote, punctuated::Punctuated, DeriveInput, ExprBlock, FnArg,
    ItemForeignMod,
};

/// Apply safe block for ffi interfaces
///
/// Given the source,
///
/// ```no_run
/// #[impl_safe]
/// extern "C" {
///   fn add(a: i32, b: i32) -> i32;
/// }
/// ```
///
/// The macro will generate the following code,
///
/// ```no_run
/// pub mod ffi {
///   extern "C" {
///     fn add(a: i32, b: i32) -> i32;
///   }
/// }
///
/// pub fn add(_a: i32, _b: i32) -> i32;
/// ```
pub fn impl_safe(input: TokenStream) -> TokenStream {
    let input = parse_macro_input!(input as ItemForeignMod);

    // Keep the original extern block
    let original = input.clone().into_token_stream();
    let safe_exports = generate_safe_exports(&input);
    let native_exports = generate_native_exports(&input);

    // Combine the original extern block with the safe_exports module
    let result = quote::quote! {
        #[cfg(target_arch = "wasm32")]
        mod ffi {
            use super::*;

            #original
        }

        #safe_exports

        #native_exports
    };

    result.into()
}

/// Generate the safe_exports module with safe wrappers for each FFI function
fn generate_safe_exports(extern_mod: &ItemForeignMod) -> proc_macro2::TokenStream {
    // Extract functions from the extern block
    let functions = extern_mod.items.iter().filter_map(|item| {
        if let syn::ForeignItem::Fn(func) = item {
            Some(func)
        } else {
            None
        }
    });

    // Generate safe wrappers for each function
    let safe_wrappers = functions.map(|func| {
        let name = &func.sig.ident;
        let inputs = &func.sig.inputs;
        let output = &func.sig.output;

        // Extract the original documentation attributes
        let doc_attrs = func.attrs.iter().filter(|attr| attr.path().is_ident("doc"));

        // Extract parameter names for the function call
        let param_names = func.sig.inputs.iter().filter_map(|param| {
            if let syn::FnArg::Typed(pat_type) = param {
                if let syn::Pat::Ident(pat_ident) = &*pat_type.pat {
                    return Some(&pat_ident.ident);
                }
            }
            None
        });

        // Generate the safe wrapper function with original docs
        quote::quote! {
            #[cfg(target_arch = "wasm32")]
            #(#doc_attrs)*
            pub fn #name(#inputs) #output {
                unsafe { ffi::#name(#(#param_names),*) }
            }
        }
    });

    // Generate the safe_exports module
    quote::quote! {
        #(#safe_wrappers)*
    }
}

/// Generate the safe_exports module with safe wrappers for each FFI function
fn generate_native_exports(extern_mod: &ItemForeignMod) -> proc_macro2::TokenStream {
    // Extract functions from the extern block
    let functions = extern_mod.items.iter().filter_map(|item| {
        if let syn::ForeignItem::Fn(func) = item {
            Some(func)
        } else {
            None
        }
    });

    // Generate safe wrappers for each function
    let native_wrappers = functions.map(|func| {
        let name = &func.sig.ident;
        let output = &func.sig.output;
        let inputs = func
            .sig
            .inputs
            .clone()
            .into_iter()
            .filter_map(|arg| {
                if let syn::FnArg::Typed(mut pat_type) = arg {
                    if let syn::Pat::Ident(pat_ident) = &mut *pat_type.pat {
                        let ident = format!("_{}", pat_ident.ident);
                        pat_ident.ident = syn::Ident::new(&ident, pat_ident.ident.span());
                        return Some(syn::FnArg::Typed(pat_type));
                    }
                }
                None
            })
            .collect::<Punctuated<syn::FnArg, syn::Token![,]>>();

        // Extract the original documentation attributes
        let doc_attrs = func.attrs.iter().filter(|attr| attr.path().is_ident("doc"));

        // Generate the safe wrapper function with original docs
        quote::quote! {
            #(#doc_attrs)*
            #[inline(always)]
            pub fn #name(#inputs) #output {
                unimplemented!("Only available in wasm32 target");
            }
        }
    });

    // Generate the safe_exports module
    quote::quote! {
        /// Native exports for the FFI functions
        #[cfg(not(target_arch = "wasm32"))]
        pub mod native_exports {
            use super::*;

            #(#native_wrappers)*
        }

        #[cfg(not(target_arch = "wasm32"))]
        pub use native_exports::*;
    }
}
