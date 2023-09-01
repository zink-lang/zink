//! Zink parser

use crate::{Error, Result};
use std::{collections::BTreeMap, iter::IntoIterator};
use wasmparser::{
    Data, DataKind, FuncToValidate, FunctionBody, Import, Operator, Payload, SectionLimited,
    TypeRef, ValidPayload, Validator, ValidatorResources,
};
use zingen::{DataSet, Func, Imports};

/// WASM module parser
#[derive(Default)]
pub struct Parser<'p> {
    pub imports: Imports,
    pub data: DataSet,
    pub funcs: BTreeMap<u32, (FuncToValidate<ValidatorResources>, FunctionBody<'p>)>,
}

impl<'p> Parser<'p> {
    /// Parse WASM module.
    pub fn parse(&mut self, wasm: &'p [u8]) -> Result<()> {
        let mut validator = Validator::new();
        let mut func_index = 0u32;
        let mut funcs = BTreeMap::new();

        // Compile functions.
        for payload in wasmparser::Parser::new(0).parse_all(wasm) {
            let payload = payload?;
            let valid_payload = validator.payload(&payload)?;

            match &payload {
                Payload::ImportSection(reader) => self.imports = Self::imports(reader),
                Payload::DataSection(reader) => self.data = Self::data(reader)?,
                _ => {}
            }

            if let ValidPayload::Func(to_validator, body) = valid_payload {
                funcs.insert(func_index, (to_validator, body));
                func_index += 1;
            }
        }

        self.funcs = funcs;
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

        tracing::debug!("dataset: {:?}", dataset);
        Ok(dataset)
    }

    /// Parse import section.
    pub fn imports(reader: &SectionLimited<Import>) -> Imports {
        let mut imports = Imports::default();
        let mut iter = reader.clone().into_iter();
        while let Some(Ok(Import {
            module,
            name,
            ty: TypeRef::Func(index),
        })) = iter.next()
        {
            if let Ok(func) = Func::try_from((module, name)) {
                tracing::debug!("imported function: {}::{} at {index}", module, name);
                imports.push(func);
            }
        }

        imports
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
