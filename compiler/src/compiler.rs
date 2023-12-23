//! Zink compiler

use crate::{parser::Parser, Config, Error, Result};
use zabi::Abi;
use zingen::{
    wasm::{self, Data, Imports},
    Buffer, Constructor, Dispatcher, Function, JumpTable, BUFFER_LIMIT,
};

/// Zink Compiler
#[derive(Default)]
pub struct Compiler {
    abi: Vec<Abi>,
    buffer: Buffer,
    table: JumpTable,
    config: Config,
}

impl Compiler {
    /// If embed constructor in bytecode.
    pub fn constructor(mut self, constructor: bool) -> Self {
        self.config.constructor = constructor;
        self
    }

    /// If embed dispatcher in bytecode.
    pub fn dispatcher(mut self, dispatcher: bool) -> Self {
        self.config.dispatcher = dispatcher;
        self
    }

    /// Compile wasm module to evm bytecode.
    ///
    /// Returns runtime bytecode.
    pub fn compile(&mut self, wasm: &[u8]) -> Result<Buffer> {
        let mut parser = Parser::try_from(wasm)?;
        let constructor = parser.remove_constructor();

        self.compile_dispatcher(&mut parser)?;
        for func in parser.funcs.into_funcs() {
            self.compile_func(parser.data.clone(), parser.imports.clone(), func)?;
        }

        self.table.code_offset(self.buffer.len() as u16);
        self.table.relocate(&mut self.buffer)?;

        if self.config.constructor {
            self.bytecode(constructor)
        } else {
            Ok(self.buffer.clone())
        }
    }

    /// Compile EVM dispatcher.
    ///
    /// Drain selectors anyway, if dispatcher is
    /// enabled, compile dispatcher.
    pub fn compile_dispatcher(&mut self, parser: &mut Parser) -> Result<()> {
        let selectors = parser.funcs.drain_selectors(&parser.exports);
        if !self.config.dispatcher {
            return Ok(());
        }

        let mut dispatcher = Dispatcher::new(&parser.funcs);
        dispatcher
            .data(parser.data.clone())
            .exports(parser.exports.clone())
            .imports(parser.imports.clone());

        let buffer = dispatcher.finish(selectors, &mut self.table)?;
        self.buffer.extend_from_slice(&buffer);

        if self.buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(self.buffer.len()));
        }

        self.abi.append(&mut dispatcher.abi);
        Ok(())
    }

    /// Compile WASM function.
    pub fn compile_func(
        &mut self,
        dataset: Data,
        imports: Imports,
        mut func: wasm::Function<'_>,
    ) -> Result<()> {
        let func_index = func.index();
        let sig = func.sig()?;

        tracing::trace!("compile function {}: {:?}", func_index, sig);
        let is_main = if self.config.dispatcher {
            false
        } else {
            func_index - (imports.len() as u32) == 0
        };

        let mut codegen = Function::new(sig, dataset, imports, is_main)?;
        let mut locals_reader = func.body.get_locals_reader()?;
        let mut ops_reader = func.body.get_operators_reader()?;

        codegen.emit_locals(&mut locals_reader, &mut func.validator)?;
        codegen.emit_operators(&mut ops_reader, &mut func.validator)?;

        self.emit_buffer(func_index, codegen)?;
        Ok(())
    }

    /// Emit buffer to the inner buffer.
    fn emit_buffer(&mut self, func_index: u32, codegen: Function) -> Result<()> {
        let buffer = codegen.finish(&mut self.table, self.buffer.len() as u16)?;
        self.table
            .call_offset(func_index, self.buffer.len() as u16)?;
        self.buffer.extend_from_slice(&buffer);

        if self.buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(buffer.len()));
        }

        Ok(())
    }

    /// Get the abi of the compiled contract.
    pub fn abi(&self) -> Vec<Abi> {
        self.abi.clone()
    }

    /// Returns bytecode.
    fn bytecode(&self, constructor: Option<wasm::Function<'_>>) -> Result<Buffer> {
        Constructor::new(constructor, self.buffer.clone())?
            .finish()
            .map_err(Into::into)
    }
}
