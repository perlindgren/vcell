//! Just like [`Cell`] but with [volatile] read / write operations
//!
//! [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
//! [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html

#![deny(missing_docs)]
#![deny(warnings)]
#![cfg_attr(feature = "const-fn", feature(const_fn))]
#![no_std]

use core::cell::UnsafeCell;
#[cfg(not(feature = "klee-analysis"))]
use core::ptr;

#[cfg(feature = "klee-analysis")]
#[macro_use]
extern crate klee;

/// Just like [`Cell`] but with [volatile] read / write operations
///
/// [`Cell`]: https://doc.rust-lang.org/std/cell/struct.Cell.html
/// [volatile]: https://doc.rust-lang.org/std/ptr/fn.read_volatile.html
pub struct VolatileCell<T> {
    #[allow(unused)]
    value: UnsafeCell<T>,
}

impl<T> VolatileCell<T> {
    /// Creates a new `VolatileCell` containing the given value
    #[cfg(feature = "const-fn")]
    pub const fn new(value: T) -> Self {
        VolatileCell {
            value: UnsafeCell::new(value),
        }
    }

    /// Creates a new `VolatileCell` containing the given value
    ///
    /// NOTE A `const fn` variant is available under the "const-fn" Cargo
    /// feature
    #[cfg(not(feature = "const-fn"))]
    pub fn new(value: T) -> Self {
        VolatileCell {
            value: UnsafeCell::new(value),
        }
    }

    #[cfg(feature = "klee-analysis")]
    /// Returns a fresh symbolic value instance of type T
    #[inline(always)]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        let mut symbolic_value: T = unsafe { core::mem::uninitialized() };

        ksymbol!(&mut symbolic_value, "vcell");
        symbolic_value
    }

    #[cfg(not(feature = "klee-analysis"))]
    /// Returns a copy of the contained value
    #[inline(always)]
    pub fn get(&self) -> T
    where
        T: Copy,
    {
        unsafe { ptr::read_volatile(self.value.get()) }
    }

    #[cfg(feature = "klee-analysis")]
    /// Writing has no side effect
    #[inline(always)]
    pub fn set(&self, _value: T)
    where
        T: Copy,
    {
    }

    #[cfg(not(feature = "klee-analysis"))]
    /// Sets the contained value
    #[inline(always)]
    pub fn set(&self, value: T)
    where
        T: Copy,
    {
        unsafe { ptr::write_volatile(self.value.get(), value) }
    }
}

// NOTE implicit because of `UnsafeCell`
// unsafe impl<T> !Sync for VolatileCell<T> {}
