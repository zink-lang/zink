//! Re-export REVM intepreter for testing usages.

pub use revm_interpreter::instruction_result::InstructionResult;
use revm_interpreter::{Contract, DummyHost, Interpreter};
use revm_primitives::{bytecode::Bytecode, specification::ShanghaiSpec, U256};

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
            interpreter: Interpreter::new(contract, 1_000_000, true),
            host: DummyHost::new(Default::default()),
        }
    }

    /// Execute a contract.
    pub fn execute(&mut self) -> (Vec<u8>, InstructionResult) {
        let instr = self
            .interpreter
            .run::<DummyHost, ShanghaiSpec>(&mut self.host);

        let ret = self.interpreter.return_value();
        (ret.to_vec(), instr)
    }

    /// Run a contract.
    pub fn run(btyecode: &[u8], input: &[u8]) -> (Vec<u8>, InstructionResult) {
        let mut evm = Self::new(btyecode, input);
        evm.execute()
    }
}
