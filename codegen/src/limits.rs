//! EVM stack abstraction.
//!
//! TODO: refactor this module with Result as outputs.
use std::ops::{Add, AddAssign};

/// EVM stack limit.
const STACK_LIMIT: u16 = 0x400;

/// EVM buffer limit
const BUFFER_LIMIT: u16 = 0x6000;

macro_rules! limit {
    ($name:ident, $limit:expr, $error:literal, $desc:literal) => {
        #[doc = concat!(" ", $desc)]
        #[derive(Debug, Default, Clone, Copy)]
        pub struct $name(pub(crate) u16);

        impl Add for $name {
            type Output = Self;

            fn add(self, other: Self) -> Self {
                let size = self.0 + other.0;
                if size > $limit {
                    panic!($error);
                }

                $name(self.0 + other.0)
            }
        }

        impl AddAssign for $name {
            fn add_assign(&mut self, other: $name) {
                *self = *self + other;
            }
        }

        impl From<u16> for $name {
            fn from(offset: u16) -> $name {
                $name(offset)
            }
        }

        impl From<$name> for u16 {
            fn from(name: $name) -> Self {
                name.0
            }
        }
    };
    ($(
        ($name:ident, $limit:expr, $error:literal, $desc:literal)
    ),+) => {
        $(limit!($name, $limit, $error, $desc);)*
    };
}

limit! {
    (StackOffset, STACK_LIMIT, "Stack overflow", "Stack offset"),
    (BufferOffset, BUFFER_LIMIT, "Buffer limit exceeded", "Buffer offset")
}
