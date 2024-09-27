extern crate proc_macro;

use heck::AsSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Span, TokenTree};
use quote::quote;
use std::{cell::RefCell, collections::HashSet};
use syn::{
    meta::{self, ParseNestedMeta},
    parse::{Parse, ParseStream, Result},
    parse_quote, Attribute, Ident, ItemFn, ItemStruct, Visibility,
};

thread_local! {
   static STORAGE_REGISTRY: RefCell<HashSet<String>> = RefCell::new(HashSet::new());
}

/// Storage attributes parser
pub struct Storage {
    /// kind of the storage
    ty: StorageType,
    /// The source and the target storage struct
    target: ItemStruct,
    /// Getter function of storage
    getter: Option<Ident>,
}

impl Storage {
    /// Parse from proc_macro attribute
    pub fn parse(ty: StorageType, target: ItemStruct) -> TokenStream {
        let storage = Self::from((ty, target));
        storage.expand()
    }

    fn expand(mut self) -> TokenStream {
        match &self.ty {
            StorageType::Value(value) => self.expand_value(value.clone()),
            StorageType::Mapping { key, value } => self.expand_mapping(key.clone(), value.clone()),
            StorageType::DoubleKeyMapping { key1, key2, value } => {
                self.expand_dk_mapping(key1.clone(), key2.clone(), value.clone())
            }
            StorageType::Invalid => panic!("Invalid storage type"),
        }
    }

    fn expand_value(&mut self, value: Ident) -> TokenStream {
        let is = &self.target;
        let name = self.target.ident.clone();
        let slot = storage_slot(name.to_string());
        let mut expanded = quote! {
            #is

            impl zink::storage::Storage for #name {
                type Value = #value;
                const STORAGE_SLOT: i32 = #slot;
            }
        };

        if let Some(getter) = self.getter() {
            // TODO: generate docs from the stroage doc
            let gs: proc_macro2::TokenStream = parse_quote! {
                #[allow(missing_docs)]
                #[zink::external]
                pub fn #getter() -> #value {
                    #name::get()
                }
            };
            expanded.extend(gs);
        }

        expanded.into()
    }

    fn expand_mapping(&mut self, key: Ident, value: Ident) -> TokenStream {
        let is = &self.target;
        let name = self.target.ident.clone();
        let slot = storage_slot(name.to_string());
        let mut expanded = quote! {
            #is

            impl zink::storage::Mapping for #name {
                const STORAGE_SLOT: i32 = #slot;

                type Key = #key;
                type Value = #value;
            }
        };

        if let Some(getter) = self.getter() {
            // TODO: generate docs from the stroage doc
            let gs: proc_macro2::TokenStream = parse_quote! {
                #[allow(missing_docs)]
                #[zink::external]
                pub fn #getter(key: #key) -> #value {
                    #name::get(key)
                }
            };
            expanded.extend(gs);
        }

        expanded.into()
    }

    fn expand_dk_mapping(&self, key1: Ident, key2: Ident, value: Ident) -> TokenStream {
        let is = &self.target;
        let name = &self.target.ident;
        let slot = storage_slot(name.to_string());
        let expanded = quote! {
            #is

            impl zink::DoubleKeyMapping for #name {
                const STORAGE_SLOT: i32 = #slot;

                type Key1 = #key1;
                type Key2 = #key2;
                type Value = #value;
            }
        };

        expanded.into()
    }

    /// Get the getter of this storage
    fn getter(&mut self) -> Option<Ident> {
        let mut getter = if matches!(self.target.vis, Visibility::Public(_)) {
            let fname = Ident::new(
                &AsSnakeCase(self.target.ident.to_string()).to_string(),
                Span::call_site(),
            );
            Some(fname)
        } else {
            None
        };

        self.getter.take().or(getter)
    }
}

impl From<(StorageType, ItemStruct)> for Storage {
    fn from(patts: (StorageType, ItemStruct)) -> Self {
        let mut this = Self {
            ty: patts.0,
            target: patts.1,
            getter: None,
        };

        let mut attrs: Vec<Attribute> = Default::default();
        for attr in this.target.attrs.iter().cloned() {
            if !attr.path().is_ident("getter") {
                attrs.push(attr);
                continue;
            }

            let Ok(list) = attr.meta.require_list().clone() else {
                panic!("Invali getter arguments");
            };

            let Some(TokenTree::Ident(getter)) = list.tokens.clone().into_iter().nth(0) else {
                panic!("Invalid getter function name");
            };

            this.getter = Some(getter);
        }

        this.target.attrs = attrs;
        this
    }
}

/// Zink storage type parser
#[derive(Default, Debug)]
pub enum StorageType {
    /// Single value storage
    Value(Ident),
    /// Mapping storage
    Mapping { key: Ident, value: Ident },
    /// Double key mapping storage
    DoubleKeyMapping {
        key1: Ident,
        key2: Ident,
        value: Ident,
    },
    /// Invalid storage type
    #[default]
    Invalid,
}

impl From<TokenStream> for StorageType {
    fn from(input: TokenStream) -> Self {
        let tokens = input.to_string();
        let types: Vec<_> = tokens.split(',').collect();
        match types.len() {
            1 => StorageType::Value(Ident::new(types[0].trim(), Span::call_site())),
            2 => StorageType::Mapping {
                key: Ident::new(types[0].trim(), Span::call_site()),
                value: Ident::new(types[1].trim(), Span::call_site()),
            },
            3 => StorageType::DoubleKeyMapping {
                key1: Ident::new(types[0].trim(), Span::call_site()),
                key2: Ident::new(types[1].trim(), Span::call_site()),
                value: Ident::new(types[2].trim(), Span::call_site()),
            },
            _ => panic!("Invalid storage attributes"),
        }
    }
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
