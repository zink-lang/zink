extern crate proc_macro;

use proc_macro2::{TokenStream, TokenTree};
use quote::quote;
use std::{cell::RefCell, collections::HashSet};
use syn::ItemStruct;

thread_local! {
   static STORAGE_REGISTRY: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Parse storage attribute.
///
/// Method `get` unwraps the ptr as the original type, mainly
/// mainly for passing the compilation checks at the moment,
/// and it works for WASM in real cases as well.
///
/// For the cases in EVM, it doesn't matter it returns pointer
/// since the value will be left on stack anyway.
pub fn parse(attr: TokenStream, input: ItemStruct) -> proc_macro::TokenStream {
    let tree: Vec<_> = attr.into_iter().collect();
    match tree.len() {
        1 => storage_value(input, tree[0].clone()),
        4 => storage_mapping(input, tree),
        _ => panic!("Invalid storage attributes"),
    }
    .into()
}

fn storage_value(is: ItemStruct, ty: TokenTree) -> TokenStream {
    let name = is.ident.clone();
    let slot = storage_slot(name.to_string());
    let expanded = quote! {
        #is

        impl zink::storage::Storage for #name {
            type Value = #ty;
            const STORAGE_SLOT: i32 = #slot;
        }
    };

    expanded
}

fn storage_mapping(is: ItemStruct, ty: Vec<TokenTree>) -> TokenStream {
    // TODO: better message for this panicking
    {
        let conv = ty[1].to_string() + &ty[2].to_string();
        if &conv != "=>" {
            panic!("Invalid mapping storage symbol");
        }
    }

    let key = &ty[0];
    let value = &ty[3];
    let name = is.ident.clone();
    let slot = storage_slot(name.to_string());
    let expanded = quote! {
        #is

        impl zink::storage::Mapping for #name {
            const STORAGE_SLOT: i32 = #slot;

            type Key = #key;
            type Value = #value;
        }
    };

    expanded
}

fn storage_slot(name: String) -> i32 {
    STORAGE_REGISTRY.with_borrow_mut(|r| {
        let key = r.len();
        if !r.insert(name.clone()) {
            panic!("Storage {name} has already been declared");
        }

        key
    }) as i32
}
