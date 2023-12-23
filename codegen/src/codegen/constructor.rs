//! Contract constructor.

use crate::{wasm, Buffer, Function, JumpTable, MacroAssembler, Result, ToLSBytes};

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
pub struct Constructor {
    /// Code buffer.
    pub masm: MacroAssembler,

    /// Code generator.
    pub init_code: Buffer,

    /// Runtime bytecode.
    pub runtime_bytecode: Buffer,
}

impl Constructor {
    /// Create a new constructor.
    pub fn new(constructor: Option<wasm::Function<'_>>, runtime_bytecode: Buffer) -> Result<Self> {
        let mut init_code = Buffer::new();
        if let Some(constructor) = constructor {
            let codegen = Function::new(
                constructor.sig()?,
                Default::default(),
                Default::default(),
                // No `return` instruction in the generated code.
                false,
            )?;

            let mut jump_table = JumpTable::default();
            init_code = codegen.finish(&mut jump_table, 0)?;
            jump_table.relocate(&mut init_code)?;
        };

        Ok(Self {
            masm: MacroAssembler::default(),
            init_code,
            runtime_bytecode,
        })
    }

    /// Returns the length of instructions.
    fn return_instr_length(init_code_length: usize, runtime_bytecode_length: usize) -> usize {
        let mut expected_length =
            runtime_bytecode_length.to_ls_bytes().len() + init_code_length.to_ls_bytes().len() + 3;

        if init_code_length < 0xff && init_code_length + expected_length > 0xff {
            expected_length += 1;
        }

        expected_length
    }

    /// Concat the constructor code.
    ///
    /// Here we override the memory totally with
    /// the runtime bytecode.
    pub fn finish(&mut self) -> Result<Buffer> {
        let init_code_length = self.init_code.len();
        let runtime_bytecode_length = self.runtime_bytecode.len();
        let return_instr_length =
            Self::return_instr_length(init_code_length, runtime_bytecode_length);

        // Copy init code and runtime bytecode to memory from offset 0.
        //
        // 1. code size ( init_code + instr_return + runtime_bytecode )
        // 2. byte offset of code which is fixed to N.
        // 3. destination offset which is fixed to 0.
        {
            self.masm.push(
                &(init_code_length + return_instr_length + runtime_bytecode_length).to_ls_bytes(),
            )?;
            // # SAFETY
            //
            // The length of the most significiant bytes of
            // the bytecode offset is fixed to 1.
            self.masm
                .push(&((self.masm.pc_offset() as usize + 9).to_ls_bytes()))?;
            self.masm._push0()?;
            self.masm._codecopy()?;
        }

        // Process instruction `CREATE`
        {
            self.masm._push0()?;
            self.masm._push0()?;
            self.masm._push0()?;
            self.masm._calldataload()?;
            self.masm._create()?;
        }

        self.masm.buffer_mut().extend_from_slice(&self.init_code);

        // Process `RETURN`.
        //
        // 1. size of the runtime bytecode
        // 2. offset of the runtime bytecode in memory
        {
            self.masm.push(&runtime_bytecode_length.to_ls_bytes())?;
            self.masm
                .push(&(init_code_length + return_instr_length).to_ls_bytes())?;
            self.masm.asm._return()?;
        }

        self.masm
            .buffer_mut()
            .extend_from_slice(&self.runtime_bytecode);

        Ok(self.masm.buffer().into())
    }
}
