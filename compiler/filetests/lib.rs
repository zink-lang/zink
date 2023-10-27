#![deny(missing_docs)]
//! Zink filetests

include!(concat!(env!("OUT_DIR"), "/tests.rs"));

/// A wat test
#[derive(Clone)]
pub struct Test {
    /// The module name
    pub module: String,
    /// The test name
    pub name: String,
    /// The test source
    pub wasm: Vec<u8>,
}

#[cfg(test)]
impl Test {
    /// Compile test to evm bytecode.
    pub fn compile(&self) -> anyhow::Result<()> {
        tracing_subscriber::fmt()
            .with_env_filter(tracing_subscriber::EnvFilter::from_default_env())
            .without_time()
            .compact()
            .try_init()
            .ok();

        let Test { module, name, wasm } = self;
        tracing::info!("Compiling {}/{}", module, name);

        zinkc::Compiler::default().compile(&wasm)?;
        Ok(())
    }
}

/// Generate tests for different modules.
#[macro_export]
macro_rules! impl_tests {
    (
        tests: [$($test:ident),+],
        modules: $modules:tt
    ) => {
        $(
            impl_tests!(@test $test $modules);
        )*
    };
    (@test $test:ident [$($mod:expr),*]) => {
        $(
            paste::paste! {
                #[test]
                fn [<$mod _ $test>]() -> anyhow::Result<()> {
                    $test($mod)
                }
            }
        )*
    };
}
