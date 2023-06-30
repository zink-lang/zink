//! Ethereum virtual machine opcode

/// Ethereum virtual machine opcode.
#[derive(Clone, Copy, Debug)]
pub enum OpCode {
    // 0x0 range - Stop and Arithmetic Operations
    //
    /// Halts execution.
    STOP,
    /// Addition operation.
    ADD,
    /// Multiplication operation.
    MUL,
    /// Subtraction operation.
    SUB,
    /// Integer division operation.
    DIV,
    /// Signed integer division operation (truncated).
    SDIV,
    /// Modulo remainder operation.
    MOD,
    /// Signed modulo remainder operation.
    SMOD,
    /// Modulo addition operation.
    ADDMOD,
    /// Modulo multiplication operation.
    MULMOD,
    /// Exponential operation.
    EXP,
    /// Extend length of signed integer.
    SIGNEXTEND,
    // 0x10 range - Comparison & Bitwise Logic Operations
    //
    /// Less-than comparision.
    LT,
    /// Greater-than comparision.
    GT,
    /// Signed less-than comparision.
    SLT,
    /// Signed greater-than comparision.
    SGT,
    /// Equality comparision.
    EQ,
    /// Simple not operator.
    ISZERO,
    /// Bitwise AND operation.
    AND,
    /// Bitwise OR operation.
    OR,
    /// Bitwise XOR operation.
    XOR,
    /// Bitwise NOT operation.
    NOT,
    /// Retrieve single byte from word.
    BYTE,
    /// Shift left operation.
    SHL,
    /// Logical shift right operation.
    SHR,
    /// Arithmetic shift right operation.
    SAR,
    // 0x20 range - SHA3
    //
    /// Compute Keccak-256 hash.
    SHA3,
    // 0x30 range - Environmental Information
    //
    /// Get address of currently executing account.
    ADDRESS,
    /// Get balance of the given account.
    BALANCE,
    /// Get execution origination address.
    ORIGIN,
    /// Get caller address.
    CALLER,
    /// Get deposited value by the instruction/transaction responsible for this execution.
    CALLVALUE,
    /// Get input data of current environment.
    CALLDATALOAD,
    /// Get size of input data in current environment.
    CALLDATASIZE,
    /// Copy input data in current environment to memory.
    CALLDATACOPY,
    /// Get size of code running in current environment.
    CODESIZE,
    /// Copy code running in current environment to memory.
    CODECOPY,
    /// Get price of gas in current environment.
    GASPRICE,
    /// Get external code size.
    EXTCODESIZE,
    /// Copy external code to memory.
    EXTCODECOPY,
    /// Get size of available gas.
    RETURNDATASIZE,
    /// Copy output data to memory.
    RETURNDATACOPY,
    /// Get size of code running in current environment.
    EXTCODEHASH,
    // 0x40 range - Block Information
    //
    /// Get hash of most recent complete block.
    BLOCKHASH,
    /// Get the block's coinbase address.
    COINBASE,
    /// Get the block's timestamp.
    TIMESTAMP,
    /// Get the block's number.
    NUMBER,
    /// Get the block's difficulty.
    DIFFICULTY,
    /// Get the block's gas limit.
    GASLIMIT,
    /// Get the chain ID.
    CHAINID,
    /// Get balance of currently executing account.
    SELFBALANCE,
    /// Get the base fee.
    BASEFEE,
    // 0x50 range - Stack, Memory, Storage and Flow Operations
    //
    /// Remove item from stack.
    POP,
    /// Load word from memory.
    MLOAD,
    /// Save word to memory.
    MSTORE,
    /// Save byte to memory.
    MSTORE8,
    /// Load word from storage.
    SLOAD,
    /// Save word to storage.
    SSTORE,
    /// Alter the program counter.
    JUMP,
    /// Conditionally alter the program counter.
    JUMPI,
    /// Get the program counter.
    PC,
    /// Get the size of active memory.
    MSIZE,
    /// Get the amount of available gas.
    GAS,
    /// Set a potential jump destination.
    JUMPDEST,
    // 0x5f range - Push Operations
    //
    /// Place value 0 on stack.
    PUSH0,
    /// Place 1 byte item on stack.
    PUSH1,
    /// Place 2 byte item on stack.
    PUSH2,
    /// Place 3 byte item on stack.
    PUSH3,
    /// Place 4 byte item on stack.
    PUSH4,
    /// Place 5 byte item on stack.
    PUSH5,
    /// Place 6 byte item on stack.
    PUSH6,
    /// Place 7 byte item on stack.
    PUSH7,
    /// Place 8 byte item on stack.
    PUSH8,
    /// Place 9 byte item on stack.
    PUSH9,
    /// Place 10 byte item on stack.
    PUSH10,
    /// Place 11 byte item on stack.
    PUSH11,
    /// Place 12 byte item on stack.
    PUSH12,
    /// Place 13 byte item on stack.
    PUSH13,
    /// Place 14 byte item on stack.
    PUSH14,
    /// Place 15 byte item on stack.
    PUSH15,
    /// Place 16 byte item on stack.
    PUSH16,
    /// Place 17 byte item on stack.
    PUSH17,
    /// Place 18 byte item on stack.
    PUSH18,
    /// Place 19 byte item on stack.
    PUSH19,
    /// Place 20 byte item on stack.
    PUSH20,
    /// Place 21 byte item on stack.
    PUSH21,
    /// Place 22 byte item on stack.
    PUSH22,
    /// Place 23 byte item on stack.
    PUSH23,
    /// Place 24 byte item on stack.
    PUSH24,
    /// Place 25 byte item on stack.
    PUSH25,
    /// Place 26 byte item on stack.
    PUSH26,
    /// Place 27 byte item on stack.
    PUSH27,
    /// Place 28 byte item on stack.
    PUSH28,
    /// Place 29 byte item on stack.
    PUSH29,
    /// Place 30 byte item on stack.
    PUSH30,
    /// Place 31 byte item on stack.
    PUSH31,
    /// Place 32 byte item on stack.
    PUSH32,
    // 0x80 range - Duplication Operations
    //
    /// Duplicate 1st stack item.
    DUP1,
    /// Duplicate 2nd stack item.
    DUP2,
    /// Duplicate 3rd stack item.
    DUP3,
    /// Duplicate 4th stack item.
    DUP4,
    /// Duplicate 5th stack item.
    DUP5,
    /// Duplicate 6th stack item.
    DUP6,
    /// Duplicate 7th stack item.
    DUP7,
    /// Duplicate 8th stack item.
    DUP8,
    /// Duplicate 9th stack item.
    DUP9,
    /// Duplicate 10th stack item.
    DUP10,
    /// Duplicate 11th stack item.
    DUP11,
    /// Duplicate 12th stack item.
    DUP12,
    /// Duplicate 13th stack item.
    DUP13,
    /// Duplicate 14th stack item.
    DUP14,
    /// Duplicate 15th stack item.
    DUP15,
    /// Duplicate 16th stack item.
    DUP16,
    // 0x90 range - Exchange Operations
    //
    /// Exchange 1st and 2nd stack items.
    SWAP1,
    /// Exchange 1st and 3rd stack items.
    SWAP2,
    /// Exchange 1st and 4th stack items.
    SWAP3,
    /// Exchange 1st and 5th stack items.
    SWAP4,
    /// Exchange 1st and 6th stack items.
    SWAP5,
    /// Exchange 1st and 7th stack items.
    SWAP6,
    /// Exchange 1st and 8th stack items.
    SWAP7,
    /// Exchange 1st and 9th stack items.
    SWAP8,
    /// Exchange 1st and 10th stack items.
    SWAP9,
    /// Exchange 1st and 11th stack items.
    SWAP10,
    /// Exchange 1st and 12th stack items.
    SWAP11,
    /// Exchange 1st and 13th stack items.
    SWAP12,
    /// Exchange 1st and 14th stack items.
    SWAP13,
    /// Exchange 1st and 15th stack items.
    SWAP14,
    /// Exchange 1st and 16th stack items.
    SWAP15,
    /// Exchange 1st and 17th stack items.
    SWAP16,
    // 0xa0 range - Logging Operations
    //
    /// Append log record with no topics.
    LOG0,
    /// Append log record with one topic.
    LOG1,
    /// Append log record with two topics.
    LOG2,
    /// Append log record with three topics.
    LOG3,
    /// Append log record with four topics.
    LOG4,
    // 0xf0 range - System operations
    //
    /// Create a new account with associated code.
    CREATE,
    /// Message-call into an account.
    CALL,
    /// Message-call into this account with an alternative account's code.
    CALLCODE,
    /// Halts execution returning output data.
    RETURN,
    /// Message-call into this account with an alternative account's code,
    /// but with persistent state and code not being modified.
    DELEGATECALL,
    /// Create a new account without associated code.
    CREATE2,
    /// Static message-call into an account.
    STATICCALL,
    /// Halt execution and register account for later deletion.
    REVERT,
    /// Designated invalid instruction.
    INVALID,
    /// Halt execution and register account for later deletion, unless
    /// already scheduled.
    SELFDESTRUCT,
}

impl From<u8> for OpCode {
    fn from(code: u8) -> Self {
        match code {
            0x00 => Self::STOP,
            0x01 => Self::ADD,
            0x02 => Self::MUL,
            0x03 => Self::SUB,
            0x04 => Self::DIV,
            0x05 => Self::SDIV,
            0x06 => Self::MOD,
            0x07 => Self::SMOD,
            0x08 => Self::ADDMOD,
            0x09 => Self::MULMOD,
            0x0a => Self::EXP,
            0x0b => Self::SIGNEXTEND,
            0x10 => Self::LT,
            0x11 => Self::GT,
            0x12 => Self::SLT,
            0x13 => Self::SGT,
            0x14 => Self::EQ,
            0x15 => Self::ISZERO,
            0x16 => Self::AND,
            0x17 => Self::OR,
            0x18 => Self::XOR,
            0x19 => Self::NOT,
            0x1a => Self::BYTE,
            0x1b => Self::SHL,
            0x1c => Self::SHR,
            0x1d => Self::SAR,
            0x20 => Self::SHA3,
            0x30 => Self::ADDRESS,
            0x31 => Self::BALANCE,
            0x32 => Self::ORIGIN,
            0x33 => Self::CALLER,
            0x34 => Self::CALLVALUE,
            0x35 => Self::CALLDATALOAD,
            0x36 => Self::CALLDATASIZE,
            0x37 => Self::CALLDATACOPY,
            0x38 => Self::CODESIZE,
            0x39 => Self::CODECOPY,
            0x3a => Self::GASPRICE,
            0x3b => Self::EXTCODESIZE,
            0x3c => Self::EXTCODECOPY,
            0x3d => Self::RETURNDATASIZE,
            0x3e => Self::RETURNDATACOPY,
            0x3f => Self::EXTCODEHASH,
            0x40 => Self::BLOCKHASH,
            0x41 => Self::COINBASE,
            0x42 => Self::TIMESTAMP,
            0x43 => Self::NUMBER,
            0x44 => Self::DIFFICULTY,
            0x45 => Self::GASLIMIT,
            0x46 => Self::CHAINID,
            0x47 => Self::SELFBALANCE,
            0x48 => Self::BASEFEE,
            0x50 => Self::POP,
            0x51 => Self::MLOAD,
            0x52 => Self::MSTORE,
            0x53 => Self::MSTORE8,
            0x54 => Self::SLOAD,
            0x55 => Self::SSTORE,
            0x56 => Self::JUMP,
            0x57 => Self::JUMPI,
            0x58 => Self::PC,
            0x59 => Self::MSIZE,
            0x5a => Self::GAS,
            0x5b => Self::JUMPDEST,
            0x5f => Self::PUSH0,
            0x60 => Self::PUSH1,
            0x61 => Self::PUSH2,
            0x62 => Self::PUSH3,
            0x63 => Self::PUSH4,
            0x64 => Self::PUSH5,
            0x65 => Self::PUSH6,
            0x66 => Self::PUSH7,
            0x67 => Self::PUSH8,
            0x68 => Self::PUSH9,
            0x69 => Self::PUSH10,
            0x6a => Self::PUSH11,
            0x6b => Self::PUSH12,
            0x6c => Self::PUSH13,
            0x6d => Self::PUSH14,
            0x6e => Self::PUSH15,
            0x6f => Self::PUSH16,
            0x70 => Self::PUSH17,
            0x71 => Self::PUSH18,
            0x72 => Self::PUSH19,
            0x73 => Self::PUSH20,
            0x74 => Self::PUSH21,
            0x75 => Self::PUSH22,
            0x76 => Self::PUSH23,
            0x77 => Self::PUSH24,
            0x78 => Self::PUSH25,
            0x79 => Self::PUSH26,
            0x7a => Self::PUSH27,
            0x7b => Self::PUSH28,
            0x7c => Self::PUSH29,
            0x7d => Self::PUSH30,
            0x7e => Self::PUSH31,
            0x7f => Self::PUSH32,
            0x80 => Self::DUP1,
            0x81 => Self::DUP2,
            0x82 => Self::DUP3,
            0x83 => Self::DUP4,
            0x84 => Self::DUP5,
            0x85 => Self::DUP6,
            0x86 => Self::DUP7,
            0x87 => Self::DUP8,
            0x88 => Self::DUP9,
            0x89 => Self::DUP10,
            0x8a => Self::DUP11,
            0x8b => Self::DUP12,
            0x8c => Self::DUP13,
            0x8d => Self::DUP14,
            0x8e => Self::DUP15,
            0x8f => Self::DUP16,
            0x90 => Self::SWAP1,
            0x91 => Self::SWAP2,
            0x92 => Self::SWAP3,
            0x93 => Self::SWAP4,
            0x94 => Self::SWAP5,
            0x95 => Self::SWAP6,
            0x96 => Self::SWAP7,
            0x97 => Self::SWAP8,
            0x98 => Self::SWAP9,
            0x99 => Self::SWAP10,
            0x9a => Self::SWAP11,
            0x9b => Self::SWAP12,
            0x9c => Self::SWAP13,
            0x9d => Self::SWAP14,
            0x9e => Self::SWAP15,
            0x9f => Self::SWAP16,
            0xa0 => Self::LOG0,
            0xa1 => Self::LOG1,
            0xa2 => Self::LOG2,
            0xa3 => Self::LOG3,
            0xa4 => Self::LOG4,
            0xf0 => Self::CREATE,
            0xf1 => Self::CALL,
            0xf2 => Self::CALLCODE,
            0xf3 => Self::RETURN,
            0xf4 => Self::DELEGATECALL,
            0xf5 => Self::CREATE2,
            0xfa => Self::STATICCALL,
            0xfd => Self::REVERT,
            0xfe => Self::INVALID,
            0xff => Self::SELFDESTRUCT,
            _ => Self::INVALID,
        }
    }
}

impl From<OpCode> for u16 {
    fn from(opcode: OpCode) -> Self {
        match opcode {
            OpCode::STOP | OpCode::RETURN => 0,
            OpCode::JUMPDEST => 1,
            OpCode::ADDRESS
            | OpCode::ORIGIN
            | OpCode::CALLER
            | OpCode::CALLVALUE
            | OpCode::CODESIZE
            | OpCode::GASPRICE
            | OpCode::RETURNDATASIZE
            | OpCode::COINBASE
            | OpCode::TIMESTAMP
            | OpCode::NUMBER
            | OpCode::DIFFICULTY
            | OpCode::GASLIMIT
            | OpCode::CHAINID
            | OpCode::BASEFEE
            | OpCode::POP
            | OpCode::PC
            | OpCode::MSIZE
            | OpCode::GAS
            | OpCode::PUSH0 => 2,
            OpCode::MUL
            | OpCode::DIV
            | OpCode::SDIV
            | OpCode::MOD
            | OpCode::SMOD
            | OpCode::SIGNEXTEND
            | OpCode::SELFBALANCE => 5,
            OpCode::ADDMOD | OpCode::MULMOD | OpCode::JUMP => 8,
            OpCode::EXP | OpCode::JUMPI => 10,
            OpCode::BLOCKHASH => 20,
            OpCode::SHA3 => 30,
            OpCode::BALANCE
            | OpCode::EXTCODESIZE
            | OpCode::EXTCODECOPY
            | OpCode::EXTCODEHASH
            | OpCode::SLOAD
            | OpCode::SSTORE
            | OpCode::CALL
            | OpCode::CALLCODE
            | OpCode::DELEGATECALL
            | OpCode::STATICCALL => 100,
            OpCode::LOG0 => 375,
            OpCode::LOG1 => 750,
            OpCode::LOG2 => 1125,
            OpCode::LOG3 => 1500,
            OpCode::LOG4 => 1875,
            OpCode::SELFDESTRUCT => 5000,
            OpCode::CREATE | OpCode::CREATE2 => 32000,
            _ => 3,
        }
    }
}

impl OpCode {
    /// Returns the minimal gas cost of the opcode.
    pub fn gas<T>(&self) -> T
    where
        T: From<u16>,
    {
        T::from(u16::from(*self))
    }
}
