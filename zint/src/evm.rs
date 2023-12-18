//! Wrapper of revm

use anyhow::{anyhow, Result};
use revm::{
    primitives::{
        AccountInfo, Bytecode, Bytes, Eval, ExecutionResult, Halt, Log, ResultAndState, TransactTo,
        U256,
    },
    InMemoryDB, EVM as REVM,
};
use std::collections::HashMap;

/// Transaction gas limit.
const GAS_LIMIT: u64 = 1_000_000_000;

/// Alice account address.
pub const ALICE: [u8; 20] = [0; 20];

/// Contract address if any.
pub const CONTRACT: [u8; 20] = [1; 20];

/// Wrapper of full REVM
pub struct EVM {
    inner: REVM<InMemoryDB>,
}

impl Default for EVM {
    fn default() -> Self {
        let mut db = InMemoryDB::default();
        db.insert_account_info(ALICE.into(), AccountInfo::from_balance(U256::MAX));

        let mut evm = REVM::new();
        evm.database(db);
        evm.env.tx.gas_limit = GAS_LIMIT;

        Self { inner: evm }
    }
}

impl EVM {
    fn db(&mut self) -> &mut InMemoryDB {
        self.inner
            .db()
            .unwrap_or_else(|| unreachable!("provided on initialization"))
    }

    /// Send transaction to the provided address.
    pub fn call(&mut self, to: [u8; 20]) -> Result<Info> {
        self.inner.env.tx.transact_to = TransactTo::Call(to.into());
        let result = self.inner.transact_ref().map_err(|e| anyhow!(e))?;
        (result, to).try_into()
    }

    /// Fill the calldata of the present transaction.
    pub fn calldata(mut self, input: &[u8]) -> Self {
        self.inner.env.tx.data = Bytes::copy_from_slice(input);
        self
    }

    /// Override the present contract
    pub fn contract(mut self, runtime_bytecode: &[u8]) -> Self {
        self.db().insert_account_info(
            CONTRACT.into(),
            AccountInfo::new(
                Default::default(),
                0,
                Default::default(),
                Bytecode::new_raw(Bytes::copy_from_slice(runtime_bytecode)),
            ),
        );

        self
    }

    /// Interpret runtime bytecode with provided arguments
    pub fn interp(runtime_bytecode: &[u8], input: &[u8]) -> Result<Info> {
        Self::default()
            .contract(runtime_bytecode)
            .calldata(input)
            .call(CONTRACT)
    }
}

/// Interp execution result info.
#[derive(Debug, Default)]
pub struct Info {
    /// Gas spent.
    pub gas: u64,
    /// Return value.
    pub ret: Vec<u8>,
    /// The storage.
    pub storage: HashMap<U256, U256>,
    /// Execution logs.
    pub logs: Vec<Log>,
    /// Transaction halt reason.
    pub halt: Option<Halt>,
}

impl TryFrom<(ResultAndState, [u8; 20])> for Info {
    type Error = anyhow::Error;

    fn try_from((res, address): (ResultAndState, [u8; 20])) -> Result<Self> {
        let ResultAndState { result, state } = res;
        let mut info: Self = Default::default();
        info.gas = result.gas_used();

        match result {
            ExecutionResult::Success {
                logs,
                reason,
                output,
                ..
            } => {
                if reason != Eval::Return {
                    return Err(anyhow!("Transaction not returned: {reason:?}."));
                }

                info.logs = logs;
                info.ret = output.into_data().to_vec();
            }
            ExecutionResult::Halt { reason, .. } => {
                info.halt = Some(reason);
            }
            _ => unreachable!("This should never happen"),
        }

        info.storage = state
            .get(&address)
            .ok_or_else(|| anyhow!("no state found for account 0x{}", hex::encode(&address)))?
            .storage
            .iter()
            .map(|(k, v)| (*k, v.present_value))
            .collect();

        Ok(info)
    }
}
