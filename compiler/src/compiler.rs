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

        let env = parser.to_func_env();
        self.compile_dispatcher(&mut parser)?;
        for func in parser.funcs.into_funcs() {
            self.compile_func(env.clone(), func)?;
        }

        self.table.code_offset(self.buffer.len() as u16);
        self.table.relocate(&mut self.buffer)?;

        self.artifact()
    }

    /// Generate artifact
    ///
    /// yields runtime bytecode and construct bytecode
    fn artifact(self) -> Result<Artifact> {
        let Compiler {
            abi,
            buffer,
            config,
            ..
        } = self;

        Ok(Artifact {
            abi,
            config,
            runtime_bytecode: buffer.to_vec(),
        })
    }

    /// Compile EVM dispatcher.
    ///
    /// Drain selectors anyway, compile dispatcher if it is enabled.
    fn compile_dispatcher(&mut self, parser: &mut Parser) -> Result<()> {
        let selectors = parser.funcs.drain_selectors(&parser.exports);
        let env = parser.to_env();

        if !self.config.dispatcher {
            self.abi.append(&mut env.load_abis(&selectors)?);
            return Ok(());
        }

        let mut dispatcher = Dispatcher::new(env, &parser.funcs)?;
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
        let abi = self.abi(&env, func_index);

        tracing::debug!("compile function {func_index} {:?}, abi: {abi:#?}", sig);
        let is_main = if self.config.dispatcher {
            false
        } else {
            func_index - (env.imports.len() as u32) == 0
        };

        let mut codegen = Function::new(env, sig, abi, is_main)?;
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

    /// Get abi from env and function index
    fn abi(&self, env: &Env, index: u32) -> Option<Abi> {
        let name = env.exports.get(&index)?;
        self.abi.iter().find(|a| name == &a.name).cloned()
    }
}
