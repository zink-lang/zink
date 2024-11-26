//! Event interface generation

use proc_macro::{Span, TokenStream};
use quote::quote;
use sha3::{Digest, Keccak256};
use syn::{Data, DeriveInput, Fields, LitByteStr, Variant};

/// Expand the event interface
pub fn parse(item: DeriveInput) -> TokenStream {
    let name = LitByteStr::new(item.ident.to_string().as_bytes(), Span::call_site().into());
    let ident = item.clone().ident;

    //ensure we are working with an enum
    let event_enum = match &item.data {
        Data::Enum(data_enum) => data_enum,
        _ => panic!("Event can only be derived for enums"),
    };

    let enum_name = &item.ident;

     // Generate ABI signature
     let abi_signature = generate_abi_signature(enum_name, &event_enum.variants);

    let variant_implementations = event_enum
        .variants
        .iter()
        .map(|variant| generate_variant_implementation(enum_name, variant))
        .collect::<Vec<_>>();

    let expanded = quote! {
        impl zink::Event for #ident {
            const NAME: &'static [u8] = #name;

            // ABI signature generation
            fn abi_signature() -> String {
                #abi_signature
            }

            // Logging methods
            fn log0(&self) {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log1(&self) {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log2(&self) {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log3(&self) {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log4(&self) {
                match self {
                    #(#variant_implementations)*
                }
            }
        }
    };

    expanded.into()
}

/// Generate implementation for a specific variant
fn generate_variant_implementation(
    enum_name: &syn::Ident,
    variant: &Variant,
) -> proc_macro2::TokenStream {
    let variant_name = &variant.ident;

    match &variant.fields {
        Fields::Named(fields) => {
            let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            quote! {
                #enum_name::#variant_name { #(#field_names),* } => {
                    let data = vec![
                        #(
                            // Basic serialization - can be expanded
                            format!("{:?}", #field_names).into_bytes()
                        ),*
                    ];

                    zink::sys::log2(
                        &generate_topic_hash(stringify!(#variant_name)),
                        &generate_data_hash(&data),
                        data
                    )
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_indices = 0..fields.unnamed.len();
            let field_indices_clone = field_indices.clone();
            quote! {
                #enum_name::#variant_name(#(field #field_indices_clone),*) => {
                    let data = vec![
                        #(
                            format!("{:?}", field #field_indices).into_bytes()
                        ),*
                    ];

                    zink::sys::log2(
                        &generate_topic_hash(stringify!(#variant_name)),
                        &generate_data_hash(&data),
                        data
                    )
                }
            }
        }
        Fields::Unit => {
            quote! {
                #enum_name::#variant_name => {
                    zink::sys::log0(&[])
                }
            }
        }
    }
}

///Generate abi signature
fn generate_abi_signature(
    enum_name: &syn::Ident, 
    variants: &syn::punctuated::Punctuated<Variant, syn::Token![,]>
) -> proc_macro2::TokenStream {
    let variant_signatures = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let params = match &variant.fields {
            Fields::Named(fields) => {
                fields.named.iter()
                    .map(|f| format!("{:?}", f.ty))
                    .collect::<Vec<_>>()
                    .join(",")
            },
            Fields::Unnamed(fields) => {
                fields.unnamed.iter()
                    .map(|f| format!("{:?}", f.ty))
                    .collect::<Vec<_>>()
                    .join(",")
            },
            Fields::Unit => String::new()
        };
        
        format!("{}({})", variant_name, params)
    }).collect::<Vec<_>>();

    quote! {
        vec![
            #(#variant_signatures.to_string()),*
        ].join(";")
    }
}

///Generate topic hash
fn generate_topic_hash(input: &str) -> [u8; 32] {
    Keccak256::digest(input.as_bytes()).into()
}

///Generate data hash
fn generate_data_hash(data: &[Vec<u8>]) -> [u8; 32] {
    let flattened: Vec<u8> = data.concat();
    Keccak256::digest(&flattened).into()
}
