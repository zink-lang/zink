//! Macro for the function selector.

// use once_cell::sync::Lazy;
// use parking_lot::Mutex;
use proc_macro::TokenStream;
use quote::{quote, ToTokens};
use syn::{FnArg, ItemFn, Signature};

use crate::utils::keccak;

// static mut SELECTOR: Lazy<Mutex<HashMap<u32, Signature>>> =
//     Lazy::new(|| Mutex::new(HashMap::new()));

/// Mark the function as external.
pub fn external(item: ItemFn) -> TokenStream {
    // let mut selector = unsafe { SELECTOR.lock() };
    let selector = parse_selector(&item.sig);
    println!("selector: {}", selector);

    quote! {
        #item
    }
    .into()
}

/// Hash function signature to EVM selector.
fn parse_selector(sig: &Signature) -> u32 {
    let args = sig.inputs.iter().map(|arg| match arg {
        FnArg::Typed(pat) => pat.ty.clone().into_token_stream().to_string(),
        _ => panic!(
            "Unsupported function argument: {:?}",
            arg.into_token_stream().to_string()
        ),
    });

    let mut input = sig.ident.to_string();
    input = input + "(" + &args.collect::<Vec<_>>().join(", ") + ")";
    println!("input: {}", input);

    let mut selector = [0u8; 4];
    let hash = keccak(input.as_bytes());
    selector.copy_from_slice(&hash[..4]);

    u32::from_le_bytes(selector)
}
