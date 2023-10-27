//! Load all wat files to structured tests.

use anyhow::Result;
use proc_macro2::Span;
use quote::{quote, ToTokens};
use std::{
    collections::BTreeMap,
    env, fs,
    path::{Path, PathBuf},
};
use syn::{parse_quote, ExprArray, ExprMatch, Ident, ItemImpl, ItemMod};
use wasm_opt::OptimizationOptions;

fn main() -> Result<()> {
    println!("cargo:rerun-if-changed=build.rs");
    println!("cargo:rerun-if-changed=wat");

    let tests = Tests::new().parse()?;
    fs::write(
        env::var("OUT_DIR")?.parse::<PathBuf>()?.join("tests.rs"),
        tests.to_token_stream().to_string(),
    )?;

    Ok(())
}

/// Read the contents of a directory, returning
/// all wat files.
fn list_wat(dir: impl AsRef<Path>, files: &mut Vec<PathBuf>) -> Result<()> {
    let entry = fs::read_dir(dir)?;
    for entry in entry {
        let entry = entry?;
        let path = entry.path();

        if path.ends_with("as_if_else.wat") {
            continue;
        }

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
    Ok(files)
}

fn examples() -> Result<Vec<PathBuf>> {
    let release = cargo_metadata::MetadataCommand::new()
        .no_deps()
        .exec()?
        .target_directory
        .join("wasm32-unknown-unknown")
        .join("release")
        .join("examples");

    if !release.exists() {
        return Ok(Default::default());
    }

    let with_commit_hash = |p: &PathBuf| -> bool {
        let name = p
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default();

        // example: addition-6313c94b67ad9699.wasm
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

struct Tests {
    match_expr: ExprMatch,
    item_impl: ItemImpl,
    modules: BTreeMap<String, ItemMod>,
}

impl Tests {
    fn new() -> Self {
        Self {
            match_expr: parse_quote! {
                match (module, name) {}
            },
            item_impl: parse_quote! {
                impl Test {}
            },
            modules: Default::default(),
        }
    }

    fn file_name(p: impl AsRef<Path>) -> String {
        p.as_ref()
            .file_name()
            .unwrap_or_default()
            .to_str()
            .unwrap_or_default()
            .to_string()
    }

    fn get_module(&mut self, mut module: &str) -> &mut ItemMod {
        module = match module {
            "if" => "_if",
            "loop" => "_loop",
            _ => module,
        };

        if !self.modules.contains_key(module) {
            let ident = Ident::new(module, Span::call_site());
            self.modules.insert(
                module.to_string(),
                parse_quote! {
                    #[cfg(test)]
                    mod #ident {
                        use anyhow::Result;
                        use crate::Test;
                    }
                },
            );
        }

        self.modules.get_mut(module).expect("module not found")
    }

    /// Push test to module.
    fn push(&mut self, p: &Path, wasm: &[u8]) -> Result<()> {
        let (module, name) = (
            Self::file_name(p.parent().expect("parent not found")),
            Self::file_name(&p.with_extension("")),
        );

        let ident_name = name.replace('-', "_");
        let ident = Ident::new(
            &(module.clone() + "_" + &ident_name).to_uppercase(),
            Span::call_site(),
        );
        {
            let len = wasm.len();
            let mut expr: ExprArray = parse_quote!([]);
            for byte in wasm {
                expr.elems.push(parse_quote!(#byte));
            }

            self.item_impl.items.push(parse_quote! {
                #[doc = concat!(" path: ", #module, "::", #name)]
                pub const #ident: [u8; #len] = #expr;
            });
        }

        self.match_expr.arms.push(parse_quote! {
            (#module, #name) => Test {
                module: module.into(),
                name: name.into(),
                wasm: Self::#ident.to_vec(),
            }
        });

        let ident_name = Ident::new(&ident_name, Span::call_site());
        self.get_module(&module)
            .content
            .as_mut()
            .expect("")
            .1
            .push(parse_quote! {
                #[test]
                fn #ident_name() -> Result<()> {
                    Test::load(#module, #name)?.compile()
                }
            });

        Ok(())
    }

    fn parse(mut self) -> Result<Self> {
        for wat in wat_files()? {
            let wat_bytes = fs::read(&wat)?;
            let wasm = wat::parse_bytes(&wat_bytes)?;
            self.push(&wat, &wasm)?;
        }

        for example in examples()? {
            let wasm = fs::read(&example)?;
            self.push(&example, &wasm)?;
        }

        self.match_expr.arms.push(parse_quote! {
            _ => return Err(anyhow::anyhow!("test not found: {{module: {}, name: {}}}", module, name))
        });

        let match_expr = self.match_expr.clone();
        let funcs: ItemImpl = parse_quote! {
            impl Test {
                /// Load test from module and name.
                pub fn load(module: &str, name: &str) -> anyhow::Result<Self> {
                    Ok(#match_expr)
                }
            }
        };

        self.item_impl.items.extend(funcs.items);
        Ok(self)
    }
}

impl ToTokens for Tests {
    fn to_tokens(&self, tokens: &mut proc_macro2::TokenStream) {
        let Tests {
            item_impl,
            modules,
            match_expr: _,
        } = self;

        tokens.extend(quote!(#item_impl));
        modules.values().for_each(|m| tokens.extend(quote!(#m)))
    }
}
