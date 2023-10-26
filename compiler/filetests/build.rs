//! Load all wat files to structured tests.

use anyhow::{anyhow, Result};
use proc_macro2::Span;
use quote::quote;
use std::{
    env, fs,
    path::{Path, PathBuf},
};
use syn::{parse_quote, ExprArray, ExprMatch, Ident, ItemConst, ItemMod};
use wasm_opt::OptimizationOptions;

/// Read the contents of a directory, returning
/// all wat files.
fn list_wat(dir: impl AsRef<Path>, files: &mut Vec<PathBuf>) -> Result<()> {
    let entry = fs::read_dir(dir)?;
    for entry in entry {
        let entry = entry?;
        let path = entry.path();
        if path.is_dir() {
            list_wat(path, files)?;
        } else if path.extension().unwrap_or_default() == "wat" {
            files.push(path);
        }
    }

    Ok(())
}

/// Batch all wat files.
fn wat_files() -> Result<Vec<PathBuf>> {
    let path = PathBuf::from(env!("CARGO_MANIFEST_DIR")).join("wat");
    let mut files = Vec::new();
    list_wat(&path, &mut files)?;

    let excludes = ["as_if_else.wat"];

    Ok(files
        .into_iter()
        .filter(|f| {
            !excludes.contains(
                &f.file_name()
                    .and_then(|n| n.to_str())
                    .expect("file name not found"),
            )
        })
        .collect())
}

fn examples() -> Result<Vec<PathBuf>> {
    let release = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()?
        .target_directory
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("examples");

    let with_commit_hash = |p: &PathBuf| -> bool {
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        // for example: addition-6313c94b67ad9699.wasm
        let len = name.len();
        if let Some(index) = name.rfind('-') {
            if len > 22 && index == len - 22 {
                return true;
            }
        }

        false
    };

    let files = fs::read_dir(release)?
        .filter_map(|e| {
            let path = e.ok()?.path();
            if path.extension().unwrap_or_default() == "wasm" && !with_commit_hash(&path) {
                Some(path)
            } else {
                None
            }
        })
        .collect::<Vec<_>>();

    for wasm in &files {
        OptimizationOptions::new_opt_level_4()
            .debug_info(false)
            .mvp_features_only()
            .set_converge()
            .run(wasm, wasm)?;
    }

    Ok(files)
}

fn parse_tests() -> Result<(ItemMod, ExprArray, ExprArray)> {
    let mut consts: ItemMod = parse_quote! {
        /// Constant tests.
        mod tests {}
    };
    let mut examples_arr: ExprArray = parse_quote!([]);
    let mut wat_files_arr: ExprArray = parse_quote!([]);
    let mut match_expr: ExprMatch = parse_quote! {
        match (module, name) {}
    };

    let mut push = |tests: &mut ExprArray, p: &PathBuf, bytes: &[u8]| {
        let name = p
            .with_extension("")
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let module = p
            .parent()
            .expect("parent not found for {p:?}")
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string();

        let mut expr: ExprArray = parse_quote!([]);
        for byte in bytes {
            expr.elems.push(parse_quote!(#byte));
        }

        let item: Ident = {
            let ident_name = module.to_uppercase() + "_" + &name.to_ascii_uppercase();
            let ident: Ident = Ident::new(&ident_name.replace('-', "_"), Span::call_site());
            let len = bytes.len();
            let item: ItemConst = parse_quote! {
                #[doc = concat!(" path: ", #module, "::", #name)]
                pub const #ident: [u8; #len] = #expr;
            };

            consts
                .content
                .as_mut()
                .expect("checked above")
                .1
                .push(item.into());

            match_expr.arms.push(parse_quote! {
                (#module, #name) => crate::Test {
                    module: module.into(),
                    name: name.into(),
                    wasm: #ident.to_vec(),
                }
            });
            ident
        };

        tests.elems.push(parse_quote! {
            Test {
                module: #module.into(),
                name: #name.into(),
                wasm: #item.to_vec()
            }
        })
    };

    for wat in wat_files()? {
        let wat_bytes = fs::read(&wat)?;
        let wasm = wat::parse_bytes(&wat_bytes)?;
        push(&mut wat_files_arr, &wat, &wasm);
    }

    for example in examples()? {
        let wasm = fs::read(&example)?;
        push(&mut examples_arr, &example, &wasm);
    }

    match_expr.arms.push(parse_quote! {
        _ => return Err(anyhow::anyhow!("test not found: {{module: {}, name: {}}}", module, name))
    });
    consts
        .content
        .as_mut()
        .expect("checked above")
        .1
        .push(parse_quote! {
            impl crate::Test {
                /// Load test from module and name.
                pub fn load(module: &str, name: &str) -> anyhow::Result<Self> {
                    Ok(#match_expr)
                }
            }
        });

    Ok((consts, examples_arr, wat_files_arr))
}

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wat");

    let tests_rs =
        PathBuf::from(env::var_os("OUT_DIR").ok_or_else(|| anyhow!("OUT_DIR not found"))?)
            .join("tests.rs");
    let (consts, examples, wat_files) = parse_tests()?;

    fs::write(
        tests_rs,
        quote! {
            #consts

            pub use tests::*;

            impl Test {
                /// Example tests.
                pub fn examples() -> Vec<Test> {
                    #examples.to_vec()
                }

                /// Wat files tests.
                pub fn wat_files() -> Vec<Test> {
                    #wat_files.to_vec()
                }

                /// All tests.
                pub fn all() -> Vec<Test> {
                    let mut tests = Self::examples();
                    tests.extend(Self::wat_files());
                    tests
                }
            }
        }
        .to_string(),
    )?;

    Ok(())
}
