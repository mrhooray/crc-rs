use crate::crc8::{finalize, init};
use crate::{Algorithm, Crc, Digest, Implementation};
use core::hash::{BuildHasher, Hasher};

#[cfg(feature = "no-table-mem-limit")]
impl Implementation for u8 {
    type Width = u8;
    type Table = ();
}

#[cfg(all(not(feature = "no-table-mem-limit"), feature = "bytewise-mem-limit"))]
impl Implementation for u8 {
    type Width = u8;
    type Table = [u8; 256];
}

#[cfg(all(
    not(feature = "no-table-mem-limit"),
    not(feature = "bytewise-mem-limit"),
    feature = "slice16-mem-limit"
))]
impl Implementation for u8 {
    type Width = u8;
    type Table = [[u8; 256]; 16];
}

#[cfg(all(
    not(feature = "no-table-mem-limit"),
    not(feature = "bytewise-mem-limit"),
    not(feature = "slice16-mem-limit")
))]
impl Implementation for u8 {
    type Width = u8;
    type Table = [u8; 256];
}

impl Crc<u8> {
    pub const fn new(algorithm: &'static Algorithm<u8>) -> Self {
        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            not(feature = "bytewise-mem-limit"),
            feature = "slice16-mem-limit"
        ))]
        let table =
            crate::table::crc8_table_slice_16(algorithm.width, algorithm.poly, algorithm.refin);

        #[cfg(all(not(feature = "no-table-mem-limit"), feature = "bytewise-mem-limit"))]
        let table = crate::table::crc8_table(algorithm.width, algorithm.poly, algorithm.refin);

        #[cfg(feature = "no-table-mem-limit")]
        #[allow(clippy::let_unit_value)]
        let table = ();

        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            not(feature = "bytewise-mem-limit"),
            not(feature = "slice16-mem-limit")
        ))]
        let table = crate::table::crc8_table(algorithm.width, algorithm.poly, algorithm.refin);

        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u8 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u8, bytes: &[u8]) -> u8 {
        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            not(feature = "bytewise-mem-limit"),
            feature = "slice16-mem-limit"
        ))]
        {
            super::update_slice16(crc, &self.table, bytes)
        }

        #[cfg(all(not(feature = "no-table-mem-limit"), feature = "bytewise-mem-limit"))]
        {
            super::update_bytewise(crc, &self.table, bytes)
        }

        #[cfg(feature = "no-table-mem-limit")]
        {
            super::update_nolookup(crc, self.algorithm, bytes)
        }

        #[cfg(all(
            not(feature = "no-table-mem-limit"),
            not(feature = "bytewise-mem-limit"),
            not(feature = "slice16-mem-limit")
        ))]
        {
            super::update_bytewise(crc, &self.table, bytes)
        }
    }

    pub const fn digest(&self) -> Digest<u8> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u8) -> Digest<u8> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, u8> {
    const fn new(crc: &'a Crc<u8>, value: u8) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u8 {
        finalize(self.crc.algorithm, self.value)
    }
}

impl<'a> Hasher for Digest<'a, u8> {
    fn finish(&self) -> u64 {
        self.clone().finalize() as u64
    }

    fn write(&mut self, bytes: &[u8]) {
        self.update(bytes);
    }
}

impl<'a> BuildHasher for &'a Crc<u8> {
    type Hasher = Digest<'a, u8>;

    fn build_hasher(&self) -> Self::Hasher {
        self.digest()
    }
}
