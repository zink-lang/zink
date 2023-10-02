//! Zink library for developing smart contracts for blockchains.

#![no_std]

mod event;
pub mod ffi;

pub use self::event::Event;
pub use zalloc::U256;

use core::{cell::UnsafeCell, sync::atomic::AtomicUsize};
use zalloc::{Zallocator, ARENA_SIZE};

// Panic hook implementation
#[cfg(not(test))]
#[panic_handler]
fn panic(_info: &core::panic::PanicInfo) -> ! {
    loop {}
}

#[global_allocator]
static ALLOCATOR: Zallocator = Zallocator {
    arena: UnsafeCell::new([0x55; ARENA_SIZE]),
    remaining: AtomicUsize::new(ARENA_SIZE),
};
