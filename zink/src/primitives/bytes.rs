//! Fixed bytes
use crate::{ffi, storage::StorageValue, Asm};
use paste::paste;

macro_rules! impl_bytes {
    ($count:expr) => {
        paste! {
            pub struct [<Bytes $count>] (
                #[cfg(target_family = "wasm")] i32,
                #[cfg(not(target_family = "wasm"))] pub [u8; $count],
            );

            impl [<Bytes $count>] {
                /// if self equal to another
                ///
                /// NOTE: not using core::cmp because it uses registers in wasm
                #[allow(clippy::should_implement_trait)]
                #[inline(always)]
                pub fn eq(self, _other: Self) -> bool {
                    todo!("unsafe {{ ffi::bytesn_eq(self, other) }}")
                }
            }

            impl Asm for [<Bytes $count>] {
                fn push(self) {
                    todo!("unsafe {{ ffi::push_bytesn(self, other) }}")
                }

                #[cfg(not(target_family = "wasm"))]
                fn bytes32(&self) -> [u8; 32] {
                    let mut output = [0; 32];
                    output[(32-$count)..].copy_from_slice(&self.0);
                    output
                }
            }

            impl StorageValue for [<Bytes $count>] {
                fn sload() -> Self {
                    todo!("unsafe {{ ffi::asm::bytesn() }}")
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
