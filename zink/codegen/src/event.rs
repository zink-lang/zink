//! Event interface generation
use proc_macro::{Span, TokenStream};
use quote::quote;
use sha3::{Digest, Keccak256};
use syn::{Data, DeriveInput, Fields, LitByteStr, Variant};

/// Expand the event interface
pub fn parse(item: DeriveInput) -> TokenStream {
    let name = LitByteStr::new(item.ident.to_string().as_bytes(), Span::call_site().into());
    let ident = item.clone().ident;

    // Ensure we are working with an enum
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

            fn log1(&self, topic: &'static [u8]) {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log2(&self, topic1: &'static [u8], topic2: &'static [u8]) {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log3(&self, topic1: &'static [u8], topic2: &'static [u8], topic3: &'static [u8]) {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log4(&self, topic1: &'static [u8], topic2: &'static [u8], topic3: &'static [u8], topic4: &'static [u8]) {
                match self {
                    #(#variant_implementations)*
                }
            }
        }
    };

    expanded.into()
}

/// Generate Variant Implementation
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
                    let topic = generate_topic_hash(stringify!(#variant_name));
                    let data = vec![
                        #(
                            format!("{:?}", #field_names).into_bytes()
                        ),*
                    ];
                    let flattened_data: Vec<u8> = data.concat();
                    zink::ffi::evm::log2(&topic, &generate_data_hash(&data), &flattened_data)
                }
            }
        }
        Fields::Unnamed(fields) => {
            let field_indices = 0..fields.unnamed.len();
            let field_indices_clone = field_indices.clone();
            quote! {
                #enum_name::#variant_name(#(field #field_indices_clone),*) => {
                    let topic = generate_topic_hash(stringify!(#variant_name));
                    let data = vec![
                        #(
                            format!("{:?}", field #field_indices).into_bytes()
                        ),*
                    ];
                    let flattened_data: Vec<u8> = data.concat();
                    zink::ffi::evm::log2(&topic, &generate_data_hash(&data), &flattened_data)
                }
            }
        }
        Fields::Unit => {
            quote! {
                #enum_name::#variant_name => {
                    zink::ffi::evm::log0(&generate_topic_hash(stringify!(#variant_name)))
                }
            }
        }
    }
}


/// Generate ABI signature
fn generate_abi_signature(
    enum_name: &syn::Ident, 
    variants: &syn::punctuated::Punctuated<Variant, syn::Token![,]>
) -> proc_macro2::TokenStream {
    let variant_signatures = variants.iter().map(|variant| {
        let variant_name = &variant.ident;
        let params = match &variant.fields {
            Fields::Named(fields) => {
                fields.named.iter()
                    .map(|f| type_to_string(&f.ty))
                    .collect::<Vec<_>>()
                    .join(",")
            },
            Fields::Unnamed(fields) => {
                fields.unnamed.iter()
                    .map(|f| type_to_string(&f.ty))
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

/// Generate topic hash
fn generate_topic_hash(input: &str) -> [u8; 32] {
    Keccak256::digest(input.as_bytes()).into()
}

/// Generate data hash
pub fn generate_data_hash(data: &[Vec<u8>]) -> [u8; 32] {
    let flattened: Vec<u8> = data.concat();
    Keccak256::digest(&flattened).into()
}

fn type_to_string(ty: &syn::Type) -> String {
    // Use quote to convert the type to a token stream, then to a string
    let type_tokens = quote! { #ty };
    type_tokens.to_string()
        .replace(' ', "")  
        .replace("&", "")  
}
