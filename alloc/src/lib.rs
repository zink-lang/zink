//! Allocator implementation for the zink language

#![no_std]

extern crate alloc;

mod allocator;
mod num;
mod result;

pub use self::{
    allocator::{Zallocator, ARENA_SIZE},
    num::U256,
    result::{Error, Result},
};
