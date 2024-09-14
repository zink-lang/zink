//! Contract constructor.

use crate::{wasm::ToLSBytes, Buffer, MacroAssembler, Result};
use smallvec::SmallVec;
use std::collections::HashMap;

/// Initial storage of contracts
pub type InitStorage = HashMap<SmallVec<[u8; 32]>, SmallVec<[u8; 32]>>;

/// Contract constructor.
///
/// # Bytecode
/// - `CREATE` instruction
/// - `INIT_CODE`
///   - `INIT_LOGIC`
///   - `RETURN RUNTIME_BYTECODE`
/// - `RUNTIME_BYTECODE`
///
/// TODO: introduce ABI for constructor
#[derive(Default, Debug, Clone)]
pub struct Constructor {
    /// Code generator.
    masm: MacroAssembler,
}

impl Constructor {
    /// preset storage for the contract
    pub fn storage(&mut self, mapping: InitStorage) -> Result<()> {
        for (key, value) in mapping.into_iter() {
            self.masm.push(&value)?;
            self.masm.push(&key)?;
            self.masm._sstore()?;
        }

        Ok(())
    }

    /// Concat the constructor code.
    ///
    /// Here we override the memory totally with
    /// the runtime bytecode.
    pub fn finish(&self, runtime_bytecode: Buffer) -> Result<Buffer> {
        let init_code = self.masm.buffer();
        tracing::trace!("init code: {}", hex::encode(&init_code));
        let init_code_len = init_code.len();
        let runtime_bytecode_len = runtime_bytecode.len();
        let runtime_bytecode_size = runtime_bytecode_len.to_ls_bytes();
        let runtime_bytecode_offset =
            Self::runtime_bytcode_offset(init_code_len, runtime_bytecode_size.len());

        let mut masm = self.masm.clone();

        // 2. copy runtime bytecode to memory
        masm.push(&runtime_bytecode_size)?; // code size
        masm.push(&runtime_bytecode_offset.to_ls_bytes())?; // code offset
        masm._push0()?; // dest offset in memory
        masm._codecopy()?;

        // 3. return runtime bytecode
        masm.push(&runtime_bytecode_size)?; // code size
        masm._push0()?; // memory offset
        masm.asm._return()?;
        masm.buffer_mut().extend_from_slice(&runtime_bytecode);

        Ok(masm.buffer().into())
    }

    /// Returns the offset of runtime bytecode.
    ///
    /// [
    ///   init_code,
    ///   pushn, runtime_bytecode_size, pushn + <offset>, push0, code_copy
    ///   pushn, runtime_bytecode_size, push0, return,
    ///   <OFFSET>
    /// ]
    fn runtime_bytcode_offset(init_code_len: usize, runtime_bytecode_size_len: usize) -> usize {
        let mut offset = init_code_len + runtime_bytecode_size_len * 2 + 8;
        if (offset <= 0xff) && (offset + offset.to_ls_bytes().len() > 0xff) {
            offset += 1;
        }

        offset
    }
}
