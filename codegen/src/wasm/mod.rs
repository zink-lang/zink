//! WASM related primitives.

macro_rules! impl_deref {
    ($doc:literal, $name:ident, $target:ty) => {
        #[derive(Clone, Debug, Default)]
        #[doc = concat!(" ", $doc)]
        pub struct $name($target);

        impl core::ops::Deref for $name {
            type Target = $target;

            fn deref(&self) -> &Self::Target {
                &self.0
            }
        }

        impl core::ops::DerefMut for $name {
            fn deref_mut(&mut self) -> &mut Self::Target {
                &mut self.0
            }
        }
    };
    ($(($doc:literal, $name:ident, $target:ty)),*) => {
        $( impl_deref!($doc, $name, $target); )*
    };
}

mod abi;
mod data;
mod func;
mod host;
mod ie;

pub use self::{
    abi::{ToLSBytes, Type},
    data::Data,
    func::{Function, Functions},
    host::HostFunc,
    ie::{Exports, Imports},
};
