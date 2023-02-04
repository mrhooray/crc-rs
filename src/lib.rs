//! # crc
//! Rust implementation of CRC.
//!
//! ## Usage
//! ### Compute CRC16
//! ```rust
//! use crc::{Crc, Algorithm, CRC_16_IBM_SDLC, CRC_32_ISCSI};
//!
//! pub const X25: Crc<u16> = Crc::<u16>::new(&CRC_16_IBM_SDLC);
//! pub const CASTAGNOLI: Crc<u32> = Crc::<u32>::new(&CRC_32_ISCSI);
//!
//! assert_eq!(X25.checksum(b"123456789"), 0x906e);
//! assert_eq!(CASTAGNOLI.checksum(b"123456789"), 0xe3069283);
//!
//! // use custom algorithm
//! const CUSTOM_ALG: Algorithm<u16> = Algorithm {
//!     width: 16,
//!     poly: 0x8005,
//!     init: 0xffff,
//!     refin: false,
//!     refout: false,
//!     xorout: 0x0000,
//!     check: 0xaee7,
//!     residue: 0x0000
//! };
//! let crc = Crc::<u16>::new(&CUSTOM_ALG);
//! let mut digest = crc.digest();
//! digest.update(b"123456789");
//! assert_eq!(digest.finalize(), 0xaee7);
//! ```
//#![no_std]
#![forbid(unsafe_code)]

pub use crc_catalog::*;

mod crc128;
mod crc16;
mod crc32;
mod crc64;
mod crc8;
mod table;
mod util;

/// Implementation using a 16 * 256 entry lookup table. Use it with `Crc<Slice16<W>>`
pub struct Slice16<W: Width>(core::marker::PhantomData<W>);

/// Implementation using a 256 entry lookup table. Use it with `Crc<Bytewise<W>>`
pub struct Bytewise<W: Width>(core::marker::PhantomData<W>);

/// Implementation using no lookup table. Use it with `Crc<Nolookup<W>>`
pub struct NoTable<W: Width>(core::marker::PhantomData<W>);

impl<W: Width> crate::private::Sealed for Slice16<W> {}
impl<W: Width> crate::private::Sealed for Bytewise<W> {}
impl<W: Width> crate::private::Sealed for NoTable<W> {}

impl<W: Width> crate::Implementation for Slice16<W> {
    type Width = W;
    type Table = [[W; 256]; 16];
}

impl<W: Width> crate::Implementation for Bytewise<W> {
    type Width = W;
    type Table = [W; 256];
}

impl<W: Width> crate::Implementation for NoTable<W> {
    type Width = W;
    type Table = ();
}

mod private {
    pub trait Sealed {}
    impl<W: super::Width> Sealed for W {}
}

pub trait Implementation: private::Sealed {
    type Width: Width;
    type Table;
}

impl<W: Width> Implementation for W {
    type Width = W;
    type Table = [W; 256];
}

/// Crc with pluggable implementations ([Nolookup], [Bytewise], [Slice16]).
/// To choose the default implemenation, use the [Width] directly (e.g. `Crc<u32>`).
pub struct Crc<I: Implementation> {
    pub algorithm: &'static Algorithm<I::Width>,
    table: I::Table,
}

#[derive(Clone)]
pub struct Digest<'a, I: Implementation> {
    crc: &'a Crc<I>,
    value: I::Width,
}
