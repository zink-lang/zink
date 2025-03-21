//! Derive macro for contract storage
use proc_macro::TokenStream;
use proc_macro2::Span;
use quote::{format_ident, quote};
use syn::{parse_macro_input, Fields, Ident, ItemStruct, Type};

// Represents the contract storage derivation
pub struct ContractStorage {
    target: ItemStruct,
}

impl ContractStorage {
    /// Create a new ContractStorage from an input struct
    pub fn new(input: ItemStruct) -> Self {
        Self { target: input }
    }

    /// Parse and validate the input, returning a TokenStream
    pub fn parse(input: TokenStream) -> TokenStream {
        let input = parse_macro_input!(input as ItemStruct);
        let storage = Self::new(input);
        storage.expand()
    }

    /// Generate the expanded TokenStream
    fn expand(&self) -> TokenStream {
        let Fields::Named(fields) = &self.target.fields else {
            return syn::Error::new(
                Span::call_site(),
                "Storage derive only supports structs with named fields",
            )
            .to_compile_error()
            .into();
        };

        let struct_name = &self.target.ident;
        let generics = &self.target.generics;
        let (impl_generics, ty_generics, where_clause) = generics.split_for_impl();

        let mut slot_counter = 0;
        let field_structs: Vec<_> = fields.named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_ty = &field.ty;
            let slot = slot_counter;
            slot_counter += 1;
            let struct_name = format_ident!("{}{}", struct_name, field_name.to_upper_camel_case());

            match classify_field_type(field_ty) {
                FieldType::Simple => {
                    quote! {
                        pub struct #struct_name;
                        impl #impl_generics zink::storage::Storage for #struct_name #ty_generics #where_clause {
                            const STORAGE_SLOT: i32 = #slot;
                            type Value = #field_ty;

                            #[cfg(not(target_family = "wasm"))]
                            const STORAGE_KEY: [u8; 32] = [0u8; 32];

                            fn get() -> Self::Value {
                                zink::Asm::push(Self::STORAGE_SLOT);
                                <Self::Value as zink::storage::StorageValue>::sload()
                            }

                            fn set(value: Self::Value) {
                                value.push();
                                zink::Asm::push(Self::STORAGE_SLOT);
                                unsafe { zink::ffi::evm::sstore(); }
                            }
                        }
                    }
                }
                FieldType::Mapping => {
                    let (key_ty, value_ty) = extract_mapping_types(field_ty).unwrap_or_else(|| {
                        panic!("Mapping type must be of form Mapping<K, V>");
                    });
                    quote! {
                        pub struct #struct_name;
                        impl #impl_generics zink::storage::Mapping for #struct_name #ty_generics #where_clause {
                            const STORAGE_SLOT: i32 = #slot;
                            type Key = #key_ty;
                            type Value = #value_ty;

                            #[cfg(not(target_family = "wasm"))]
                            fn storage_key(key: Self::Key) -> [u8; 32] {
                                [0u8; 32]
                            }

                            fn get(key: Self::Key) -> Self::Value {
                                zink::storage::mapping::load_key(key, Self::STORAGE_SLOT);
                                <Self::Value as zink::storage::StorageValue>::sload()
                            }

                            fn set(key: Self::Key, value: Self::Value) {
                                value.push();
                                zink::storage::mapping::load_key(key, Self::STORAGE_SLOT);
                                unsafe { zink::ffi::evm::sstore(); }
                            }
                        }
                    }
                }
                FieldType::DoubleKeyMapping => {
                    let (key1_ty, key2_ty, value_ty) = extract_double_key_mapping_types(field_ty).unwrap_or_else(|| {
                        panic!("DoubleKeyMapping type must be of form DoubleKeyMapping<K1, K2, V>");
                    });
                    quote! {
                        pub struct #struct_name;
                        impl #impl_generics zink::storage::DoubleKeyMapping for #struct_name #ty_generics #where_clause {
                            const STORAGE_SLOT: i32 = #slot;
                            type Key1 = #key1_ty;
                            type Key2 = #key2_ty;
                            type Value = #value_ty;

                            #[cfg(not(target_family = "wasm"))]
                            fn storage_key(key1: Self::Key1, key2: Self::Key2) -> [u8; 32] {
                                [0u8; 32]
                            }

                            fn get(key1: Self::Key1, key2: Self::Key2) -> Self::Value {
                                zink::storage::dkmapping::load_double_key(key1, key2, Self::STORAGE_SLOT);
                                <Self::Value as zink::storage::StorageValue>::sload()
                            }

                            fn set(key1: Self::Key1, key2: Self::Key2, value: Self::Value) {
                                value.push();
                                zink::storage::dkmapping::load_double_key(key1, key2, Self::STORAGE_SLOT);
                                unsafe { zink::ffi::evm::sstore(); }
                            }
                        }
                    }
                }
                FieldType::Unknown => {
                    syn::Error::new_spanned(field_ty, "Unsupported storage type").to_compile_error()
                }
            }
        }).collect();

        let method_impls: Vec<_> = fields.named.iter().map(|field| {
            let field_name = field.ident.as_ref().unwrap();
            let field_ty = &field.ty;
            let setter_name = format_ident!("set_{}", field_name);
            let field_struct = format_ident!("{}{}", struct_name, field_name.to_upper_camel_case());

            match classify_field_type(field_ty) {
                FieldType::Simple => {
                    quote! {
                        pub fn #field_name(&self) -> #field_ty {
                            #field_struct::get()
                        }

                        pub fn #setter_name(&self, value: #field_ty) {
                            #field_struct::set(value);
                        }
                    }
                }
                FieldType::Mapping => {
                    let (key_ty, value_ty) = extract_mapping_types(field_ty).unwrap();
                    quote! {
                        pub fn #field_name(&self, key: #key_ty) -> #value_ty {
                            #field_struct::get(key)
                        }

                        pub fn #setter_name(&self, key: #key_ty, value: #value_ty) {
                            #field_struct::set(key, value);
                        }
                    }
                }
                FieldType::DoubleKeyMapping => {
                    let (key1_ty, key2_ty, value_ty) = extract_double_key_mapping_types(field_ty).unwrap();
                    quote! {
                        pub fn #field_name(&self, key1: #key1_ty, key2: #key2_ty) -> #value_ty {
                            #field_struct::get(key1, key2)
                        }

                        pub fn #setter_name(&self, key1: #key1_ty, key2: #key2_ty, value: #value_ty) {
                            #field_struct::set(key1, key2, value);
                        }
                    }
                }
                FieldType::Unknown => {
                    syn::Error::new_spanned(field_ty, "Unsupported storage type").to_compile_error()
                }
            }
        }).collect();

        let expanded = quote! {
            use zink::Asm;
            #(#field_structs)*
            impl #impl_generics #struct_name #ty_generics #where_clause {
                #(#method_impls)*
            }
        };

        TokenStream::from(expanded)
    }
}

trait ToUpperCamelCase {
    fn to_upper_camel_case(&self) -> String;
}

impl ToUpperCamelCase for Ident {
    fn to_upper_camel_case(&self) -> String {
        let s = self.to_string();
        let mut result = String::new();
        let mut capitalize_next = true;

        for c in s.chars() {
            if c == '_' {
                capitalize_next = true;
            } else if capitalize_next {
                result.push(c.to_ascii_uppercase());
                capitalize_next = false;
            } else {
                result.push(c);
            }
        }
        result
    }
}

enum FieldType {
    Simple,
    Mapping,
    DoubleKeyMapping,
    Unknown,
}

fn classify_field_type(ty: &Type) -> FieldType {
    if let Type::Path(type_path) = ty {
        let path = &type_path.path;
        if let Some(segment) = path.segments.last() {
            match segment.ident.to_string().as_str() {
                "Mapping" => FieldType::Mapping,
                "DoubleKeyMapping" => FieldType::DoubleKeyMapping,
                _ => FieldType::Simple,
            }
        } else {
            FieldType::Unknown
        }
    } else {
        FieldType::Unknown
    }
}

/// Extract generic types from Mapping<K, V>
fn extract_mapping_types(ty: &Type) -> Option<(Type, Type)> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "Mapping" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    let args: Vec<_> = args.args.iter().collect();
                    if args.len() == 2 {
                        if let (
                            syn::GenericArgument::Type(key_ty),
                            syn::GenericArgument::Type(value_ty),
                        ) = (&args[0], &args[1])
                        {
                            return Some((key_ty.clone(), value_ty.clone()));
                        }
                    }
                }
            }
        }
    }
    None
}

/// Extract generic types from DoubleKeyMapping<K1, K2, V>
fn extract_double_key_mapping_types(ty: &Type) -> Option<(Type, Type, Type)> {
    if let Type::Path(type_path) = ty {
        if let Some(segment) = type_path.path.segments.last() {
            if segment.ident == "DoubleKeyMapping" {
                if let syn::PathArguments::AngleBracketed(args) = &segment.arguments {
                    let args: Vec<_> = args.args.iter().collect();
                    if args.len() == 3 {
                        if let (
                            syn::GenericArgument::Type(key1_ty),
                            syn::GenericArgument::Type(key2_ty),
                            syn::GenericArgument::Type(value_ty),
                        ) = (&args[0], &args[1], &args[2])
                        {
                            return Some((key1_ty.clone(), key2_ty.clone(), value_ty.clone()));
                        }
                    }
                }
            }
        }
    }
    None
}
