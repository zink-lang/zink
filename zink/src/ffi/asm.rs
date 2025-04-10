//! Assembly FFI.

macro_rules! impl_push {
    ($($ty:ty),*) => {
        #[link(wasm_import_module = "asm")]
        #[allow(improper_ctypes)]
        extern "C" {
            #[cfg(target_arch = "wasm32")]
            paste::paste! {
                $(
                    #[doc = concat!("Push a ", stringify!($ty), " to the stack.")]
                    pub fn [< push_ $ty >](val: $ty);
                )*
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        paste::paste! {
            $(
                #[no_mangle]
                pub extern "C" fn [< push_ $ty >](_val: $ty) {
                    unimplemented!(concat!("push_", stringify!($ty), " only available in wasm32 target"));
                }
            )*
        }
    };
}

macro_rules! impl_mod_ops {
    ($($ty:ty),*) => {
        #[link(wasm_import_module = "asm")]
        #[allow(improper_ctypes)]
        extern "C" {
            #[cfg(target_arch = "wasm32")]
            paste::paste! {
                $(
                    /// Emit opcode ADDMOD
                    pub fn [< addmod_ $ty >](a: $ty, b: $ty, n: $ty) -> $ty;
                    /// Emit opcode MULMOD
                    pub fn [< mulmod_ $ty >](a: $ty, b: $ty, n: $ty) -> $ty;
                )*
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        paste::paste! {
            $(
                #[no_mangle]
                pub extern "C" fn [< addmod_ $ty >](_a: $ty, _b: $ty, _n: $ty) -> $ty {
                    unimplemented!(concat!("addmod_", stringify!($ty), " only available in wasm32 target"));
                }
                #[no_mangle]
                pub extern "C" fn [< mulmod_ $ty >](_a: $ty, _b: $ty, _n: $ty) -> $ty {
                    unimplemented!(concat!("mulmod_", stringify!($ty), " only available in wasm32 target"));
                }
            )*
        }
    };
}

macro_rules! impl_load {
    ($($ty:ty),*) => {
        #[link(wasm_import_module = "asm")]
        #[allow(improper_ctypes)]
        extern "C" {
            #[cfg(target_arch = "wasm32")]
            paste::paste! {
                $(
                    #[doc = concat!("Load a ", stringify!($ty), " from the storage.")]
                    pub fn [< sload_ $ty >]() -> $ty;
                    #[doc = concat!("Load a ", stringify!($ty), " from the transient storage.")]
                    pub fn [< tload_ $ty >]() -> $ty;
                )*
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        paste::paste! {
            $(
                #[no_mangle]
                pub extern "C" fn [< sload_ $ty >]() -> $ty {
                    unimplemented!(concat!("sload_", stringify!($ty), " only available in wasm32 target"));
                }
                #[no_mangle]
                pub extern "C" fn [< tload_ $ty >]() -> $ty {
                    unimplemented!(concat!("tload_", stringify!($ty), " only available in wasm32 target"));
                }
            )*
        }
    };
}

macro_rules! impl_tstore {
    ($($ty:ty),*) => {
        #[link(wasm_import_module = "asm")]
        #[allow(improper_ctypes)]
        extern "C" {
            #[cfg(target_arch = "wasm32")]
            paste::paste! {
                $(
                    #[doc = concat!("Store a ", stringify!($ty), " to the transient storage.")]
                    pub fn [< tstore_ $ty >](val: $ty);
                )*
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        paste::paste! {
            $(
                #[no_mangle]
                pub extern "C" fn [< tstore_ $ty >](_val: $ty) {
                    unimplemented!(concat!("tstore_", stringify!($ty), " only available in wasm32 target"));
                }
            )*
        }
    };
}

macro_rules! impl_revert {
    ($($num:expr),*) => {
        #[link(wasm_import_module = "asm")]
        #[allow(improper_ctypes)]
        extern "C" {
            #[cfg(target_arch = "wasm32")]
            paste::paste! {
                $(
                    #[doc = concat!("Revert with message in ", stringify!($num), " bytes")]
                    pub fn [< revert $num >](message: &'static str);
                )*
            }
        }

        #[cfg(not(target_arch = "wasm32"))]
        paste::paste! {
            $(
                #[no_mangle]
                pub fn [< revert $num >](message: &'static str) {
                    if $num == 1 {
                        panic!("Revert: {}", message);
                    } else {
                        panic!("Revert called");
                    }
                }
            )*
        }
    };
}

#[link(wasm_import_module = "asm")]
#[allow(improper_ctypes)]
extern "C" {
    #[cfg(target_arch = "wasm32")]
    /// Load a 32-bit signed integer from the storage.
    pub fn sload(slot: i32) -> i32;
    #[cfg(target_arch = "wasm32")]
    /// Store a 32-bit signed integer to the storage.
    pub fn sstore(slot: i32, value: i32);
    #[cfg(target_arch = "wasm32")]
    /// Load a 32-bit signed integer from the transient storage.
    pub fn tload(slot: i32) -> i32;
    #[cfg(target_arch = "wasm32")]
    /// Store a 32-bit signed integer to the transient storage.
    pub fn tstore(slot: i32, value: i32);
}

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub extern "C" fn sload(_slot: i32) -> i32 {
    unimplemented!("sload only available in wasm32 target");
}

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub extern "C" fn sstore(_slot: i32, _value: i32) {
    unimplemented!("sstore only available in wasm32 target");
}

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub extern "C" fn tload(_slot: i32) -> i32 {
    unimplemented!("tload only available in wasm32 target");
}

#[cfg(not(target_arch = "wasm32"))]
#[no_mangle]
pub extern "C" fn tstore(_slot: i32, _value: i32) {
    unimplemented!("tstore only available in wasm32 target");
}

impl_push!(i8, u8, i16, u16, i32, u32, i64, u64);
impl_mod_ops!(i8, u8, i16, u16, i32, u32, i64, u64);
impl_load!(i8, u8, i16, u16, i32, u32, i64, u64);
impl_tstore!(i8, u8, i16, u16, i32, u32, i64, u64);
impl_revert!(1, 2, 3, 4);
