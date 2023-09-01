//! Zink parser

use crate::Error;
use std::{
    collections::{btree_map::IntoIter, BTreeMap},
    iter::IntoIterator,
};
use wasmparser::{
    FuncToValidate, FunctionBody, Import, Payload, TypeRef, ValidPayload, Validator,
    ValidatorResources,
};
use zingen::{DataSet, Func, Imports};

/// WASM module parser
pub struct Parser<'p> {
    imports: Imports,
    data: DataSet,
    funcs: BTreeMap<u32, (FuncToValidate<ValidatorResources>, FunctionBody<'p>)>,
}

impl<'p> Parser<'p> {
    /// Get parsed imports.
    pub fn imports(&self) -> Imports {
        self.imports.clone()
    }

    /// Get parsed data set.
    pub fn data(&self) -> DataSet {
        self.data.clone()
    }
}

impl<'p> TryFrom<&'p [u8]> for Parser<'p> {
    type Error = Error;

    fn try_from(wasm: &'p [u8]) -> Result<Self, Self::Error> {
        let mut validator = Validator::new();
        let mut func_index = 0u32;
        let mut imports = Imports::default();
        let mut funcs = BTreeMap::new();

        // Compile functions.
        for payload in wasmparser::Parser::new(0).parse_all(wasm) {
            let payload = payload?;
            let valid_payload = validator.payload(&payload)?;

            match &payload {
                // Get imported functions
                //
                // NOTE: this is safe here since the import section is
                // ahead of the function section after the optimization
                // of wasm-opt.
                Payload::ImportSection(reader) => {
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

                    tracing::debug!("imports: {:?}", imports);

                    // if let Payload::DataSection(reader) = &payload {
                    //     let mut iter = reader.clone().into_iter();
                    //     while let Some(Ok(data)) = iter.next() {
                    //         if let DataKind::Active {
                    //             memory_index: _,
                    //             offset_expr: _,
                    //         } = data.kind
                    //         {
                    //             // TODO: parse offset expression.
                    //
                    //             // let buf = &offset_expr.data[1..5];
                    //             // let offset = leb128::read::signed(&mut data)?;
                    //             //
                    //             // dataset.insert(offset, data.data);
                    //         }
                    //         tracing::debug!("data: {:?}", data);
                    //     }
                    //     continue;
                    // }
                }
                _ => {}
            }

            if let ValidPayload::Func(to_validator, body) = valid_payload {
                funcs.insert(func_index, (to_validator, body));
                func_index += 1;
            }
        }

        Ok(Parser {
            imports,
            data: DataSet::default(),
            funcs,
        })
    }
}

impl<'p> IntoIterator for Parser<'p> {
    type Item = (u32, (FuncToValidate<ValidatorResources>, FunctionBody<'p>));
    type IntoIter = IntoIter<u32, (FuncToValidate<ValidatorResources>, FunctionBody<'p>)>;

    fn into_iter(self) -> Self::IntoIter {
        self.funcs.into_iter()
    }
}
