//! Re-export REVM interpreter for testing usages.

pub use revm::interpreter::{instruction_result::InstructionResult, primitives::U256};
use revm::interpreter::{
    primitives::{bytecode::Bytecode, specification::ShanghaiSpec},
    Contract, DummyHost, Interpreter,
};
use std::collections::HashMap;

const INITIAL_GAS: u64 = 1_000_000_000;

/// EVM execution result info.
#[derive(Debug)]
pub struct Info {
    /// Gas spent.
    pub gas: u64,
    /// The last instruction.
    pub instr: InstructionResult,
    /// Return value.
    pub ret: Vec<u8>,
    /// The storage.
    pub storage: HashMap<U256, U256>,
}

/// EVM interpreter.
pub struct EVM {
    interpreter: Interpreter,
    host: DummyHost,
}

impl EVM {
    /// Create a new EVM instance.
    pub fn new(btyecode: &[u8], input: &[u8]) -> Self {
        let contract = Contract::new(
            input.to_vec().into(),                       // input
            Bytecode::new_raw(btyecode.to_vec().into()), // code
            Default::default(),                          // address
            Default::default(),                          // caller
            U256::ZERO,                                  // value
        );

        Self {
            interpreter: Interpreter::new(contract, INITIAL_GAS, false),
            host: DummyHost::new(Default::default()),
        }
    }

    /// Execute a contract.
    pub fn execute(&mut self) -> Info {
        let instr = self
            .interpreter
            .run::<DummyHost, ShanghaiSpec>(&mut self.host);

        let ret = self.interpreter.return_value().to_vec();
        self.interpreter.gas();

        let storage = self
            .host
            .storage
            .clone()
            .into_iter()
            .map(|(k, v)| (k, U256::from_le_bytes(v.to_le_bytes::<32>())))
            .collect();

        Info {
            gas: self.interpreter.gas().spend(),
            instr,
            ret,
            storage,
        }
    }

    /// Run a contract.
    pub fn run(btyecode: &[u8], input: &[u8]) -> Info {
        let mut evm = Self::new(btyecode, input);
        let info = evm.execute();
        tracing::debug!("{info:?}");

        info
    }
}
