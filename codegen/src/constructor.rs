//! Contract constructor.

use crate::{Buffer, CodeGen, Function, JumpTable, MacroAssembler, Result};

/// Contract constructor.
///
/// # Bytecode
/// - `CREATE` instruction
/// - `INIT_CODE`
///   - `INIT_LOGIC`
///   - `RETURN RUNTIME_BYTECODE`
/// - `RUNTIME_BYTECODE`
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
    pub fn new(constructor: Option<Function<'_>>, runtime_bytecode: Buffer) -> Result<Self> {
        let mut init_code = Buffer::new();
        if let Some(constructor) = constructor {
            let codegen = CodeGen::new(
                constructor.sig()?,
                Default::default(),
                Default::default(),
                true,
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

    /// Concat the constructor code.
    pub fn finish(&self) -> Result<Buffer> {
        Ok(self.masm.buffer().into())
    }
}
