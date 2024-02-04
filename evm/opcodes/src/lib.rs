//! Ethereum virtual machine opcode
#![deny(missing_docs)]

mod shanghai;

pub use shanghai::ShangHai;

/// Ethereum virtual machine opcode generator.
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
        #[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
        pub enum $version {
            #[cfg(feature = "data")]
            /// No operation but provides a byte for serializing.
            Data(u8),
            $(
                #[doc = concat!(" ", $desc)]
                $name,
            )*
        }

        impl From<u8> for $version {
            fn from(value: u8) -> Self {
                match value {
                    $(
                        $opcode => Self::$name,
                    )*
                    _ => unreachable!("Invalid opcode."),
                }
            }
        }

        impl From<$version> for u8 {
            fn from(version: $version) -> Self {
                match version {
                    #[cfg(feature = "data")]
                    $version::Data(data) => data,
                    $(
                        $version::$name => $opcode,
                    )*
                }
            }
        }

        impl OpCode for $version {
            fn group(&self) -> Group {
                match self {
                    #[cfg(feature = "data")]
                    Self::Data(_) => Group::StopArithmetic,
                    $(
                        Self::$name => Group::$group,
                    )*
                }
            }

            fn gas(&self) -> u16 {
                match self {
                    #[cfg(feature = "data")]
                    Self::Data(_) => 0,
                    $(
                        Self::$name => $gas,
                    )*
                }
            }

            fn since(&self) -> Upgrade {
                match self {
                    #[cfg(feature = "data")]
                    Self::Data(_) => Upgrade::Shanghai,
                    $(
                        Self::$name => Upgrade::$since,
                    )*
                }
            }

            fn stack_in(&self) -> u16 {
                match self {
                    #[cfg(feature = "data")]
                    Self::Data(_) => 0,
                    $(
                        Self::$name => $input,
                    )*
                }
            }

            fn stack_out(&self) -> u16 {
                match self {
                    #[cfg(feature = "data")]
                    Self::Data(_) => 0,
                    $(
                        Self::$name => $output,
                    )*
                }
            }
        }

        impl core::str::FromStr for $version {
            type Err = ();

            fn from_str(s: &str) -> Result<Self, Self::Err> {
                paste::paste! {
                    match s {
                        $(
                            stringify!([< $name:lower >]) => Ok(Self::$name),
                        )*
                            _ => Err(()),
                    }
                }
            }
        }

        paste::paste! {
            #[doc = concat!(" For each ", stringify!($version), " operator.")]
            #[macro_export]
            macro_rules! [<for_each_ $version:lower _operator>] {
                ($mac:ident) => {
                    $mac! {
                        $([<_ $name:lower>] => $name),+
                    }
                }
            }
        }
    };
}

/// EVM opcode groups
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
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
#[derive(Clone, Copy, Debug, PartialOrd, Ord, PartialEq, Eq)]
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
    /// The stack input count.
    fn stack_in(&self) -> u16;

    /// The stack output count.
    fn stack_out(&self) -> u16;

    /// The OpCode is available since.
    fn since(&self) -> Upgrade;

    /// The group of the OpCode.
    fn group(&self) -> Group;

    /// The basic gas cost of the OpCode.
    fn gas(&self) -> u16;
}
