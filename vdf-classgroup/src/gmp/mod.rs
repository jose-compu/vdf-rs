//! Vendored `rust-gmp` subset used by `vdf-classgroup`.
//!
//! Remaining TODOs in this module (limb width vs `__gmp_bits_per_limb`,
//! invalid-base handling, `set_from_str_radix` return value, zero moduli,
//! perfect-root returns) are deferred with the GMP backend replacement in
//! 0.3.0. This 0.2.x series keeps the existing backend.
#![allow(deprecated)]
#![allow(non_camel_case_types)]

extern crate libc;
extern crate num_traits;

mod ffi;
pub mod mpz;
pub mod sign;

#[cfg(test)]
mod test;
