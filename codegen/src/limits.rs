//! EVM stack abstraction.
//!
//! TODO: refactor this module with Result as outputs. (#21)
use std::ops::{Add, AddAssign};

/// EVM stack limit.
const STACK_LIMIT: u16 = 0x400;

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

        impl TryFrom<usize> for $name {
            type Error = &'static str;

            fn try_from(value: usize) -> Result<Self, Self::Error> {
                if value > $limit as usize {
                    Err($error)
                } else {
                    Ok($name(value as u16))
                }
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
    (StackOffset, STACK_LIMIT, "Stack overflow", "Stack offset")
}
