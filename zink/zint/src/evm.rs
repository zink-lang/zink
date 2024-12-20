//! Wrapper of revm

use anyhow::{anyhow, Result};
use revm::{
    db::EmptyDB,
    primitives::{
        AccountInfo, Bytecode, Bytes, ExecutionResult, HaltReason, Log, Output, ResultAndState,
        SuccessReason, TransactTo, TxKind, B256, U256,
    },
    Database, Evm as Revm, InMemoryDB,
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
    /// Caller for the execution
    pub caller: [u8; 20],
    /// Blob hashes
    pub blob_hashes: Option<Vec<B256>>,
    /// The gas limit of the transaction
    pub tx_gas_limit: u64,
    /// If commit changes
    commit: bool,
}

impl<'e> Default for EVM<'e> {
    fn default() -> Self {
        let mut db = InMemoryDB::default();
        db.insert_account_info(ALICE.into(), AccountInfo::from_balance(U256::MAX));

        let evm = Revm::<'e, (), EmptyDB>::builder().with_db(db).build();
        Self {
            inner: evm,
            caller: [0; 20],
            blob_hashes: None,
            tx_gas_limit: GAS_LIMIT,
            commit: false,
        }
    }
}

impl EVM<'_> {
    /// Interpret runtime bytecode with provided arguments
    pub fn interp(runtime_bytecode: &[u8], input: &[u8]) -> Result<Info> {
        Self::default()
            .contract(runtime_bytecode)
            .calldata(input)
            .call(CONTRACT)
    }

    /// Get storage from address and storage index
    pub fn storage(&mut self, address: [u8; 20], key: [u8; 32]) -> Result<[u8; 32]> {
        let db = self.inner.db_mut();
        Ok(db
            .storage(address.into(), U256::from_be_bytes(key))?
            .to_be_bytes())
    }

    /// If commit changes
    pub fn commit(mut self, flag: bool) -> Self {
        self.commit = flag;
        self
    }

    /// Set caller for the execution
    pub fn caller(mut self, caller: [u8; 20]) -> Self {
        self.caller = caller;
        self
    }

    /// Set chain id
    pub fn chain_id(mut self, id: u64) -> Self {
        self.inner.tx_mut().chain_id = Some(id);
        self
    }

    /// Set block number
    pub fn block_number(mut self, number: u64) -> Self {
        self.inner.block_mut().number = U256::from(number);
        self
    }

    /// Set block hash
    pub fn block_hash(mut self, number: u64, hash: [u8; 32]) -> Self {
        self.inner
            .db_mut()
            .block_hashes
            .insert(U256::from(number), hash.into());
        self
    }

    /// Set blob hashes
    pub fn blob_hashes(mut self, blob_hashes: Vec<[u8; 32]>) -> Self {
        let blob_hashes = blob_hashes.into_iter().map(Into::into).collect();
        self.blob_hashes = Some(blob_hashes);
        self
    }

    /// Set block basefee
    pub fn basefee(mut self, basefee: u64, gas_price: u64) -> Self {
        self.inner.block_mut().basefee = U256::from(basefee);
        self.inner.tx_mut().gas_price = U256::from(gas_price);
        self
    }

    /// Set block’s blob basefee
    pub fn blob_basefee(mut self, excess_blob_gas: u64) -> Self {
        self.inner
            .block_mut()
            .set_blob_excess_gas_and_price(excess_blob_gas);
        self
    }

    /// Get block’s blob basefee
    pub fn get_blob_basefee(&self) -> [u8; 32] {
        let basefee = self.inner.block().get_blob_gasprice();
        let basefee = match basefee {
            Some(fee) => fee.to_be_bytes(),
            None => [0; 16],
        };
        let mut blob_basefee = [0; 32];
        blob_basefee[16..].copy_from_slice(&basefee);
        blob_basefee
    }

    /// Set block’s coinbase
    pub fn coinbase(mut self, coinbase: [u8; 20]) -> Self {
        self.inner.block_mut().coinbase = coinbase.into();
        self
    }

    /// Set block’s prevrandao
    pub fn prevrandao(mut self, prevrandao: [u8; 32]) -> Self {
        self.inner.block_mut().prevrandao = Some(B256::from(prevrandao));
        self
    }

    /// Set block’s timestamp
    pub fn timestamp(mut self, timestamp: u64) -> Self {
        self.inner.block_mut().timestamp = U256::from(timestamp);
        self
    }

    /// Set tx’s gaslimit
    pub fn tx_gas_limit(mut self, gaslimit: u64) -> Self {
        self.tx_gas_limit = gaslimit;
        self
    }

    /// Send transaction to the provided address.
    pub fn call(&mut self, to: [u8; 20]) -> Result<Info> {
        let to = TransactTo::Call(to.into());
        self.inner.tx_mut().gas_limit = self.tx_gas_limit;
        self.inner.tx_mut().transact_to = to;
        self.inner.tx_mut().caller = self.caller.into();
        if let Some(hashes) = &self.blob_hashes {
            self.inner.tx_mut().max_fee_per_blob_gas = Some(U256::from(1));
            self.inner.tx_mut().blob_hashes = hashes.clone();
        }

        if self.commit {
            self.inner.transact_commit()?.try_into()
        } else {
            let result = self.inner.transact().map_err(|e| anyhow!(e))?;
            (result, to).try_into()
        }
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
    /// The revert message.
    pub revert: Option<String>,
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
            ExecutionResult::Revert { gas_used, output } => {
                info.gas = gas_used;
                info.revert = Some(
                    String::from_utf8_lossy(&output)
                        .trim_start_matches("\0")
                        .to_string(),
                );
            }
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
