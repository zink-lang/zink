//! Contract constructor.

use crate::{wasm::ToLSBytes, Buffer, MacroAssembler, Result};

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
    pub masm: MacroAssembler,
}

impl Constructor {
    /// Concat the constructor code.
    ///
    /// Here we override the memory totally with
    /// the runtime bytecode.
    pub fn finish(&mut self, runtime_bytecode: Buffer) -> Result<Buffer> {
        let init_code: Buffer = Default::default();
        let init_code_len = init_code.len();
        let runtime_bytecode_len = runtime_bytecode.len();
        let runtime_bytecode_size = runtime_bytecode_len.to_ls_bytes();
        let runtime_bytecode_offset =
            Self::runtime_bytcode_offset(init_code_len, runtime_bytecode_size.len());

        // 1. set up init code
        *self.masm.buffer_mut() = init_code.clone();

        // 2. copy runtime bytecode to memory
        self.masm.push(&runtime_bytecode_size)?; // code size
        self.masm.push(&runtime_bytecode_offset.to_ls_bytes())?; // code offset
        self.masm._push0()?; // dest offset in memory
        self.masm._codecopy()?;

        // 3. return runtime bytecode
        self.masm.push(&runtime_bytecode_size)?; // code size
        self.masm._push0()?; // memory offset
        self.masm.asm._return()?;
        self.masm.buffer_mut().extend_from_slice(&runtime_bytecode);

        Ok(self.masm.buffer().into())
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
