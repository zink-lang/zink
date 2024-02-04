//! Zink parser

use crate::{Error, Result};
use std::iter::IntoIterator;
use wasmparser::{
    Data, DataKind, Export, ExternalKind, FuncType, Import, Operator, Payload, SectionLimited,
    TypeRef, ValidPayload, Validator,
};
use zingen::wasm::{Data as DataSet, Env, Exports, Functions, HostFunc, Imports};

/// WASM module parser
#[derive(Default)]
pub struct Parser<'p> {
    pub imports: Imports,
    pub data: DataSet,
    pub funcs: Functions<'p>,
    pub exports: Exports,
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
                Payload::ImportSection(reader) => self.imports = Self::imports(reader)?,
                Payload::DataSection(reader) => self.data = Self::data(reader)?,
                Payload::ExportSection(reader) => self.exports = Self::exports(reader)?,
                _ => {}
            }

            if let ValidPayload::Func(to_validator, body) = valid_payload {
                self.funcs
                    .add(to_validator.into_validator(Default::default()), body);
            }
        }

        Ok(())
    }

    /// Parse data section.
    pub fn data(reader: &SectionLimited<Data>) -> Result<DataSet> {
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

    /// Returns constructor if some.
    pub fn remove_constructor(&mut self) -> Option<FuncType> {
        self.funcs.remove_constructor(&self.exports)
    }

    /// Returns full environment.
    pub fn to_env(&self) -> Env {
        Env {
            imports: self.imports.clone(),
            data: self.data.clone(),
            exports: self.exports.clone(),
        }
    }

    /// Returns function environment.
    pub fn to_func_env(&self) -> Env {
        Env {
            imports: self.imports.clone(),
            data: self.data.clone(),
            exports: self.exports.clone(),
        }
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
