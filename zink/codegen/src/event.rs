use heck::ToSnakeCase;
use proc_macro::TokenStream;
use proc_macro2::{Ident, Span};
use quote::{quote, ToTokens};
use syn::{
    parse::Parse, parse_quote, punctuated::Punctuated, spanned::Spanned, Arm, Data, DataEnum,
    DeriveInput, Expr, ExprMatch, Fields, FnArg, ImplItemFn, ItemFn, LitByteStr, Result, Token,
    Type, Variant, Visibility,
};

/// Expand the event interface with better error handling
pub fn parse(item: DeriveInput) -> TokenStream {
    let name = &item.ident;
    let name_str = name.to_string();
    let name_bytes = LitByteStr::new(name_str.as_bytes(), Span::call_site());

    // 1. Check if the name is too long
    if name_str.len() > 32 {
        panic!("Event name too long: {name_str}");
    }

    // 2. Ensure we are working with an enum
    let Data::Enum(event_enum) = &item.data else {
        panic!("Event can only be derived for enums");
    };

    // 3. Generate variant implementations
    let mut expr_match: ExprMatch = parse_quote!(match self {});
    let variant_fns = event_enum
        .variants
        .iter()
        .map(|variant| impl_variant_fns(variant, &mut expr_match))
        .collect::<Vec<_>>();

    // 4. Generate the impl block
    quote! {
        impl #name {
            /// Name of the event
            pub const fn name() -> &'static [u8] {
                #name_bytes
            }

            /// Emit the event name
            pub fn emit_name() {
                unsafe { zink::ffi::evm::log0(Self::name()) }
            }

            #(#variant_fns)*

            /// Emit the event
            pub fn emit(self) {
                #expr_match
            }
        }
    }
    .into()
}

/// Generate Variant Implementation with validation
fn impl_variant_fns(variant: &Variant, expr_match: &mut ExprMatch) -> ImplItemFn {
    let name = &variant.ident;
    let topic = variant.fields.len();

    // Parse function inputs
    let mut inputs: Punctuated<FnArg, Token![,]> = Punctuated::new();
    let mut args: Vec<Ident> = Vec::new();
    for (index, field) in variant.fields.iter().enumerate() {
        let var = field
            .ident
            .clone()
            .unwrap_or(Ident::new(&format!("param_{index}"), Span::call_site()));
        let ty = &field.ty;

        args.push(var.clone());
        inputs.push(FnArg::Typed(parse_quote!(#var: #ty)));
    }

    // Generate the snake case name
    let name_snake: Ident = Ident::new(&name.to_string().to_snake_case(), Span::call_site());

    // Generate the match arm
    let arm: Arm = parse_quote! {
        Self::#name( #(#args),* ) => Self::#name_snake( #(#args),* ),
    };
    expr_match.arms.push(arm);

    // Generate the impl block
    let logn = Ident::new(&format!("log{topic}"), Span::call_site());
    let args = args
        .iter()
        .map(|arg| quote!(#arg.bytes32()))
        .collect::<Vec<_>>();
    parse_quote! {
        pub fn #name_snake(#inputs) {
            unsafe {zink::ffi::evm::#logn(#(#args),*, &Self::name()) }
        }
    }
}
