use proc_macro::{Span, TokenStream};
use quote::{quote, ToTokens};
use sha3::{Digest, Keccak256};
use syn::{
    parse_quote, spanned::Spanned, Data, DeriveInput, Error, Fields, LitByteStr, Result, Type,
    Variant,
};

/// Custom error type for better error handling
#[derive(Debug)]
enum EventError {
    NotEnum(Span),
    UnsupportedType(Span, String),
    TooManyFields(Span),
}

impl EventError {
    fn to_compile_error(&self) -> TokenStream {
        let error_msg = match self {
            Self::NotEnum(span) => {
                Error::new((*span).into(), "Event can only be derived for enums")
            }
            Self::UnsupportedType(span, ty) => {
                Error::new((*span).into(), format!("Unsupported type: {}", ty))
            }
            Self::TooManyFields(span) => {
                Error::new((*span).into(), "Too many fields for event logging")
            }
        };
        error_msg.to_compile_error().into()
    }
}

/// Expand the event interface with better error handling
pub fn parse(item: DeriveInput) -> TokenStream {
    match parse_impl(item) {
        Ok(token_stream) => token_stream,
        Err(err) => err.to_compile_error().into(),
    }
}

fn parse_impl(item: DeriveInput) -> Result<TokenStream> {
    let name = LitByteStr::new(item.ident.to_string().as_bytes(), Span::call_site().into());
    let ident = item.clone().ident;

    // Ensure we are working with an enum
    let event_enum = match &item.data {
        Data::Enum(data_enum) => data_enum,
        _ => {
            return Err(Error::new_spanned(
                &item,
                "Event can only be derived for enums",
            ))
        }
    };

    let enum_name = &item.ident;

    // Generate ABI signature with validation
    let abi_signature = generate_abi_signature(enum_name, &event_enum.variants)?;

    // Generate variant implementations with validation
    let variant_implementations = event_enum
        .variants
        .iter()
        .map(|variant| generate_variant_implementation(enum_name, variant))
        .collect::<Result<Vec<_>>>()?;

    // Generate the final implementation
    let expanded = quote! {
        impl zink::Event for #ident {
            const NAME: &'static [u8] = #name;

            pub fn abi_signature() -> String {
                #abi_signature
            }

            fn log0(&self) -> Result<(), zink::EventError> {
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log1(&self, topic: &[u8]) -> Result<(), zink::EventError> {
                if topic.len() != 32 {
                    return Err(zink::EventError::InvalidTopicLength);
                }
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log2(&self, topic1: &[u8], topic2: &[u8]) -> Result<(), zink::EventError> {
                if topic1.len() != 32 || topic2.len() != 32 {
                    return Err(zink::EventError::InvalidTopicLength);
                }
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log3(&self, topic1: &[u8], topic2: &[u8], topic3: &[u8]) -> Result<(), zink::EventError> {
                if topic1.len() != 32 || topic2.len() != 32 || topic3.len() != 32 {
                    return Err(zink::EventError::InvalidTopicLength);
                }
                match self {
                    #(#variant_implementations)*
                }
            }

            fn log4(
                &self,
                topic1: &[u8],
                topic2: &[u8],
                topic3: &[u8],
                topic4: &[u8]
            ) -> Result<(), zink::EventError> {
                if topic1.len() != 32 || topic2.len() != 32 ||
                   topic3.len() != 32 || topic4.len() != 32 {
                    return Err(zink::EventError::InvalidTopicLength);
                }
                match self {
                    #(#variant_implementations)*
                }
            }
        }
    };

    Ok(expanded.into())
}

/// Generate Variant Implementation with validation
fn generate_variant_implementation(
    enum_name: &syn::Ident,
    variant: &Variant,
) -> Result<proc_macro2::TokenStream> {
    let variant_name = &variant.ident;
    let span = variant.span();

    match &variant.fields {
        Fields::Named(fields) => {
            if fields.named.len() > 4 {
                return Err(Error::new(span, "Named event can have at most 4 fields"));
            }

            let field_names: Vec<_> = fields.named.iter().map(|f| &f.ident).collect();
            let field_types: Vec<_> = fields.named.iter().map(|f| &f.ty).collect();

            validate_types(&field_types)?;

            Ok(quote! {
                #enum_name::#variant_name { #(#field_names),* } => {
                    let topic = generate_topic_hash(stringify!(#variant_name));
                    let data: Vec<Vec<u8>> = vec![
                        #(encode_field(&#field_names)?),*
                    ];
                    let flattened_data = flatten_and_pad_data(&data)?;
                    zink::ffi::evm::log1(&topic, &flattened_data)
                        .map_err(|e| zink::EventError::LogError(e))
                }
            })
        }
        Fields::Unnamed(fields) => {
            if fields.unnamed.len() > 4 {
                return Err(Error::new(
                    variant.span(),
                    "Tuple event can have at most 4 fields",
                ));
            }

            // Use a consistent binding pattern for tuple variants with explicit ref patterns
            let field_count = fields.unnamed.len();
            let field_bindings = (0..field_count)
                .map(|i| quote::format_ident!("v{}", i))
                .collect::<Vec<_>>();
            let ref_patterns = field_bindings
                .iter()
                .map(|id| quote!(ref #id))
                .collect::<Vec<_>>();

            let field_types: Vec<_> = fields.unnamed.iter().map(|f| &f.ty).collect();
            validate_types(&field_types)?;

            Ok(quote! {
                #enum_name::#variant_name(#(#ref_patterns),*) => unsafe {
                    let topic = generate_topic_hash(stringify!(#variant_name));
                    let data = vec![
                        #(encode_field(#field_bindings)?),*
                    ];
                    zink::ffi::evm::log1(&topic, &data)
                        .map_err(|e| zink::EventError::LogError(e))
                }
            })
        }
        Fields::Unit => Ok(quote! {
            #enum_name::#variant_name => {
                let topic = generate_topic_hash(stringify!(#variant_name));
                zink::ffi::evm::log0(&topic)
                    .map_err(|e| zink::EventError::LogError(e))
            }
        }),
    }
}

/// Validate field types
fn validate_types(types: &[&Type]) -> Result<()> {
    for ty in types {
        if !is_supported_type(ty) {
            return Err(Error::new_spanned(
                ty,
                format!("Unsupported type for event field: {}", quote!(#ty)),
            ));
        }
    }
    Ok(())
}

/// Check if type is supported
fn is_supported_type(ty: &Type) -> bool {
    matches!(
        type_to_string(ty).as_str(),
        "u8" | "u16"
            | "u32"
            | "u64"
            | "u128"
            | "i8"
            | "i16"
            | "i32"
            | "i64"
            | "i128"
            | "bool"
            | "String"
            | "Vec<u8>"
            | "&str"
            | "&[u8]"
            | "[u8;32]"
            | "Address"
            | "U256"
    )
}

/// Generate ABI signature with validation
fn generate_abi_signature(
    enum_name: &syn::Ident,
    variants: &syn::punctuated::Punctuated<Variant, syn::Token![,]>,
) -> Result<proc_macro2::TokenStream> {
    let variant_signatures = variants
        .iter()
        .map(|variant| {
            let variant_name = &variant.ident;
            let params = match &variant.fields {
                Fields::Named(fields) => fields
                    .named
                    .iter()
                    .map(|f| validate_and_convert_type(&f.ty))
                    .collect::<Result<Vec<_>>>()?
                    .join(","),
                Fields::Unnamed(fields) => fields
                    .unnamed
                    .iter()
                    .map(|f| validate_and_convert_type(&f.ty))
                    .collect::<Result<Vec<_>>>()?
                    .join(","),
                Fields::Unit => String::new(),
            };

            Ok(format!("{}({})", variant_name, params))
        })
        .collect::<Result<Vec<_>>>()?;

    Ok(quote! {
        vec![
            #(#variant_signatures.to_string()),*
        ].join(";")
    })
}

/// Validate and convert type to ABI string
fn validate_and_convert_type(ty: &Type) -> Result<String> {
    let type_str = type_to_string(ty);
    match type_str.as_str() {
        "u8" | "u16" | "u32" | "u64" => Ok("uint".to_string()),
        "i8" | "i16" | "i32" | "i64" => Ok("int".to_string()),
        "bool" => Ok("bool".to_string()),
        "String" | "&str" => Ok("string".to_string()),
        "Vec<u8>" | "&[u8]" | "[u8;32]" => Ok("bytes".to_string()),
        "Address" => Ok("address".to_string()),
        "U256" => Ok("uint256".to_string()),
        _ => Err(Error::new_spanned(
            ty,
            format!("Unsupported type for ABI: {}", type_str),
        )),
    }
}

/// Helper function to convert type to string
fn type_to_string(ty: &Type) -> String {
    quote!(#ty).to_string().replace([' ', '&'], "")
}

/// Generate topic hash
fn generate_topic_hash(input: &str) -> [u8; 32] {
    Keccak256::digest(input.as_bytes()).into()
}

/// Generate data hash
fn generate_data_hash(data: &[Vec<u8>]) -> [u8; 32] {
    let flattened: Vec<u8> = data.concat();
    Keccak256::digest(&flattened).into()
}

/// Helper function to flatten and pad data
fn flatten_and_pad_data(data: &[Vec<u8>]) -> Result<Vec<u8>> {
    let mut result = Vec::new();
    for chunk in data {
        if chunk.len() > 32 {
            // return Err(zink::EventError::DataTooLong);
            panic!("Data too long");
        }
        let mut padded = vec![0u8; 32];
        padded[..chunk.len()].copy_from_slice(chunk);
        result.extend_from_slice(&padded);
    }
    Ok(result)
}
