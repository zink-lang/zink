//! Shared macros

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
