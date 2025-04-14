//! Fixed bytes
use crate::{asm, storage::Value};
use paste::paste;

macro_rules! impl_bytes {
    ($count:expr) => {
        paste! {
            #[derive(Debug, Clone, Copy, PartialEq, PartialOrd)]
            pub struct [<Bytes $count>] (
                #[allow(unused)]
                #[cfg(target_family = "wasm")] pub i32,
                #[cfg(not(target_family = "wasm"))] pub [u8; $count],
            );

            impl [<Bytes $count>] {
                /// Returns empty bytes
                #[cfg(target_family = "wasm")]
                pub const fn empty() -> Self {
                    [<Bytes $count>](0)
                }

                #[cfg(not(target_family = "wasm"))]
                pub const fn empty() -> Self {
                    [<Bytes $count>]([0; $count])
                }

                /// if self equal to another
                #[allow(clippy::should_implement_trait)]
                #[inline(always)]
                pub fn eq(self, other: Self) -> bool {
                    paste::paste! {
                        unsafe { asm::bytes::[< bytes $count _eq >](self, other) }
                    }
                }
            }

            impl Value for [<Bytes $count>] {
                fn tload() -> Self {
                    unsafe { asm::bytes::[<tload_bytes $count>]() }
                }

                fn sload() -> Self {
                    unsafe { asm::bytes::[<sload_bytes $count>]() }
                }

                fn push(self) {
                    unsafe { asm::bytes::[<push_bytes $count>](self) }
                }

                #[cfg(not(target_family = "wasm"))]
                fn bytes32(&self) -> [u8; 32] {
                    let mut output = [0; 32];
                    output[(32-$count)..].copy_from_slice(&self.0);
                    output
                }
            }
        }
    };
    ($($count:expr),+) => {
        $(impl_bytes!($count);)+
    };
}

impl_bytes! {
    2, 3, 4, 5, 6, 7, 8, 9, 10, 11, 12, 13, 14, 15, 16,
    17, 18, 19, 20, 21, 22, 23, 24, 25, 26, 27, 28, 29, 30, 31, 32
}
