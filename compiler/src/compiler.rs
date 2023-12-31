//! Zink compiler

use crate::{parser::Parser, Artifact, Config, Error, Result};
use zabi::Abi;
use zingen::{
    wasm::{self, Env},
    Buffer, Dispatcher, Function, JumpTable, BUFFER_LIMIT,
};

/// Zink Compiler
#[derive(Default)]
pub struct Compiler {
    /// ABIs of the compiled contract.
    pub(crate) abi: Vec<Abi>,
    /// EVM bytecode buffer.
    pub(crate) buffer: Buffer,
    /// Compiler configuration.
    pub config: Config,
    /// Global jump table.
    table: JumpTable,
}

impl Compiler {
    /// Create a new compiler from config.
    pub fn new(config: Config) -> Self {
        Self {
            config,
            ..Default::default()
        }
    }

    /// Compile wasm module to evm bytecode.
    ///
    /// Returns runtime bytecode.
    pub fn compile(mut self, wasm: &[u8]) -> Result<Artifact> {
        let mut parser = Parser::try_from(wasm)?;
        let constructor = parser.remove_constructor();

        self.compile_dispatcher(&mut parser)?;
        let env = parser.to_func_env();
        for func in parser.funcs.into_funcs() {
            self.compile_func(env.clone(), func)?;
        }

        self.table.code_offset(self.buffer.len() as u16);
        self.table.relocate(&mut self.buffer)?;

        Artifact::try_from((self, constructor)).map_err(Into::into)
    }

    /// Compile EVM dispatcher.
    ///
    /// Drain selectors anyway, if dispatcher is
    /// enabled, compile dispatcher.
    fn compile_dispatcher(&mut self, parser: &mut Parser) -> Result<()> {
        let selectors = parser.funcs.drain_selectors(&parser.exports);
        if !self.config.dispatcher {
            return Ok(());
        }

        let mut dispatcher = Dispatcher::new(parser.to_env(), &parser.funcs)?;
        let buffer = dispatcher.finish(selectors, &mut self.table)?;
        self.buffer.extend_from_slice(&buffer);

        if self.buffer.len() > BUFFER_LIMIT {
            return Err(Error::BufferOverflow(self.buffer.len()));
        }

        self.abi.append(&mut dispatcher.abi);
        Ok(())
    }

    /// Compile WASM function.
    fn compile_func(&mut self, env: Env, mut func: wasm::Function<'_>) -> Result<()> {
        let func_index = func.index();
        let sig = func.sig()?;

        tracing::trace!("compile function {}: {:?}", func_index, sig);
        let is_main = if self.config.dispatcher {
            false
        } else {
            func_index - (env.imports.len() as u32) == 0
        };

        let mut codegen = Function::new(env, sig, is_main)?;
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
}
