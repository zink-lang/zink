//! Ethereum virtual machine opcode

mod shanghai;

pub use shanghai::ShangHai;

#[macro_export]
macro_rules! opcodes {
    ($name:ident, $desc:literal) => {
        #[doc = $desc]
        $name
    };
    {
        $version:ident,
        $((
            $opcode:expr,
            $name:ident,
            $gas:expr,
            $input:expr,
            $output:expr,
            $desc:literal,
            $since:ident,
            $group:ident
        )),+
    } => {
        /// Ethereum virtual machine opcode.
        #[derive(Clone, Copy, Debug)]
        pub enum $version {
            $(
                #[doc = concat!(" ", $desc)]
                $name,
            )*
        }

        impl From<u8> for $version {
            fn from(value: u8) -> Self {
                match value {
                    $(
                        $opcode => $version::$name,
                    )*
                    _ => unreachable!("Invalid opcode."),
                }
            }
        }

        impl Into<u8> for $version {
            fn into(self) -> u8 {
                match self {
                    $(
                        $version::$name => $opcode,
                    )*
                }
            }
        }

        impl OpCode for $version {
            fn since(&self) -> Upgrade {
                match self {
                    $(
                        $version::$name => Upgrade::$since,
                    )*
                }
            }

            fn group(&self) -> Group {
                match self {
                    $(
                        $version::$name => Group::$group,
                    )*
                }
            }

            fn gas(&self) -> u16 {
                match self {
                    $(
                        $version::$name => $gas,
                    )*
                }
            }
        }
    };
}

/// EVM opcode groups
pub enum Group {
    /// Stop and Arithmetic Operations
    StopArithmetic,
    /// Comparison & Bitwise Logic Operations
    ComparisonBitwiseLogic,
    /// SHA3
    Sha3,
    /// Environmental Information
    EnvironmentalInformation,
    /// Block Information
    BlockInformation,
    /// Stack, Memory, Storage and Flow Operations
    StackMemoryStorageFlow,
    /// Push Operations
    Push,
    /// Duplication Operations
    Duplication,
    /// Exchange Operations
    Exchange,
    /// Logging Operations
    Logging,
    /// System operations
    System,
}

/// Ethereum upgrades.
pub enum Upgrade {
    /// Frontier
    Frontier,
    /// Byzantium
    Byzantium,
    /// Constantinople
    Constantinople,
    /// Istanbul
    Istanbul,
    /// Berlin
    Berlin,
    /// London
    London,
    /// Shanghai
    Shanghai,
}

/// Ethereum virtual machine opcode.
pub trait OpCode: From<u8> + Into<u8> {
    fn since(&self) -> Upgrade;
    fn group(&self) -> Group;
    fn gas(&self) -> u16;
}