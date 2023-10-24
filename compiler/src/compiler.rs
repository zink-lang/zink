//! Zink compiler

use crate::{parser::Parser, Error, Result};
use zingen::{
    Buffer, CodeGen, DataSet, Dispatcher, Exports, Function, Functions, Imports, JumpTable,
    BUFFER_LIMIT,
};

/// Zink Compiler
#[derive(Default)]
pub struct Compiler {
    buffer: Buffer,
    table: JumpTable,
    dispatcher: bool,
}

impl Compiler {
    /// Embed dispatcher in bytecode.
    pub fn with_dispatcher(&mut self) -> &mut Self {
        self.dispatcher = true;
        self
    }

    /// Compile wasm module to evm bytecode.
    pub fn compile(mut self, wasm: &[u8]) -> Result<Buffer> {
        let Parser {
            imports,
            data,
            mut funcs,
            exports,
        } = Parser::try_from(wasm)?;
        tracing::trace!("imports: {:?}", imports);
        tracing::trace!("data: {:?}", data);
        tracing::trace!("exports: {:?}", exports);

        let selectors = funcs.drain_selectors(&exports);
        if !selectors.is_empty() && self.dispatcher {
            self.compile_dispatcher(data.clone(), exports, imports.clone(), selectors)?;
        }

        for func in funcs.into_funcs() {
            self.compile_func(data.clone(), imports.clone(), func)?;
        }

        self.finish()
    }

    /// Compile EVM dispatcher.
    pub fn compile_dispatcher(
        &mut self,
        data: DataSet,
        exports: Exports,
        imports: Imports,
        selectors: Functions,
    ) -> Result<()> {
        let mut dispatcher = Dispatcher::default();
        dispatcher.data(data).exports(exports).imports(imports);

        let buffer = dispatcher.finish(selectors, &mut self.table)?;
        self.buffer.extend_from_slice(&buffer);

        if self.buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(self.buffer.len()));
        }

        Ok(())
    }

    /// Compile WASM function.
    pub fn compile_func(
        &mut self,
        dataset: DataSet,
        imports: Imports,
        mut func: Function<'_>,
    ) -> Result<()> {
        let func_index = func.index();
        let sig = func.sig()?;

        tracing::debug!("compile function {}: {:?}", func_index, sig);

        let is_main = if self.dispatcher {
            false
        } else {
            func_index - (imports.len() as u32) == 0
        };

        let mut codegen = CodeGen::new(sig, dataset, imports, is_main)?;
        let mut locals_reader = func.body.get_locals_reader()?;
        let mut ops_reader = func.body.get_operators_reader()?;

        codegen.emit_locals(&mut locals_reader, &mut func.validator)?;
        codegen.emit_operators(&mut ops_reader, &mut func.validator)?;

        self.emit_buffer(func_index, codegen)?;
        Ok(())
    }

    /// Emit buffer to the inner buffer.
    fn emit_buffer(&mut self, func_index: u32, codegen: CodeGen) -> Result<()> {
        let buffer = codegen.finish(&mut self.table, self.buffer.len() as u16)?;
        self.table
            .call_offset(func_index, self.buffer.len() as u16)?;
        self.buffer.extend_from_slice(&buffer);

        if self.buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(buffer.len()));
        }

        Ok(())
    }

    /// Finish compilation.
    pub fn finish(mut self) -> Result<Buffer> {
        tracing::trace!("buffer length {:x}", self.buffer.len());
        self.table.code_offset(self.buffer.len() as u16);
        self.table.relocate(&mut self.buffer)?;

        Ok(self.buffer)
    }
}
