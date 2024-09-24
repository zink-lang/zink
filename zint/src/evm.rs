//! Wrapper of revm

use anyhow::{anyhow, Result};
use revm::{
    db::EmptyDB,
    primitives::{
        AccountInfo, Bytecode, Bytes, ExecutionResult, HaltReason, Log, Output, ResultAndState,
        SuccessReason, TransactTo, TxKind, U256,
    },
    Evm as Revm, InMemoryDB,
};
use std::collections::HashMap;

/// Transaction gas limit.
const GAS_LIMIT: u64 = 1_000_000_000;

/// Alice account address.
pub const ALICE: [u8; 20] = [0; 20];

/// Contract address if any.
pub const CONTRACT: [u8; 20] = [1; 20];

/// Wrapper of full REVM
pub struct EVM<'e> {
    inner: Revm<'e, (), InMemoryDB>,
}

impl<'e> Default for EVM<'e> {
    fn default() -> Self {
        let mut db = InMemoryDB::default();
        db.insert_account_info(ALICE.into(), AccountInfo::from_balance(U256::MAX));

        let evm = Revm::<'e, (), EmptyDB>::builder().with_db(db).build();
        Self { inner: evm }
    }
}

impl<'e> EVM<'e> {
    /// Interpret runtime bytecode with provided arguments
    pub fn interp(runtime_bytecode: &[u8], input: &[u8]) -> Result<Info> {
        Self::default()
            .contract(runtime_bytecode)
            .calldata(input)
            .call(CONTRACT)
    }

    /// Send transaction to the provided address.
    pub fn call(&mut self, to: [u8; 20]) -> Result<Info> {
        let to = TransactTo::Call(to.into());
        self.inner.tx_mut().gas_limit = GAS_LIMIT;
        self.inner.tx_mut().transact_to = to;
        let result = self.inner.transact().map_err(|e| anyhow!(e))?;
        (result, to).try_into()
    }

    /// Interpret runtime bytecode with provided arguments
    pub fn deploy(&mut self, bytecode: &[u8]) -> Result<Info> {
        self.calldata(bytecode);
        self.inner.tx_mut().transact_to = TxKind::Create;
        self.inner.transact_commit()?.try_into()
    }

    /// Fill the calldata of the present transaction.
    pub fn calldata(&mut self, input: &[u8]) -> &mut Self {
        self.inner.tx_mut().data = Bytes::copy_from_slice(input);
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
        self.inner.db_mut()
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
    pub halt: Option<HaltReason>,
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
                if reason != SuccessReason::Return {
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
