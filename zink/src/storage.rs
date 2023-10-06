/// Storage trait. Currently not for public use
pub trait Storage<T> {
    const STORAGE_KEY: &'static [u8];

    fn get() -> T;
    fn set(value: T);
}
