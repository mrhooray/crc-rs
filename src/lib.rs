//! # crc
//! Rust implementation of CRC.
//!
//! ### Examples
//! Using a well-known algorithm:
//! ```rust
//! const X25: crc::Crc<u16> = crc::Crc::<u16>::new(&crc::CRC_16_IBM_SDLC);
//! assert_eq!(X25.checksum(b"123456789"), 0x906e);
//! ```
//!
//! Using a custom algorithm:
//! ```rust
//! const CUSTOM_ALG: crc::Algorithm<u16> = crc::Algorithm {
//!     width: 16,
//!     poly: 0x8005,
//!     init: 0xffff,
//!     refin: false,
//!     refout: false,
//!     xorout: 0x0000,
//!     check: 0xaee7,
//!     residue: 0x0000
//! };
//! let crc = crc::Crc::<u16>::new(&CUSTOM_ALG);
//! let mut digest = crc.digest();
//! digest.update(b"123456789");
//! assert_eq!(digest.finalize(), 0xaee7);
//! ```
#![no_std]
//#![forbid(unsafe_code)]

pub use crc_catalog::algorithm::*;
pub use crc_catalog::{Algorithm, Width};

mod crc128;
mod crc16;
mod crc32;
mod crc64;
mod crc8;
mod table;
mod util;

#[cfg(all(
    target_feature = "sse2",
    target_feature = "sse4.1",
    target_feature = "pclmulqdq"
))]
mod simd;

/// A trait for CRC implementations.
pub trait Implementation: private::Sealed {
    /// Associated data necessary for the implementation (e.g. lookup tables).
    type Data<W>;
}

/// A table-based implementation of the CRC algorithm, with `L` lanes.
/// The number of entries in the lookup table is `L * 256`.
pub struct Table<const L: usize> {}

/// A carry-less multiplication based implementation of the CRC algorithm,
/// which can run in lanes, and only requires 8 coefficients
/// and for fallback reasons a 256-entry lookup table.
#[cfg(any(
    doc,
    all(
        target_feature = "sse2",
        target_feature = "sse4.1",
        target_feature = "pclmulqdq"
    )
))]
pub struct Simd {}

#[cfg(not(any(
    doc,
    all(
        target_feature = "sse2",
        target_feature = "sse4.1",
        target_feature = "pclmulqdq"
    )
)))]
pub type Simd = DefaultImpl;

/// An implementation of the CRC algorithm with no lookup table.
pub type NoTable = Table<0>;

type DefaultImpl = Table<1>;

impl<const L: usize> Implementation for Table<L> {
    type Data<W> = [[W; 256]; L];
}

#[cfg(all(
    target_feature = "sse2",
    target_feature = "sse4.1",
    target_feature = "pclmulqdq"
))]
impl Implementation for Simd {
    type Data<W> = ([W; 256], [simd::Value; 4]);
}

mod private {
    pub trait Sealed {}
    impl<const L: usize> Sealed for super::Table<L> {}

    #[cfg(all(
        target_feature = "sse2",
        target_feature = "sse4.1",
        target_feature = "pclmulqdq"
    ))]
    impl Sealed for super::Simd {}
}

/// Crc instance with a specific width, algorithm, and implementation.
pub struct Crc<W: Width, I: Implementation = DefaultImpl> {
    pub algorithm: &'static Algorithm<W>,
    data: I::Data<W>,
}

#[derive(Clone)]
pub struct Digest<'a, W: Width, I: Implementation = DefaultImpl> {
    crc: &'a Crc<W, I>,
    value: W,
}
