//! Zink parser

use crate::{Error, Result};
use std::iter::IntoIterator;
use wasmparser::{
    Data, DataKind, Export, ExternalKind, Import, Operator, Payload, SectionLimited, TypeRef,
    ValidPayload, Validator,
};
use zingen::wasm::{Data as DataSet, Env, Exports, Functions, HostFunc, Imports};

/// WASM module parser
#[derive(Default)]
pub struct Parser<'p> {
    /// Function environment
    pub env: Env,
    /// All functions
    pub funcs: Functions<'p>,
}

impl<'p> Parser<'p> {
    /// Parse WASM module.
    pub fn parse(&mut self, wasm: &'p [u8]) -> Result<()> {
        let mut validator = Validator::new();

        // Compile functions.
        for payload in wasmparser::Parser::new(0).parse_all(wasm) {
            let payload = payload?;
            let valid_payload = validator.payload(&payload)?;

            match &payload {
                Payload::ImportSection(reader) => self.env.imports = Self::imports(reader)?,
                Payload::DataSection(reader) => self.env.data = Self::data(reader)?,
                Payload::ExportSection(reader) => self.env.exports = Self::exports(reader)?,
                _ => {}
            }

            if let ValidPayload::Func(to_validator, body) = valid_payload {
                self.funcs
                    .add(to_validator.into_validator(Default::default()), body);
            }
        }

        // compute slots from functions
        let mut slots = self.env.imports.reserved();
        for (idx, fun) in self.funcs.iter() {
            let sig = fun.sig()?;
            let locals = fun.body.get_locals_reader()?.get_count();
            let params = sig.params().len();
            tracing::trace!(
                "computing slots for function {idx}, locals: {locals}, params: {params}, reserved: {slots}, external: {}",
                self.env.is_external(fun.index())
            );

            self.env.slots.insert(fun.index(), slots);
            self.env
                .funcs
                .insert(fun.index(), (params as u32, sig.results().len() as u32));

            slots += locals;

            // process prarams for internal functions only
            if !self.env.is_external(fun.index()) && !self.env.is_main(fun.index()) {
                slots += params as u32;
            }
        }

        Ok(())
    }

    /// Drain selectors from parsed functions
    pub fn drain_selectors(&mut self) -> Functions<'p> {
        self.funcs.drain_selectors(&self.env.exports)
    }

    /// Parse data section.
    fn data(reader: &SectionLimited<Data>) -> Result<DataSet> {
        let mut dataset = DataSet::default();
        let mut iter = reader.clone().into_iter();
        while let Some(Ok(data)) = iter.next() {
            if let DataKind::Active {
                memory_index: _,
                offset_expr,
            } = data.kind
            {
                // [i32.const offset call_indirect]
                let mut reader = offset_expr.get_binary_reader();
                let Operator::I32Const { value: offset } = reader.read_operator()? else {
                    return Err(Error::InvalidDataOffset);
                };

                dataset.insert(offset, data.data.into());
            }
        }

        Ok(dataset)
    }

    /// Parse export section
    pub fn exports(reader: &SectionLimited<Export>) -> Result<Exports> {
        let mut exports = Exports::default();
        let mut iter = reader.clone().into_iter();
        while let Some(Ok(Export {
            name,
            kind: ExternalKind::Func,
            index,
        })) = iter.next()
        {
            if let Some(existing) = exports.get(&index) {
                return Err(anyhow::anyhow!(
                    "duplicate function: {name} and {existing} are sharing the same logic, \
                    consider removing one of them, see https://github.com/zink-lang/zink/issues/319 \
                    for more details."
                )
                .into());
            }

            exports.insert(index, name.into());
        }

        Ok(exports)
    }

    /// Parse import section.
    pub fn imports(reader: &SectionLimited<Import>) -> Result<Imports> {
        // TODO: use real index from WASM. (#122)
        let mut index = 0;

        let mut imports = Imports::default();
        let mut iter = reader.clone().into_iter();
        while let Some(Ok(Import {
            module,
            name,
            ty: TypeRef::Func(_),
        })) = iter.next()
        {
            let func = HostFunc::try_from((module, name))?;
            tracing::trace!("imported function: {}::{} at {index}", module, name);
            imports.insert(index, func);
            index += 1;
        }

        Ok(imports)
    }
}

impl<'p> TryFrom<&'p [u8]> for Parser<'p> {
    type Error = Error;

    fn try_from(wasm: &'p [u8]) -> Result<Self> {
        let mut parser = Self::default();
        parser.parse(wasm)?;
        Ok(parser)
    }
}
