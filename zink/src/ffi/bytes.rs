//! Bytes-based instructions

use crate::primitives::*;

macro_rules! impl_bytes {
    ($($count:expr),*) => {
        #[link(wasm_import_module = "bytes")]
        #[allow(improper_ctypes)]
        extern "C" {
            #[cfg(target_arch = "wasm32")]
            paste::paste! {
                $(
                    #[doc = concat!("Push ", stringify!($count), " bytes to stack")]
                    pub fn [< push_bytes $count >] (bytes: [< Bytes $count >]);

                    #[doc = concat!("Load ", stringify!($count), " bytes from storage")]
                    pub fn [< sload_bytes $count >] () -> [< Bytes $count >];

                    #[doc = concat!("Load ", stringify!($count), " bytes from transient storage")]
                    pub fn [< tload_bytes $count >] () -> [< Bytes $count >];

                    #[doc = concat!("Check equality for Bytes", stringify!($count))]
                    pub fn [< bytes $count _eq >] (this: [< Bytes $count >], other: [< Bytes $count >]) -> bool;
                )*
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        paste::paste! {
            $(
                #[no_mangle]
                pub extern "C" fn [< push_bytes $count >](_bytes: [< Bytes $count >]) {
                    unimplemented!(concat!("push_bytes", stringify!($count), " not implemented for native targets"));
                }

                #[no_mangle]
                pub extern "C" fn [< sload_bytes $count >]() -> [< Bytes $count >] {
                    unimplemented!(concat!("sload_bytes", stringify!($count), " not implemented for native targets"));
                }

                #[no_mangle]
                pub extern "C" fn [< tload_bytes $count >]() -> [< Bytes $count >] {
                    unimplemented!(concat!("tload_bytes", stringify!($count), " not implemented for native targets"));
                }

                #[no_mangle]
                pub extern "C" fn [< bytes $count _eq >](this: [< Bytes $count >], other: [< Bytes $count >]) -> bool {
                    this == other // use PartialEq for native targets
                }
            )*
        }
    };
}

impl_bytes!(
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16, 17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27,
    28, 29, 30, 31, 32
);
