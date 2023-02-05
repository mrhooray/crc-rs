use crate::table::crc128_table;
use crate::{Algorithm, Bytewise, Crc, Digest};

use super::{finalize, init, update_bytewise};

impl Crc<Bytewise<u128>> {
    pub const fn new(algorithm: &'static Algorithm<u128>) -> Self {
        let table = crc128_table(algorithm.width, algorithm.poly, algorithm.refin);
        Self { algorithm, table }
    }

    pub const fn checksum(&self, bytes: &[u8]) -> u128 {
        let mut crc = init(self.algorithm, self.algorithm.init);
        crc = self.update(crc, bytes);
        finalize(self.algorithm, crc)
    }

    const fn update(&self, crc: u128, bytes: &[u8]) -> u128 {
        update_bytewise(crc, self.algorithm.refin, &self.table, bytes)
    }

    pub const fn digest(&self) -> Digest<Bytewise<u128>> {
        self.digest_with_initial(self.algorithm.init)
    }

    /// Construct a `Digest` with a given initial value.
    ///
    /// This overrides the initial value specified by the algorithm.
    /// The effects of the algorithm's properties `refin` and `width`
    /// are applied to the custom initial value.
    pub const fn digest_with_initial(&self, initial: u128) -> Digest<Bytewise<u128>> {
        let value = init(self.algorithm, initial);
        Digest::new(self, value)
    }
}

impl<'a> Digest<'a, Bytewise<u128>> {
    const fn new(crc: &'a Crc<Bytewise<u128>>, value: u128) -> Self {
        Digest { crc, value }
    }

    pub fn update(&mut self, bytes: &[u8]) {
        self.value = self.crc.update(self.value, bytes);
    }

    pub const fn finalize(self) -> u128 {
        finalize(self.crc.algorithm, self.value)
    }
}
