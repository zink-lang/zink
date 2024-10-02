//! Types loader

use crate::ffi;
use core::{marker::PhantomData, ptr};

/// Param for non-primitive types
pub struct Param<T: Load> {
    ptr: *const i32,
    len: usize,
    inner: PhantomData<T>,
}

impl<T: Load> Param<T> {
    /// Load the stored type from pointer
    pub fn load(self) -> T {
        T::load(self.ptr as u32, self.len)
    }
}

impl<T: Load> From<&T> for Param<T> {
    fn from(ty: &T) -> Param<T> {
        Param::<T> {
            ptr: ty as *const T as *const i32,
            len: ty.len(),
            inner: Default::default(),
        }
    }
}

/// Types that can be loaded from pointers
pub trait Load: Sized {
    /// Get the length of this type
    fn len(&self) -> usize;

    /// Load self from calldata
    fn load(ptr: u32, len: usize) -> Self {
        unsafe {
            let output = ffi::load(ptr, len);
            ptr::read(output as *const i32 as *const Self)
        }
    }
}

macro_rules! impl_params {
    ($ty:ty, $len:expr) => {
        impl Load for $ty {
            fn len(&self) -> usize {
                $len
            }
        }
    };
    ($(
        $len:expr
    ),+) => {
        $(impl_params!([u8; $len], $len);)+
    };
}

impl_params!(20, 32);
