//! Bytes based instructions

use crate::primitives::*;

macro_rules! impl_bytes {
    ($($count:expr),*) => {
        #[link(wasm_import_module = "bytes")]
        #[allow(improper_ctypes)]
        extern "C" {
            paste::paste! {
              $(
                #[doc = concat!("Push ", stringify!($count), " bytes to stack")]
                pub fn [< push_bytes $count >] (bytes: [< Bytes $count >]);

                #[doc = concat!("Load ", stringify!($count), " bytes from storage")]
                pub fn [< sload_bytes $count >] () -> [< Bytes $count >];

                #[doc = concat!("Check equal for bytes", stringify!($count))]
                pub fn [< bytes $count _eq >] (this: [< Bytes $count >], other: [< Bytes $count >]) -> bool;
              )*
            }
        }
    };
}

impl_bytes!(
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32
);
