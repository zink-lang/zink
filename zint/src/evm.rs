//! Wrapper of revm

use anyhow::{anyhow, Result};
use revm::{
    primitives::{
        AccountInfo, Bytecode, Bytes, CreateScheme, Eval, ExecutionResult, Halt, Log, Output,
        ResultAndState, TransactTo, U256,
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
    /// Interpret runtime bytecode with provided arguments
    pub fn interp(runtime_bytecode: &[u8], input: &[u8]) -> Result<Info> {
        Self::default()
            .contract(runtime_bytecode)
            .calldata(input)
            .refcall(CONTRACT)
    }

    /// Reference call
    pub fn refcall(&mut self, to: [u8; 20]) -> Result<Info> {
        let to = TransactTo::Call(to.into());
        self.inner.env.tx.transact_to = to.clone();
        let result = self.inner.transact_ref().map_err(|e| anyhow!(e))?;
        (result, to).try_into()
    }

    /// Send transaction to the provided address.
    pub fn call(&mut self, to: [u8; 20]) -> Result<Info> {
        let to = TransactTo::Call(to.into());
        self.inner.env.tx.transact_to = to.clone();
        self.inner.transact_commit()?.try_into()
    }

    /// Interpret runtime bytecode with provided arguments
    pub fn deploy(&mut self, bytecode: &[u8]) -> Result<Info> {
        self.calldata(bytecode);
        self.inner.env.tx.transact_to = TransactTo::Create(CreateScheme::Create);
        self.inner.transact_commit()?.try_into()
    }

    /// Fill the calldata of the present transaction.
    pub fn calldata(&mut self, input: &[u8]) -> &mut Self {
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

    fn db(&mut self) -> &mut InMemoryDB {
        self.inner
            .db()
            .unwrap_or_else(|| unreachable!("provided on initialization"))
    }
}

/// Interp execution result info.
#[derive(Debug, Default)]
pub struct Info {
    /// the created contract address if any.
    pub address: [u8; 20],
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

impl TryFrom<ExecutionResult> for Info {
    type Error = anyhow::Error;

    fn try_from(result: ExecutionResult) -> Result<Self> {
        let mut info = Info {
            gas: result.gas_used(),
            ..Default::default()
        };

        match result {
            ExecutionResult::Success {
                logs,
                reason,
                output,
                ..
            } => {
                if reason != Eval::Return {
                    return Err(anyhow!("Transaction is not returned: {reason:?}"));
                }
                info.logs = logs;

                let ret = match output {
                    Output::Call(bytes) => bytes,
                    Output::Create(bytes, maybe_address) => {
                        let Some(address) = maybe_address else {
                            return Err(anyhow!(
                                "No contract created after the creation transaction."
                            ));
                        };

                        info.address = *address.as_ref();
                        bytes
                    }
                };

                info.ret = ret.into();
            }
            ExecutionResult::Halt { reason, .. } => {
                info.halt = Some(reason);
            }
            _ => unreachable!("This should never happen"),
        }

        Ok(info)
    }
}

impl TryFrom<(ResultAndState, TransactTo)> for Info {
    type Error = anyhow::Error;

    fn try_from((res, to): (ResultAndState, TransactTo)) -> Result<Self> {
        let ResultAndState { result, state } = res;
        let mut info = Self::try_from(result)?;

        if let TransactTo::Call(address) = to {
            info.storage = state
                .get(&address)
                .ok_or_else(|| anyhow!("no state found for account 0x{}", hex::encode(address)))?
                .storage
                .iter()
                .map(|(k, v)| (*k, v.present_value))
                .collect();
        }

        Ok(info)
    }
}
